
#[derive(Debug, PartialEq)]
pub struct Error {
    pub msg: String,
    pub line: u32,
    pub col: u32,
}

