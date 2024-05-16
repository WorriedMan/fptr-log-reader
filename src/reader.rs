use std::fs::File;
use std::io::{BufReader, Error, ErrorKind};

use chrono::NaiveDateTime;
use regex::{Captures, Regex};

use crate::models::{Document, Line, LineType};

lazy_static! {
    static ref REGEX_OPEN: Regex = {
        let p = r"^(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}\.\d{3})\s+T:[0-9A-F]+\s+INFO\s+\[FiscalPrinter\]\s+libfptr_open\(\)\s+\[0x[0-9A-F]+\]$";
        Regex::new(p).unwrap()
    };
    static ref REGEX_CLOSE: Regex = {
        let p = r"^(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}\.\d{3})\s+T:[0-9A-F]+\s+INFO\s+\[FiscalPrinter\]\s+libfptr_close\(\)\s+\[0x[0-9A-F]+\]$";
        Regex::new(p).unwrap()
    };
    static ref REGEX_NO_CONNECTION: Regex = {
        let p = r"^(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}\.\d{3})\s+T:([0-9A-F]+)\s+(ERROR)\s+\[FiscalPrinter\]\s+Объекту\s+(0x[0-9A-F]+)\s+присвоен код ошибки 2\s+\[Нет связи\]$";
        Regex::new(p).unwrap()
    };
    static ref REGEX_LOST: Regex = {
        let p = r"^(\d{4}\.\d{2}\.\d{2} \d{2}:\d{2}:\d{2}\.\d{3})\s+T:([0-9A-F]+)\s+(WARN)\s+\[Transport\]\s+Возможен обрыв связи\. Переподключаемся и проверяем результат задачи\.\.\.$";
        Regex::new(p).unwrap()
    };
}

pub fn open_file(path: &str) -> Result<BufReader<File>, Error> {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(e) => {
            let msg = format!("Не удалось открыть файл {}: {}", path, e);
            return Err(Error::new(ErrorKind::Other, msg));
        }
    };
    return Ok(BufReader::new(file));
}

pub fn parse_file<I>(lines: I) -> Result<Vec<Document>, Error>
    where I: Iterator<Item=Result<String, Error>> {
    let mut parsed: Vec<Document> = Vec::new();
    let mut doc: Option<Document> = None;
    for (index, line) in lines.enumerate() {
        let file_line = match line {
            Ok(l) => l,
            Err(e) => {
                let msg = format!("Не удалось прочитать строку {}: {}", index + 1, e);
                return Err(Error::new(ErrorKind::Other, msg));
            }
        };
        let line = match parse_line(index + 1, &file_line) {
            Some(l) => l,
            None => continue,
        };
        match line.line_type {
            LineType::Open => {
                if let Some(doc) = doc {
                    println!("[{}] Найден не закрытый документ: {}", index, doc);
                    parsed.push(doc);
                }
                doc = Some(Document { open: line, close: None, freeze: None })
            }
            LineType::Close(_) => {
                if let Some(mut existing_doc) = doc.take() {
                    existing_doc.close = Some(line);
                    parsed.push(existing_doc);
                } else {
                    println!("[{}] Не найден открытый документ при закрытии", line.index);
                }
            }
            LineType::Freeze => {
                match &mut doc {
                    None => println!("[{}] Не найден открытый документ при обрыве связи", line.index),
                    Some(doc) => {
                        if doc.freeze.is_none() {
                            doc.freeze = Some(Vec::new());
                        }
                        doc.freeze.as_mut().unwrap().push(line);
                    }
                }
            }
        }
    }
    return Ok(parsed);
}


fn parse_line(index: usize, line: &str) -> Option<Line> {
    let captures: Captures;
    let line_type: LineType;

    if let Some(c) = REGEX_OPEN.captures(line) {
        captures = c;
        line_type = LineType::Open;
    } else if let Some(c) = REGEX_CLOSE.captures(line) {
        captures = c;
        line_type = LineType::Close(true);
    } else if let Some(c) = REGEX_NO_CONNECTION.captures(line) {
        captures = c;
        line_type = LineType::Close(false);
    } else if let Some(c) = REGEX_LOST.captures(line) {
        captures = c;
        line_type = LineType::Freeze;
    } else {
        return None;
    }
    let date_time = captures.get(1).unwrap().as_str();
    let dt = get_line_dt(date_time);
    return Some(Line { index, line_type, dt });
}

fn get_line_dt(line: &str) -> NaiveDateTime {
    return NaiveDateTime::parse_from_str(line, "%Y.%m.%d %H:%M:%S%.3f").unwrap();
}