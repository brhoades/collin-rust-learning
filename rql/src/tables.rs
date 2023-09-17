use ratatui::widgets::ListState;

/// Represents a list of tables.
pub struct DbTables {
    pub names: Vec<String>,
    pub state: ListState,
}

impl DbTables {
    pub fn new(names: Vec<String>) -> Self {
        let mut state = ListState::default();
        if !names.is_empty() {
            state.select(Some(0));
        }
        Self { names, state }
    }

    pub fn max_len(&self) -> u16 {
        self.names.iter().map(|s| s.len() as u16).max().unwrap_or(0)
    }

    pub fn next(&mut self) {
        let i = self
            .state
            .selected()
            .map(|i| if i >= self.names.len() - 1 { 0 } else { i + 1 })
            .unwrap_or(0);
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = self
            .state
            .selected()
            .map(|i| if i == 0 { self.names.len() - 1 } else { i - 1 })
            .unwrap_or(0);
        self.state.select(Some(i));
    }

    pub fn selected(&self) -> Option<String> {
        self.state
            .selected()
            .map(|i| self.names.get(i).map(|s| s.clone()))
            .flatten()
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}
