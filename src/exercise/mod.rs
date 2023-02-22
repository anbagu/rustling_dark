use std::fmt::Display;
use std::fs;
use std::fs::metadata;
use std::io::{prelude::*};
use std::path::Path;
use std::path::PathBuf;
use std::process::{Command, ExitStatus, Output};
use std::time::UNIX_EPOCH;

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
}

impl Exercise {
    pub(crate) fn compile(&self) -> std::io::Result<ExitStatus> {
        Command::new("rustc").args([&self.path]).status()
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

    // pub fn get_last_mtime(&mut self) -> bool {
    //     let current_last_mod = self.get_last_mod();
    //     let has_changed = current_last_mod != self.last_mod.get_mut().or_else(|| Some(SystemTime::now())).unwrap();
    //     println!("{:?} {:?}", current_last_mod, { self.last_mod.get_mut() });
    //     self.last_mod = Some(current_last_mod).into();
    //     has_changed
    // }
    pub fn get_last_mod(&self) -> u64 {
        let metadata = metadata::<&PathBuf>(&self.path);
        let last_mod_st = metadata.unwrap().modified().unwrap();
        last_mod_st.duration_since(UNIX_EPOCH).unwrap().as_secs()
    }
}