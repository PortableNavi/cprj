use cprj::settings::ProjectSettings;




#[test]
fn from_correct_settings()
{
    let settings = ProjectSettings::new("test_data/TestProject/.cprj/settings.toml");
    assert_eq!(&settings.name, "TestProject");
    assert_eq!(&settings.version, "0.0.2");
    assert_eq!(settings.build_files, vec!["makefile"]);
}


#[test]
fn from_broken_settings()
{
    let settings = ProjectSettings::new("test_data/TestProject/.cprj/broken_settings.toml");
    assert_eq!(&settings.name, "NewProject");
    assert_eq!(&settings.version, "0.0.1");
    assert_eq!(settings.build_files, Vec::<String>::new());
}

