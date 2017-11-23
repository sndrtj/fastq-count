extern crate bio;
extern crate flate2;

#[macro_use]
extern crate serde_json;

use std::env;
use std::io::BufReader;
use bio::io::fastq;
use flate2::bufread;
use std::fs;

fn get_fastq_reader(path: &String) -> Box<::std::io::Read> {
    if path.ends_with(".gz") {
        let f = fs::File::open(path).unwrap();
        Box::new(bufread::GzDecoder::new(BufReader::new(f)).unwrap())
    } else {
        Box::new(fs::File::open(path).unwrap())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let r1 = &args[1];
    let r2 = &args[2];

    let mut basecount = 0;
    let mut readcount = 0;

    let r1_reader = fastq::Reader::new(get_fastq_reader(r1));
    let r2_reader = fastq::Reader::new(get_fastq_reader(r2));

    for r1_record in r1_reader.records(){
        let len = r1_record.unwrap().seq().len();
        basecount += len;
        readcount += 1;
    }
    for r2_record in r2_reader.records(){
        let len = r2_record.unwrap().seq().len();
        basecount += len;
        readcount += 1;
    }

    let j = json!({
       "reads": readcount,
       "bases": basecount
    });

    println!("{}",j.to_string());

}
