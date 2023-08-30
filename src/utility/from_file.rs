use std::path::Path;
use std::fs;
use crate::prelude::*;
use serde::Deserialize;


pub trait FromFile
{
    fn from_file<T: AsRef<Path>>(path: T) -> CprjResult<Self> where Self: Sized, for<'de> Self: Deserialize<'de>
    {
        match fs::read_to_string(path.as_ref())
        {
            Ok(contents) => match toml::from_str::<Self>(&contents)
            {
                Ok(me) => Ok(me),
                Err(e) => Err(CprjError::Placeholder(e.to_string())),
            },

            Err(e) => Err(CprjError::InvalidPath(format!("Failed to read {:?}, because: {e}", path.as_ref()))),
        }
    }
}

