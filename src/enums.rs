pub enum Container {
    ABOX,
    LINE,
    COLUMN,
}

#[derive(PartialEq)]
pub enum SetKind {
    NORMAL,
    GUESS,
}

pub enum Progress {
    Solved(String),
    InProgress(i32),
    LimitReached(String),
}
