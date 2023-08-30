use serde::{Deserialize, Serialize};
use toml_parse_from_file::FromFile;
use std::fs::File;
use std::io::Write;
use crate::prelude::*;
use crate::utility::paths::{get_cprj_config_dir, check_path};


#[derive(Deserialize, Serialize, FromFile, Default)]
pub struct Config
{
    pub default: Option<String>,
    pub template_repos: Option<Vec<String>>,
}


impl Config
{
    pub fn new(name: &str) -> CprjResult<Self>
    {
        if let Ok(config_file) = check_path(get_cprj_config_dir()?.join(name), false)
        {
            let ret = match Self::from_file(config_file)
            {
                Ok(config) => config,
                Err(e) =>
                {
                    println!("Error: {e:#?}");
                    Self::default()
                }
            };
             
            Ok(ret)
        }

        else
        {
            Ok(Self::default())
        }
    }


    pub fn write_config(&self, name: &str) -> CprjResult<()>
    {
        let config_name = get_cprj_config_dir()?.join(name);

        let mut file = match File::create(config_name)
        {
            Ok(file) => file,
            Err(e) => return Err(CprjError::InvalidPath(e.to_string())),
        };

        if let Err(e) = file.write_all(toml::to_string(&self).expect("should not fail").as_bytes())
        {
            return Err(CprjError::InvalidPath(e.to_string()))
        }

        Ok(())
    }
}





