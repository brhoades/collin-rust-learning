use anyhow::{Context, Result};
use sqlx::{sqlite::SqliteRow, Column, Pool, Row, Sqlite, SqlitePool, TypeInfo};
use std::{ops::Deref, sync::Arc, time::Instant};
use tokio::runtime::Runtime;

#[derive(Clone)]
pub struct BlockingDao {
    inner: Arc<BlockingInner>,
}

struct BlockingInner {
    dao: Dao,
    rt: Runtime,
}

impl BlockingDao {
    pub fn new(db: DB) -> Result<Self> {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()?;
        let dao = rt.block_on(Dao::new(db))?;
        let inner = BlockingInner { dao, rt };
        let inner = inner.into();
        Ok(Self { inner })
    }

    pub fn tables(&self) -> Result<Vec<String>> {
        self.inner.rt.block_on(self.inner.dao.tables())
    }

    pub fn table_schema<P: AsRef<str>>(&self, table_name: P) -> Result<TableSchema> {
        self.inner
            .rt
            .block_on(self.inner.dao.table_schema(table_name))
    }
}

#[derive(Clone)]
struct Dao {
    pool: Pool<Sqlite>,
}

pub struct TableSchema {
    pub cols: Vec<TableColumn>,
}

#[derive(sqlx::FromRow)]
pub struct TableColumn {
    cid: u32,
    name: String,
    #[sqlx(rename = "type")]
    typ: String,
    notnull: bool,
    dflt_value: String,
    pk: bool,
}

/// A row in the table
#[derive(Default)]
pub struct Record {
    fields: Vec<Field>,
}

pub struct Field {
    name: String,
    typ: FieldType,
    val: FieldValue,
}

#[derive(Debug, PartialEq)]
pub enum FieldValue {
    Null,
    Text(String),
    Real(f64),
    Blob(Vec<u8>),
    Integer(i64),
    Numeric(f64),
    Boolean(bool),
    Date(Instant),
    Time(Instant),
    DateTime(Instant),
}

#[derive(Debug)]
pub enum FieldType {
    Null,
    Text,
    Real,
    Blob,
    Integer,
    Numeric,
    Boolean,
    Date,
    Time,
    DateTime,
}

impl FieldType {
    fn decode(&self, row: &SqliteRow, idx: usize) -> Result<FieldValue> {
        let val = match self {
            FieldType::Null => FieldValue::Null,
            FieldType::Text => FieldValue::Text(self.decode_string(row, idx)?),
            FieldType::Real => FieldValue::Real(self.decode_f64(row, idx)?),
            FieldType::Blob => FieldValue::Blob(self.decode_bytes(row, idx)?),
            FieldType::Integer => FieldValue::Integer(self.decode_i64(row, idx)?),
            FieldType::Numeric => FieldValue::Numeric(self.decode_f64(row, idx)?),
            FieldType::Boolean => FieldValue::Boolean(self.decode_bool(row, idx)?),
            FieldType::Date => FieldValue::Date(self.decode_instant(row, idx)?),
            FieldType::Time => FieldValue::Time(self.decode_instant(row, idx)?),
            FieldType::DateTime => FieldValue::DateTime(self.decode_instant(row, idx)?),
        };
        Ok(val)
    }

    fn decode_instant(&self, row: &SqliteRow, idx: usize) -> Result<Instant> {
        todo!()
    }

    fn decode_bool(&self, row: &SqliteRow, idx: usize) -> Result<bool> {
        Ok(row.try_get::<bool, _>(idx)?)
    }

    fn decode_i64(&self, row: &SqliteRow, idx: usize) -> Result<i64> {
        Ok(row.try_get::<i64, _>(idx)?)
    }

    fn decode_bytes(&self, row: &SqliteRow, idx: usize) -> Result<Vec<u8>> {
        Ok(row.try_get::<Vec<u8>, _>(idx)?)
    }

    fn decode_f64(&self, row: &SqliteRow, idx: usize) -> Result<f64> {
        Ok(row.try_get::<f64, _>(idx)?)
    }

    fn decode_string(&self, row: &SqliteRow, idx: usize) -> Result<String> {
        Ok(row.try_get::<String, _>(idx)?)
    }
}

