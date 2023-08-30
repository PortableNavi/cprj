use std::path::PathBuf;
use cprj::prelude::*;
use cprj::build_file_parser::*;
use cprj::settings::ProjectSettings;

mod helpers;


#[test]
fn finds_all_placeholders() -> CprjResult<()>
{
    helpers::dir_setup();

    let settings = ProjectSettings::new("test_data/TestProject/.cprj/settings.toml");
    let build_file = PathBuf::from("test_data/TestProject").join(&settings.build_files[0]);

    let parser = Parser::new(settings, PathBuf::from("test_data/TestProject.cprj/"), PathBuf::from("test_data/TestProject"));
    
    let placeholders = parser.parse_file(&build_file)?;
    
    assert_eq!(&placeholders[0].var_name, "name");
    assert_eq!(placeholders[0].intervall, (16, 22));
    assert_eq!(placeholders[0].line, 0);

    assert_eq!(&placeholders[1].var_name, "version");
    assert_eq!(placeholders[1].intervall, (19, 28));
    assert_eq!(placeholders[1].line, 1);

    assert_eq!(&placeholders[2].var_name, "name");
    assert_eq!(placeholders[2].intervall, (15, 21));
    assert_eq!(placeholders[2].line, 3);

    assert_eq!(&placeholders[3].var_name, "version");
    assert_eq!(placeholders[3].intervall, (23, 32));
    assert_eq!(placeholders[3].line, 3);

    assert_eq!(placeholders.len(), 5);

    Ok(())
}


#[test]
fn replaces_all_placeholders() -> CprjResult<()>
{
    helpers::dir_setup();

    let project_dir = PathBuf::from("test_data/TestProject");
    let mut parser = Parser::new_with_settings(project_dir.clone(), project_dir.join(".cprj/"));

    let build_file = PathBuf::from("test_data/TestProject/makefile");

    let placeholders = parser.parse_file(&build_file)?;

    assert!(parser.write_file("makefile", placeholders).is_ok());

    Ok(())
}


#[test]
fn parse_all_works() -> CprjResult<()>
{
    helpers::dir_setup();

    let project_dir = PathBuf::from("test_data/TestProject");
    let mut parser = Parser::new_with_settings(project_dir.clone(), project_dir.join(".cprj/"));

    assert!(parser.parse_all_build_files().is_ok());

    assert_eq!(parser.unknown_var_names(), &vec!["HabIchVergessen"]);

    Ok(())
}
