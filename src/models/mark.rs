#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mark {
    X,
    O,
}

impl Mark {
    pub fn oppiste(&self) -> Self {
        match self {
            Mark::O => Mark::X,
            Mark::X => Mark::O,
        }
    }
}
