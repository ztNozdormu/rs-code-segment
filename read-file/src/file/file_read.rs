use std::fs::File;
use std::io::Read;
use std::num;
use std::io;
use anyhow::{Context,Result};
use thiserror::Error;



#[derive(Error,Debug)]
pub enum CliError {
  #[error("{0}")]  
  IO(#[from] io::Error),
  #[error("{0}")]
  PARSE(#[from] num::ParseIntError),
}

pub struct FileIno <'a>{
    path: &'a str 
}

impl<'a> FileIno<'a> {

    pub fn new(path: &'a str) -> FileIno {
        FileIno { path }
    }
    pub fn read_file(&self) -> Result<u32,anyhow::Error> {
      let mut file = File::open(self.path).context(format!("file path open {}",self.path))?;

      let mut content = String::new();
      let mut sum = 0;
      file.read_to_string(&mut content)?;
      for line in content.lines() {
        sum += line.parse::<u32>()?;
      }
       Ok(sum)
    }

}