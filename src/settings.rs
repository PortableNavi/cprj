use std::path::Path;

use serde::Deserialize;
use toml_parse_from_file::FromFile;
use crate::prelude::*;


#[derive(Deserialize, FromFile)]
pub struct ProjectSettings
{
    pub version: String,
    pub name: String,
    pub run_command: String,
    pub build_command: String,
    pub build_files: Vec<String>,
}


impl ProjectSettings
{
    pub fn new<T: AsRef<Path>>(path: T) -> Self
    {
        match Self::from_file(&path)
        {
            Ok(me) => me,
            Err(e) => 
            {
                println!("Failed to read project settings at {:?} because of: {e}", path.as_ref());
                Self::default()
            },
        }
    }
}


impl Default for ProjectSettings
{
    fn default() -> Self 
    {
        Self
        {
            version: "0.0.1".into(),
            name: "NewProject".into(),
            run_command: "".into(),
            build_command: "".into(),
            build_files: vec![],
        }
    }
}
