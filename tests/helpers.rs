use std::env;
use std::path::PathBuf;


pub fn dir_setup()
{
    // They call me 007: 0 panics, 0 iq, 7 unwraps... 
    env::set_var("XDG_DATA_HOME", PathBuf::from("test_data/.local/share").canonicalize().unwrap().to_str().unwrap());
    env::set_var("XDG_CONFIG_HOME", PathBuf::from("test_data/.config").canonicalize().unwrap().to_str().unwrap());

    let _ = std::fs::remove_file("test_data/.config/generated_config.toml");
    let _ = std::fs::remove_dir_all("test_data/NewProject/");
}

