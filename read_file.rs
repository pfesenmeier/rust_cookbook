use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let file_name = args.pop();
    if let Some(name) = file_name {
        let file = File::open(name)?;
        let mut buf_reader = BufReader::new(file);
        let mut contents = String::new();

        buf_reader.read_to_string(&mut contents)?;
        println!("{}", contents);
    }
    Ok(())
}
