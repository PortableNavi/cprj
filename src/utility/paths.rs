use std::{env, fs};
use std::path::{PathBuf, Path};
use crate::prelude::*;


/// Tries to get the path of the data home directory
///
/// Checks whether $XDG_DATA_HOME is set to a directory with read and write permissions.
/// if such a directory could not be found, this function will default to $HOME/.local/share
/// if the default path cannot be expanded to a existing directory with read and write permissions
/// than an error is thrown.
pub fn get_data_dir() -> CprjResult<PathBuf>
{
    let data_dir: PathBuf;

    // Check whether XDG_DATA_HOME exists
    if let Ok(path) = env::var("XDG_DATA_HOME")
    {
        // If yes, data_dir is found
        data_dir = check_path(&path, true)?;
    }

    else
    {
        // If no, the default path has to be used.
        println!("XDG_DATA_HOME not set. Using default of ~/.local/share"); //TODO: Logging
        
        if let Ok(home) = env::var("HOME")
        {
            data_dir = check_path(PathBuf::from(home).join(".local/share"), true)?;
        }

        else
        {
            // If the default path is also not available, throw an error.
            return Err(CprjError::InvalidPath("Neither $HOME or $XDG_DATA_HOME is set.".into()));
        }
    }

    Ok(data_dir)
}


/// Gets or creates a cprj directory located in the data home directory.
/// 
/// Throws an error if a diretory with read and write permissions
/// could not be located or created.
///
/// See get_data_dir() for more details.
pub fn get_cprj_data_dir() -> CprjResult<PathBuf>
{   
    // Get valid data dir path
    let data_dir = get_data_dir()?;
    let cprj_data_dir = data_dir.join("cprj");
    
    // Check if a dir called cprj exists in data_dir
    // if not, try and create it.
    if let Err(_) = check_path(&cprj_data_dir, true)
    {
        // cprj does not exist. Try and create it.
        if let Err(e) = fs::create_dir(&cprj_data_dir)
        {
            return Err(CprjError::InvalidPath(e.to_string()));
        }
        
        // Return whether the dir exists now
        return check_path(&cprj_data_dir, true);
    };

    // Return the path to the cprj data dir
    Ok(cprj_data_dir)
}


/// Tries to get the path of the config directory
///
/// Checks whether $XDG_CONFIG_HOME_HOME is set to a directory with read and write permissions.
/// if such a directory could not be found, this function will default to $HOME/.config/
/// if the default path cannot be expanded to a existing directory with read and write permissions
/// than an error is thrown.
pub fn get_config_dir() -> CprjResult<PathBuf>
{
    let config_dir: PathBuf;

    // Check whether XDG_CONFIG_HOME exists
    if let Ok(path) = env::var("XDG_CONFIG_HOME")
    {
        // If yes, config_dir is found
        config_dir = check_path(&path, true)?;
    }

    else
    {
        // If no, the default path has to be used.
        println!("XDG_CONFIG_HOME not set. Using default of ~/.config"); //TODO: Logging
        
        if let Ok(home) = env::var("HOME")
        {
            config_dir = check_path(PathBuf::from(home).join(".config"), true)?;
        }

        else
        {
            // If the default path is also not available, throw an error.
            return Err(CprjError::InvalidPath("Neither $HOME or $XDG_CONFIG_HOME is set.".into()));
        }
    }

    Ok(config_dir)
}


/// Gets or creates a cprj directory located in the config directory.
/// 
/// Throws an error if a diretory with read and write permissions
/// could not be located or created.
///
/// See get_config_dir() for more details.
pub fn get_cprj_config_dir() -> CprjResult<PathBuf>
{   
    // Get valid config dir path
    let config_dir = get_config_dir()?;
    let cprj_config_dir = config_dir.join("cprj");
    
    // Check if a dir called cprj exists in config_dir
    // if not, try and create it.
    if let Err(_) = check_path(&cprj_config_dir, true)
    {
        // cprj does not exist. Try and create it.
        if let Err(e) = fs::create_dir(&cprj_config_dir)
        {
            return Err(CprjError::InvalidPath(e.to_string()));
        }
        
        // Return whether the dir exists now
        return check_path(&cprj_config_dir, true);
    };

    // Return the path to the cprj data dir
    Ok(cprj_config_dir)
}


/// Checks if a path: exists and has read and write permissions.
pub fn check_path(path: impl AsRef<Path>, should_be_dir: bool) -> CprjResult<PathBuf>
{
    if let Ok(metadata) = fs::metadata(&path.as_ref())
    {
        if metadata.permissions().readonly() || metadata.is_dir() != should_be_dir
        {
            Err(CprjError::InvalidPath(format!("\"{:?}\" is not directory={should_be_dir} or is readonly", &path.as_ref())))
        }

        else
        {
            Ok(PathBuf::from(&path.as_ref()))
        }
    }

    else
    {
        Err(CprjError::InvalidPath(format!("\"{:?}\" could not be read", &path.as_ref())))
    }
}

