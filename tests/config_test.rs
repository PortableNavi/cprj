use cprj::config::Config;
use cprj::prelude::*;
mod helpers;


#[test]
fn success_from_correct_config()
{
    let config = Config::from_file("test_data/.config/cprj/config.toml").unwrap();
    assert_eq!(config.default, Some("repo2/Default".into()));
    assert_eq!(config.template_repos, Some(["repo1".into(), "repo2".into(), "repo3".into()].into()));
}


#[test]
fn sucess_from_empty_config()
{
    let config = Config::from_file("test_data/.config/cprj/empty_config.toml").unwrap();
    assert_eq!(config.default, None);
    assert_eq!(config.template_repos, None);
}


#[test]
fn error_from_faulty_config()
{
    let config = Config::from_file("test_data/.config/cprj/faulty_config.toml");
    assert!(config.is_err());
}


#[test]
fn auto_find_correct_config_file()
{
    helpers::dir_setup();
    let config = Config::new("config.toml").unwrap();
    assert_eq!(config.default, Some("repo2/Default".into()));
    assert_eq!(config.template_repos, Some(["repo1".into(), "repo2".into(), "repo3".into()].into()));
}


#[test]
fn auto_find_empty_config_file()
{
    helpers::dir_setup();
    let config = Config::new("empty_config.toml").unwrap();
    assert_eq!(config.default, None);
    assert_eq!(config.template_repos, None);
}


#[test]
fn auto_find_faulty_config_file()
{
    helpers::dir_setup();
    let config = Config::new("faulty_config.toml").unwrap();
    assert_eq!(config.default, None);
    assert_eq!(config.template_repos, None);
}


#[test]
fn write_config()
{
    helpers::dir_setup();
    
    let mut config = Config::new("config.toml").unwrap();
    config.default = Some("repo3/Blank".into());
    
    config.write_config("generated_config.toml").unwrap();

    let generated_config = Config::new("generated_config.toml").unwrap();

    assert_eq!(generated_config.default, Some("repo3/Blank".into()));
    assert_eq!(generated_config.template_repos, config.template_repos);

}
