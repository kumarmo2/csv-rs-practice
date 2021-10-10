use std::{
    fs,
    io::{self},
    str::{Lines, Split},
};

pub struct CsvContent {
    _content: String,
}

impl CsvContent {
    pub fn new(path: &str) -> io::Result<CsvContent> {
        let content = fs::read_to_string(path)?;
        Ok(Self { _content: content })
    }

    pub fn rows(&self) -> RowsIterator {
        let lines = self._content.lines();
        RowsIterator::new(lines)
    }
}

pub struct CsvRow<'a> {
    _row: &'a str, // _row might not be needed at all.
    _value_iter: Split<'a, char>,
}

impl<'a> CsvRow<'a> {
    fn new(row: &'a str) -> Self {
        let value_iter = row.split(',');
        Self {
            _row: row,
            _value_iter: value_iter,
        }
    }
}

impl<'a> Iterator for CsvRow<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self._value_iter.next()
    }
}

pub struct RowsIterator<'a> {
    _lines: Lines<'a>,
}

impl<'a> RowsIterator<'a> {
    fn new(lines: Lines<'a>) -> Self {
        Self { _lines: lines }
    }
}

impl<'a> Iterator for RowsIterator<'a> {
    type Item = CsvRow<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self._lines.next();
        if let None = next {
            return None;
        }
        Some(CsvRow::new(next.unwrap()))
    }
}

fn main() -> io::Result<()> {
    let file_name = "./person.csv";

    let csv_content = CsvContent::new(file_name)?;

    for row in csv_content.rows() {
        println!("row starting");
        for value in row {
            print!("{} ", value)
        }
        println!()
    }

    println!("reading was succesfull");
    Ok(())
}
