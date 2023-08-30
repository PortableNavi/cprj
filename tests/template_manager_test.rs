mod helpers;

use std::path::PathBuf;
use cprj::prelude::*;
use cprj::template_manager::TemplateManager;
use cprj::utility::paths::get_cprj_data_dir;



#[test]
fn finds_all_templates() -> CprjResult<()>
{
    helpers::dir_setup();

    let data_home = get_cprj_data_dir()?;
    let template_manager = TemplateManager::new(&data_home);

    assert_eq!(template_manager.templates()[0].name(), "Empty");
    assert_eq!(template_manager.templates()[0].path(), &PathBuf::from("/home/rk/dev/cprj/test_data/.local/share/cprj/user_templates/Empty"));

    Ok(())
}


#[test]
fn creates_new_project() -> CprjResult<()>
{
    helpers::dir_setup();

    let data_home = get_cprj_data_dir()?;
    let template_manager = TemplateManager::new(&data_home);

    template_manager.use_template("Empty", "NewProject", "test_data")?;

    Ok(())
}

