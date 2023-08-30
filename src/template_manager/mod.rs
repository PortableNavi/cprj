pub mod template;

use std::path::Path;
use template::Template;
use crate::prelude::*;


pub struct TemplateManager
{
    user_templates: Vec<Template>,
}


impl TemplateManager
{
    pub fn new<T: AsRef<Path>>(data_home: T) -> Self
    {
        let user_template_dir = data_home.as_ref().join("user_templates");
        let mut user_templates = vec![];

        if let Ok(names) = std::fs::read_dir(&user_template_dir)
        {
            for wrapped_name in names
            {
                if let Ok(name) = wrapped_name
                {
                    let path = name.path();

                    if path.is_dir()
                    {
                        if let Ok(template) = Template::from_path(path)
                        {
                            user_templates.push(template);
                        }
                    }
                }
            }
        }

        Self {user_templates}
    }

    pub fn use_template(&self, name: &str, new_name: &str, location: &str) -> CprjResult<()>
    {
        let mut template: Option<&Template> = None;

        for project in &self.user_templates
        {
            if project.name() == name
            {
                template = Some(&project);
            }
        }

        if let Some(project) = template
        {
            project.use_this(&location, &new_name)?;
            Ok(())
        }

        else
        {
            Err(CprjError::NoTemplate(name.to_string()))
        }
    }

    pub fn templates(&self) -> &Vec<Template>
    {
        &self.user_templates
    }
}