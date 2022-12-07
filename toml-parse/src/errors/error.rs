use std::fmt;
use std::path::PathBuf;
use std::error::Error;
use self::ConfigError::*;

#[derive(Debug)]
pub enum ConfigError {
    // The configuration file was not found.
    NotFound,
    // There was an I/O error while reading the configuration file.
    IoError,
    // The path at which the configuration file was found was invalid.
    BadFilePath(PathBuf,& 'static str),
    // An environment specified in `POEM_ENV` is invalid.
    BadEnv(String),
    // An environment specified as a table `[environment]` is invalid.
    BadEntry(String,PathBuf),
    // A config key was specified with a value of the wrong type.
    BadType(String,&'static str,&'static str,Option<PathBuf>),
    // There was a TOML parsing error.
    ParseError(String,PathBuf,String,Option<(usize,usize)>)

}

impl Error for ConfigError {

    fn description(&self) -> &str {
        match *self {
            NotFound => "The configuration file was not found.",
            IoError => "There was an I/O error while reading the configuration file.",
            BadFilePath(..) => "The path at which the configuration file was found was invalid.",
            BadEnv(..) => "An environment specified in `POEM_ENV` is invalid.",
            BadEntry(..) => "An environment specified as a table `[environment]` is invalid.",
            BadType(..) => "A config key was specified with a value of the wrong type.",
            ParseError(..) => "There was a TOML parsing error.",
        }
    }
}

impl fmt::Display for ConfigError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            NotFound => write!(f,"The configuration file was not found."),
            IoError => write!(f,"There was an I/O error while reading the configuration file."),
            BadFilePath(ref p,_) => write!(f,"The path {:?} at which the configuration file was found was invalid.",p),
            BadEnv(ref s) => write!(f,"{:?} environment specified in `POEM_ENV` is invalid.",s),
            BadEntry(ref s,_) => write!(f,"{:?} environment specified as a table `[environment]` is invalid.",s),
            BadType(ref s,s1,s2,_) => {
                write!(f, "type mismatch for '{}'. expected {}, found {}", s, s1, s2)
            }
            ParseError(..) => write!(f,"the config file contains invalid TOML"),
        }
    }
}