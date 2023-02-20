use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::PathBuf;

mod fs;

pub fn read_file(path: &PathBuf) -> Result<BufReader<File>, io::Error, > {
    let file = match File::open(&path) {
        Err(e)=> return Err(e),
        Ok(res) => res,
    };
    let reader = BufReader::new(file);
    Ok(reader)
}
