use std::collections::HashMap;

use crate::prelude::*;

/// Enables the display of a table's contents
pub struct DbTable {
    dao: BlockingDao,
    pub schema: TableSchema,
    records: Records,
    pub state: TableState,
    pub count: u64,
    max_lens: HashMap<TableColumn, usize>,
}

impl DbTable {
    pub fn new(dao: BlockingDao, name: String) -> Result<Self> {
        info!(name, "Building db table");
        let count = dao.count(&name)?;
        let schema = dao.table_schema(&name)?;
        let records = Records::default();
        let state = TableState::default();
        let max_lens = HashMap::default();
        let mut table = Self {
            dao,
            schema,
            records,
            state,
            count,
            max_lens,
        };
        table.fetch()?;
        Ok(table)
    }

    pub fn max_len(&self, col: &TableColumn, dfvalue: usize) -> usize {
        *self.max_lens.get(col).unwrap_or(&dfvalue)
    }

    fn fetch(&mut self) -> Result<()> {
        self.records = self
            .dao
            .records(&self.schema, GetRecords::new(&self.schema.name))?;
        let cols = &self.schema.cols;
        for record in self.records.iter() {
            debug!(?record.fields, "Record");
            for (field_idx, field) in record.fields.iter().enumerate() {
                let col = &cols[field_idx];
                let val = &field.val;
                let len = val.len();
                let insert = match self.max_lens.get(col) {
                    Some(l) if &len < l => false,
                    _ => true,
                };
                if insert {
                    self.max_lens.insert(col.clone(), len);
                }
            }
        }
        Ok(())
    }

    pub fn records<'a>(&'a self) -> &[Record] {
        info!(?self.state, "Records");
        return &self.records;
    }

    pub fn name<'a>(&'a self) -> &'a str {
        return &self.schema.name;
    }

    pub fn next(&mut self) {
        self.incr(1);
    }

    pub fn previous(&mut self) {
        self.incr(-1);
    }

    pub fn select_first(&mut self) {
        self.state.select(if self.records.is_empty() {
            None
        } else {
            Some(0)
        });
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }

    fn incr(&mut self, amt: i64) {
        if self.records.is_empty() {
            self.state.select(Some(0));
            return;
        }
        if self.state.selected().is_none() {
            self.state.select(Some(0));
            return;
        }
        let selected = self.state.selected().unwrap_or_default();
        let selected: i64 = selected.try_into().unwrap();
        let selected = selected + amt;
        let selected = if selected < 0 {
            self.records.len() - 1
        } else if selected >= self.records.len().try_into().unwrap() {
            0
        } else {
            selected as usize
        };
        self.state.select(Some(selected));
    }
}
