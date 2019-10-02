extern crate parse_drivelog as lib;

use std::error::Error;
use std::fs;
use std::io::BufReader;
//use std::io;
use std::process;

use lib::record::Record;
use lib::AverageBuilder;

fn run() -> Result<(), Box<dyn Error>> {
    let finn = BufReader::new(fs::File::open("log.csv")?);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .double_quote(false)
        .from_reader(finn);

    let mut data: Vec<Record> = vec![];
    for i in rdr.deserialize() {
        data.push(Record::from_raw(i?));
    }

    let mut ave = AverageBuilder::new(data.len());
    ave.setup(data);
    let ave = ave.build();

    // println!("speed: {}", ave.speed);
    // println!("displacement.x1: {}", ave.displacement.x1);
    println!("{:?}", ave);

    Ok(())
}

fn main() {
    println!("Hello, world!");

    if let Err(err) = run() {
        println!("error running: {}", err);
        process::exit(1);
    }
}
