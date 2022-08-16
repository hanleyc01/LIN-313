#![allow(non_snake_case, dead_code, unused)]

use std::{fs::File, io::{Error, Read}};

/// CSV data set, entries delimited by `:` (as `,`'s are present in data)
const DATA: &str = "csv-data.csv";

fn main() -> Result<(), Error> {
    // open file
    let mut f = File::open(DATA)?;
    
    // read to string buffer
    let mut raw_data = String::new();
    f.read_to_string(&mut raw_data);
    
    // push raw_data into vector
    let mut data: Vec<&str> = Vec::new();
    raw_data.split(':').for_each(|s| data.push(s));

    Ok(())
}
