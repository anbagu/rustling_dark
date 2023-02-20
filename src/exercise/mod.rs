use std::fmt::Display;
use std::fs;
use std::fs::metadata;
use std::io::{prelude::*};
use std::path::Path;
use std::path::PathBuf;
use std::time::SystemTime;

use serde::Deserialize;

use errors::ExerciseError;

use crate::utils;

mod errors;

#[derive(Deserialize, Debug)]
pub struct Exercise {
    pub name: String,
    path: PathBuf,
    mode: Mode,
    hint: String,
    last_mod: Option<SystemTime>,
}

impl Default for Exercise {
    fn default() -> Exercise {
        Exercise {
            name: String::default(),
            path: PathBuf::default(),
            mode: Mode::Test,
            hint: String::default(),
            last_mod: None,
        }
    }
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
    Pending,
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
        let reader = match utils::read_file(&self.path) {
            Ok(value) => value,
            Err(e) => return Err(ExerciseError::IO(e)),
        };


        let lines_iter = &mut reader.lines().into_iter();
        while let Some(line) = lines_iter.next() {
            println!("Line was {:?}", line);
            if line.unwrap().contains("// I AM NOT DONE") {
                return Ok(Status::Pending);
            }
        }
        Ok(Status::Done)
    }

    // pub fn new() {
    //     println!("Helllllllllllllloooow")
    // }

    pub fn has_changed(&mut self) -> bool {
        let current_last_mod = self.get_last_mod();
        let has_changed = current_last_mod == self.last_mod.unwrap();
        println!("{:?} {:?}", current_last_mod, {self.last_mod.unwrap()});
        self.last_mod = Some(current_last_mod);
        has_changed
    }
    fn get_last_mod(&self) -> SystemTime {
        let metadata = metadata::<&PathBuf>(&self.path);
        metadata.unwrap().modified().unwrap()
    }
}