impl From<&str> for FieldType {
    fn from(value: &str) -> Self {
        match value.to_lowercase().as_ref() {
            "string" => FieldType::Text,
            "integer" => FieldType::Integer,

            "NULL" => FieldType::Null,
            "TEXT" => FieldType::Text,
            "REAL" => FieldType::Real,
            "BLOB" => FieldType::Blob,
            "INTEGER" => FieldType::Integer,
            "NUMERIC" => FieldType::Numeric,
            "BOOLEAN" => FieldType::Boolean,
            "DATE" => FieldType::Date,
            "TIME" => FieldType::Time,
            "DATETIME" => FieldType::DateTime,
            _ => panic!("unknown type: {}", value),
        }
    }
}

#[derive(Default)]
pub struct Records {
    records: Vec<Record>,
}

impl Deref for Records {
    type Target = Vec<Record>;
    fn deref(&self) -> &Self::Target {
        &self.records
    }
}

pub enum DB<'a> {
    Path(&'a str),
    Memory,
}

impl<'a> DB<'a> {
    async fn connect(&'a self) -> Result<Pool<Sqlite>> {
        match self {
            Self::Path(path) => {
                let path = path.as_ref();
                SqlitePool::connect(path)
                    .await
                    .context(format!(r#"could not open "{path}""#))
            }
            Self::Memory => SqlitePool::connect(":memory:")
                .await
                .context("could not connect to memory db"),
        }
    }
}

impl Dao {
    pub async fn new(db: DB<'_>) -> Result<Self> {
        let pool = db.connect().await?;
        Ok(Self { pool })
    }

    async fn tables(&self) -> Result<Vec<String>> {
        #[derive(sqlx::FromRow)]
        struct Record {
            name: String,
        }
        let mut conn = self.pool.acquire().await?;
        let res = sqlx::query_as::<_, Record>(
            "select name from sqlite_schema where type='table' order by name",
        )
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .map(|s| s.name)
        .collect();
        Ok(res)
    }

    async fn table_schema<P: AsRef<str>>(&self, table_name: P) -> Result<TableSchema> {
        let mut conn = self.pool.acquire().await?;
        let query = format!("pragma table_info({})", table_name.as_ref());
        let cols = sqlx::query_as::<_, TableColumn>(&query)
            .fetch_all(&mut *conn)
            .await?;
        let schema = TableSchema { cols };
        Ok(schema)
    }

    async fn records<P: AsRef<str>>(&self, table_name: P, schema: &TableSchema) -> Result<Records> {
        let mut conn = self.pool.acquire().await?;
        let query = format!("select * from {}", table_name.as_ref());
        let rows = sqlx::query(&query).fetch_all(&mut *conn).await?;
        let mut records = Records::default();
        for row in rows {
            let mut record = Record::default();
            for column in row.columns() {
                let name = column.name().to_string();
                let ord = column.ordinal();
                let col = &schema.cols[ord];
                let type_info = column.type_info();
                println!(
                    "ord: {ord} type_info: {type_info} name: {name} coltyp: {}",
                    col.typ
                );
                let typ = FieldType::from(col.typ.as_ref());
                let val = typ.decode(&row, ord)?;
                let field = Field { name, typ, val };
                record.fields.push(field);
            }
            records.records.push(record);
        }
        Ok(records)
    }

    async fn execute(&self, sql: &'static str) -> Result<()> {
        let mut conn = self.pool.acquire().await?;
        sqlx::query(sql).execute(&mut *conn).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_decode() -> Result<()> {
        let dao = Dao::new(DB::Memory).await?;
        dao.execute("create table foo (name string, age integer)")
            .await?;
        let schema = dao.table_schema("foo").await?;
        let records = dao.records("foo", &schema).await?;
        assert_eq!(records.len(), 0);
        dao.execute("insert into foo (name, age) values ('collin', 46)")
            .await?;
        let records = dao.records("foo", &schema).await?;
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].fields.len(), 2);
        assert_eq!(
            records[0].fields[0].val,
            FieldValue::Text("collin".to_string())
        );
        assert_eq!(records[0].fields[1].val, FieldValue::Integer(46),);
        Ok(())
    }
}
