extern crate parse_drivelog as lib;

use std::error::Error;
use std::fs;
use std::io::BufReader;
//use std::io;
use std::process;

use lib::Record;

fn run() -> Result<(), Box<dyn Error>> {
    let finn = BufReader::new(fs::File::open("log.csv")?);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b',')
        .double_quote(false)
        .from_reader(finn);

    let mut data: Vec<Record> = vec![];
    for i in rdr.deserialize() {
        data.push(Record::new(i?));
    }
    let speed_ave = data.iter().map(|x| x.speed).fold(0., |sum, x| sum + x) / data.len() as f64;
    // let current_motor_ave = data
    //     .iter()
    //     .map(|x| x.current_motor.ave)
    //     .fold(0., |sum, x| sum + x)
    //     / data.len() as f64;

    println!("speed: {}", speed_ave);

    Ok(())
}

fn main() {
    println!("Hello, world!");

    if let Err(err) = run() {
        println!("error running: {}", err);
        process::exit(1);
    }
}
