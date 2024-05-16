use std::fmt;
use chrono::{NaiveDateTime, TimeDelta};

pub struct Document {
    pub open: Line,
    pub close: Option<Line>,
    pub freeze: Option<Vec<Line>>,
}

impl Document {
    pub fn get_printing_time(&self) -> Option<TimeDelta> {
        let close = match self.close.as_ref() {
            Some(l) => l.dt,
            None => return None,
        };
        return Some(close - self.open.dt);
    }
}

#[derive(Debug, Clone)]
pub struct Line {
    pub index: usize,
    pub line_type: LineType,
    pub dt: NaiveDateTime,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Line {{ index: {}, line_type: {:?}, dt: {} }}", self.index, self.line_type, self.dt)
    }
}

impl fmt::Display for LineType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LineType::Open => write!(f, "Open"),
            LineType::Close(b) => {
                if *b {
                    write!(f, "Close (success)")
                } else {
                    write!(f, "Close (failed)")
                }
            }
            LineType::Freeze => write!(f, "Freeze"),
        }
    }
}

impl fmt::Display for Document {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Document {{ open: {}, close: {}, freeze: {:?} }}",
               self.open,
               self.close.as_ref().map_or("None".to_string(), |line| format!("{}", line)),
               self.freeze.as_ref().map_or("None".to_string(), |lines| format!("{:?}", lines))
        )
    }
}

#[derive(Debug, Clone)]
pub enum LineType {
    Open,
    Close(bool),
    Freeze,
}
