use std::path::{Path, PathBuf};
use fs_extra;
use fs_extra::dir::CopyOptions;
use crate::prelude::*;


pub struct Template
{
    path: PathBuf,
    name: String,
}


impl Template
{
    pub fn from_path<T: AsRef<Path>>(path: T) -> CprjResult<Self>
    {
        if !path.as_ref().is_dir()
        {
            return Err(CprjError::InvalidPath(format!("{:?} is not a directory", path.as_ref())));
        }

        let name = match path.as_ref().file_name()
        {
            Some(name) => match name.to_str()
            {
                Some(n) => n.to_string(),
                None => return Err(CprjError::InvalidPath(format!("{:?}, Cant be converted to String", path.as_ref()))),
            },
            None => return Err(CprjError::InvalidPath(format!("{:?}, Terminates in ..", path.as_ref()))),
        };

        let path = match path.as_ref().canonicalize()
        {
            Ok(pathbuf) => pathbuf,
            Err(e) => return Err(CprjError::InvalidPath(e.to_string())),
        };

        Ok(Self {name, path})
    }

    pub fn use_this<T: AsRef<Path>>(&self, target_location: T, new_name: &str) -> CprjResult<()>
    {
        let target_location = target_location.as_ref().join(new_name);

        if let Err(e) = std::fs::create_dir(&target_location)
        {
            return Err(CprjError::InvalidPath(e.to_string()));
        }

        if let Err(e) = fs_extra::dir::copy(&self.path, &target_location, &CopyOptions::default().content_only(true))
        {
            Err(CprjError::InvalidPath(e.to_string()))
        }

        else
        {
            Ok(())
        }
    }

    pub fn name(&self) -> &str
    {
        &self.name
    }

    pub fn path(&self) -> &Path
    {
        &self.path
    }
}