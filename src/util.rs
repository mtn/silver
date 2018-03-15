use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Error {
    pub msg: String,
    pub line: u32,
    pub col: u32,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error: {}\nLine: {}, Column: {}\n",
               self.msg,
               self.line,
               self.col)
    }
}

