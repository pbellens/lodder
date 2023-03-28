use tui::widgets::{ListState, TableState};

pub trait GrowingState {
    fn selected(&self) -> Option<usize>;
    fn select(&mut self, index: Option<usize>);
}

impl GrowingState for ListState {
    fn selected(&self) -> Option<usize> {
        self.selected()
    }

    fn select(&mut self, index: Option<usize>) {
        self.select(index)
    }
}

impl GrowingState for TableState {
    fn selected(&self) -> Option<usize> {
        self.selected()
    }

    fn select(&mut self, index: Option<usize>) {
        self.select(index)
    }
}

pub struct GrowingListState<T: GrowingState> {
    pub state: T,
    pub length: Option<usize>,
}

impl<T: GrowingState + std::default::Default> GrowingListState<T> {
    pub fn default() -> Self {
        GrowingListState {
            state: T::default(),
            length: None,
        }
    }

    pub fn next(&mut self, jump: usize) {
        let target = match self.state.selected() {
            Some(i) => {
                let l = self.length.unwrap();
                if i + jump >= l {
                    l - 1
                } else {
                    i + jump
                }
            }
            None => 0,
        };
        self.state.select(Some(target));
    }

    pub fn prev(&mut self, jump: usize) {
        let target = match self.state.selected() {
            Some(i) if i == 0 => i,
            Some(i) => i.saturating_sub(jump),
            None => 0,
        };
        self.state.select(Some(target));
    }
}
