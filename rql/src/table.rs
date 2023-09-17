use anyhow::Result;

use crate::dao::{BlockingDao, Records, TableSchema};

/// Enables the display of a table's contents
pub struct Table {
    dao: BlockingDao,
    name: String,
    schema: TableSchema,
    records: Records,
}

impl Table {
    pub fn new(dao: BlockingDao, name: String) -> Result<Self> {
        let schema = dao.table_schema(&name)?;
        let records = dao.records(&name, &schema)?;
        let table = Self {
            dao,
            name,
            schema,
            records,
        };
        Ok(table)
    }
}
