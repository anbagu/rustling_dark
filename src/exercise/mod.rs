use std::fs;
use std::path::Path;

use std::fs::{File};
use std::io::{self, prelude::*, BufReader, Error as IOError};

use serde::Deserialize;
use std::path::PathBuf;
#[derive(Deserialize, Debug)]

pub struct Exercise {
    name: String,
    path: PathBuf,
    mode: Mode,
    hint: String,
}

#[derive(Deserialize)]
pub struct ExerciseList {
    pub exercises: Vec<Exercise>,
}

#[derive(Deserialize, Copy, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Compile,
    Test,
    Clippy,
}

pub enum Status {
    Done,
    Pending
}

#[derive(Debug)]
pub enum ExerciseError {
    IO(IOError),
}

pub fn get_exercises() -> Vec<Exercise> {
    check_toml_info_exists();

    let toml_str = &fs::read_to_string("info.toml").unwrap();
    let exercises = toml::from_str::<ExerciseList>(toml_str).unwrap().exercises;
    exercises
}

fn check_toml_info_exists() {
    if !Path::new("info.toml").exists() {
        println!(
            "{} must be run from the rustlings directory",
            std::env::current_exe().unwrap().to_str().unwrap()
        );
        println!("Try `cd dark_rustling/`!");
        std::process::exit(1);
    }
}




impl Exercise {
    pub fn check_status(&self) -> Result<Status, ExerciseError> {
        println!("Checking status for {:?}", &self.path);
        // "Exercise {} not found in path: {:?}", self.name, self.path)
        let file =  match File::open(&self.path) {
            Err(err) => return Err(ExerciseError::IO((err))),
            Ok(res) => res
        };

        let reader = BufReader::new(file);
        

        let lines_iter = &mut reader.lines().into_iter();
        while let Some(line) = lines_iter.next() {
            println!("Line was {:?}", line);
            if line.unwrap().contains("// I AM NOT DONE") {
                return Ok(Status::Pending);
            }
        }
        Ok(Status::Done)
    }
}
