use cprj::utility::paths::*;
mod helpers;



#[test]
fn data_home_is_found() -> Result<(), Box<dyn std::error::Error>>
{
    helpers::dir_setup();

    let path = get_cprj_data_dir();
    println!("{path:#?}");
    assert!(path.is_ok());
    
    // This is obviously not a reliable method of
    // checking whether two paths are the same but
    // it will suffice for now...
    //
    // I did choose to do it this way because it was quick
    // and i didn't want to use the same method i used to get the absolute path
    // of test_data to get another absolute path of test_data to see
    // whether they point to the same location or not
    assert_eq!(
        std::fs::metadata(path?)?.created()?, 
        std::fs::metadata("./test_data/.local/share/cprj")?.created()?
    );

    Ok(())
}


#[test]
fn config_home_is_found() -> Result<(), Box<dyn std::error::Error>>
{
    helpers::dir_setup();

    let path = get_cprj_config_dir();
    assert!(path.is_ok());
    
    // Again, This is obviously not a reliable method of
    // checking whether two paths are the same...
    assert_eq!(
        std::fs::metadata(path?)?.created()?, 
        std::fs::metadata("./test_data/.config/cprj")?.created()?
    );

    Ok(())
}
