use arrow::csv;
use arrow::datatypes::{DataType, Field, Schema};
use std::fs::File;
use std::sync::Arc;

fn main() {
    let schema = Schema::new(vec![
        Field::new("name", DataType::Utf8, false),
        Field::new("age", DataType::UInt8, false),
    ]);

    let file = File::open("resources/test.csv").unwrap();

    let mut csv = csv::Reader::new(file, Arc::new(schema), false, None, 1024, None, None, None);
    let batch = csv.next().unwrap().unwrap();

    println!("batch={:?}", batch);
}
