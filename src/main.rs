

use cprj::build_file_parser::Parser;
use cprj::cli::build_cli;

use cprj::template_manager::TemplateManager;
use cprj::prelude::*;
use cprj::settings::ProjectSettings;
use cprj::utility::paths::get_cprj_data_dir;
use std::process::{Command, ExitStatus};
use cprj::error::CprjError::Placeholder;


fn new(name: &str, template: &str) -> CprjResult<()>
{
    let data_home = get_cprj_data_dir()?;
    let template_manager = TemplateManager::new(&data_home);
    template_manager.use_template(&template, &name, "./")
}


fn build() -> CprjResult<ExitStatus>
{
    let project_settings = ProjectSettings::from_file(".cprj/settings.toml")?;
    let mut parser = Parser::new(project_settings, ".cprj/".into(), "./".into());

    parser.parse_all_build_files()?;

    if !parser.unknown_var_names().is_empty()
    {
        println!("Found unresolved cprj unresolved varnames:\n{:#?}\nAssuming their names as their values...\n", parser.unknown_var_names());
    }

    let build_command = &parser.project_settings().build_command;
    println!("--- Building {} ver. {} --------------------", &parser.project_settings().name, &parser.project_settings().version);
    let exit_status = spawn_command(&build_command)?;
    println!("\nBuild Command Exit Status: ({exit_status})");
    println!("--- Finished Building {} ver. {} -----------", &parser.project_settings().name, &parser.project_settings().version);

    Ok(exit_status)
}


fn run() -> CprjResult<ExitStatus>
{
    let build_exit_status = build()?;

    if !build_exit_status.success()
    {
        return Err(Placeholder("Build Failed. Not Running".into()))
    }

    let project_settings = ProjectSettings::from_file(".cprj/settings.toml")?;
    println!("\n--- Running {} ver. {} ---------------------", &project_settings.name, &project_settings.version);
    let exit_status = spawn_command(&project_settings.run_command)?;
    println!("\nRun Command Exit Status: ({exit_status})");
    println!("--- Finished Running {} ver. {} ------------", &project_settings.name, &project_settings.version);

    Ok(exit_status)
}

fn list_templates(line: bool) -> CprjResult<()>
{
    let data_home = get_cprj_data_dir()?;
    let template_manager = TemplateManager::new(&data_home);

    if !line
    {
        for template in template_manager.templates()
        {
            println!("{}", template.name());
        }
    }

    else
    {
        let templates = template_manager.templates();

        if templates.len() <= 0
        {
            return Ok(());
        }

        let last_index = templates.len()-1;

        for template in &templates[..last_index]
        {
            print!("{}, ", template.name());
        }

        println!("{}", templates[last_index].name());
    }

    Ok(())
}


fn spawn_command(cmd: &str) -> CprjResult<ExitStatus>
{
    let command = Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .spawn();

    if let Ok(mut handle) = command
    {
        match handle.wait()
        {
            Ok(status) => Ok(status),
            Err(e) => Err(Placeholder(e.to_string())),
        }
    }

    else
    {
        Err(Placeholder("Failed to invoke command".to_string()))
    }
}


fn main()
{
    let matches = build_cli().get_matches();

    match matches.subcommand()
    {
        Some(("new", submatches)) =>
        {
            if let (Some(name), Some(template)) = (submatches.get_one::<String>("NAME"), submatches.get_one::<String>("template"))
            {
                if let Err(e) = new(&name, &template)
                {
                    println!("{}", e.to_string());
                }
            }
        },

        Some(("build", _)) =>
        {
            if let Err(e) = build()
            {
                println!("{}", e.to_string());
            }
        },

        Some(("run", _)) =>
        {
            if let Err(e) = run()
            {
                println!("{}", e.to_string());
            }
        },

        Some(("list_templates", submatches)) =>
            {
                let line = match submatches.get_one("line")
                {
                    Some(val) => *val,
                    None => false,
                };

                if let Err(e) = list_templates(line)
                {
                    println!("{}", e.to_string());
                }
            },

        _ => {},
    }
}

