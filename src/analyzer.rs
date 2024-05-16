use chrono::{Duration, TimeDelta};
use crate::models::{Document, LineType};

pub fn print_stats(data: Vec<Document>) {
    println!();
    println!("-----Средняя статистика-----");
    println!("Найдено документов: {}", data.len());
    let bad_connection = get_bad_connection(&data);
    println!("Не удалось подключиться: {bad_connection}");
    let (average, max, threshold, with_freezes) = get_printing_times(&data);
    println!("Среднее время печати: {}", average.num_seconds());
    println!("Печать дольше 10 секунд: {}", threshold);
    println!("Имеются обрывы связи: {}", with_freezes);
    if let Some(doc) = max {
        println!();
        println!("-----Самый долгий документ-----");
        println!("Максимальное время печати: {}", doc.get_printing_time().unwrap().num_seconds());
        println!("Линия начала: {}", doc.open.index);
        let open_dt = doc.open.dt.format("%H:%M:%S").to_string();
        let close_dt = doc.close.as_ref().unwrap().dt.format("%H:%M:%S").to_string();
        println!("Время печати: {} – {}", open_dt, close_dt);
        match doc.freeze.as_ref() {
            Some(f) => println!("Обрывов связи: {}", f.len()),
            None => println!("Обрывов связи в документе не было"),
        }
    }
}

fn get_bad_connection(data: &Vec<Document>) -> usize {
    data.iter().filter(|d| {
        if let Some(ref close) = d.close {
            return match close.line_type {
                LineType::Close(success) => !success,
                _ => false,
            };
        }
        false
    }).count()
}

fn get_printing_times(data: &Vec<Document>) -> (TimeDelta, Option<&Document>, i32, i32) {
    let threshold = Duration::new(10, 0).unwrap();
    let mut threshold_count = 0;
    let mut with_freezes = 0;
    let mut total = Duration::new(0, 0).unwrap();
    let mut max_delta = Duration::new(0, 0).unwrap();
    let mut max_doc: Option<&Document> = None;
    let mut count = 0;
    for doc in data {
        let doc_time = doc.get_printing_time();
        if let Some(doc_time) = doc_time {
            if doc_time > max_delta {
                max_delta = doc_time;
                max_doc = Some(doc);
            }
            if doc_time > threshold {
                threshold_count += 1;
            }
            total += doc_time;
            count += 1;
        }
        if let Some(freezes) = doc.freeze.as_ref() {
            if freezes.len() > 0 {
                with_freezes += 1;
            }
        }
    }
    return (total / count, max_doc, threshold_count, with_freezes);
}