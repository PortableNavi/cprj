use std::fs::File;
use std::io::{BufReader, BufRead, BufWriter, Write};
use std::path::{Path, PathBuf};

use crate::settings::ProjectSettings;
use crate::utility::pop_front::PopFront;
use crate::prelude::*;


#[derive(Debug)]
pub struct Placeholder
{
    pub var_name: String,
    pub intervall: (usize, usize),
    pub line: usize,
}


pub struct Parser
{
    project_settings: ProjectSettings,
    project_dir: PathBuf,
    settings_root_dir: PathBuf,
    unknown_var_names: Vec<String>,
}


impl Parser
{
    pub fn new(project_settings: ProjectSettings, settings_root_dir: PathBuf, project_dir: PathBuf) -> Self
    {
        Self {project_settings, settings_root_dir, project_dir, unknown_var_names: vec![]}
    }

    pub fn new_with_settings(project_dir: PathBuf, settings_root_dir: PathBuf) -> Self
    {
        let project_settings = ProjectSettings::new(settings_root_dir.join("settings.toml"));
        Self {project_settings, project_dir, settings_root_dir, unknown_var_names: vec![]}
    }

    pub fn parse_file<T: AsRef<Path>>(&self, path: T) -> CprjResult<Vec<Placeholder>>
    { 
        let mut line_counter = 0;
        let mut placeholders: Vec<Placeholder> = vec![];

        for build_line in Self::read(&path)?.lines()
        {
            if let Ok(line) = build_line
            {
                let mut index = 0;
                loop
                {
                    if index > line.len()
                    {
                        break;
                    }

                    let l = &line[index..];

                    if let Some(start_index) = l.find("@[")
                    {
                        if let Some(end_index) = (&l[start_index+2..]).find("]")
                        {

                            if end_index > 0
                            {
                                let end_index = start_index + end_index + 2;
                                let var_name = l[start_index + 2..end_index].to_string();
                                placeholders.push(Placeholder { var_name, intervall: (start_index + index, end_index + index), line: line_counter });
                                index += end_index;
                            }

                            else
                            {
                                index += start_index+1;
                            }
                        }

                        else
                        {
                            break;
                        }
                    }

                    else
                    {
                        break;
                    }
                }

                line_counter += 1;
            }
        }

        Ok(placeholders)
    }

    pub fn write_file<T: AsRef<Path>>(&mut self, name: T, mut placeholders: Vec<Placeholder>) -> CprjResult<()>
    {
        let mut writer = Self::write(self.settings_root_dir.join(&name))?;
        let mut placeholder = placeholders.pop_front();
        let build_file_path = &self.project_dir.join(&name);

        for (i, wrapped_line) in Self::read(build_file_path)?.lines().enumerate()
        {
            if let Ok(mut line) = wrapped_line
            {
                let mut index_change: isize = 0;

                loop
                {
                    if let Some(pholder) = &placeholder
                    {
                        if  i == pholder.line
                        {
                            let start = (pholder.intervall.0 as isize + index_change) as usize;
                            let end = (pholder.intervall.1 as isize + 1 + index_change) as usize;

                            let original_length = line.len() as isize;

                            line = format!("{}{}{}", &line[..start], &self.resolve_var_name(&pholder.var_name), &line[end..]);

                            index_change += (line.len() as isize) - original_length;
                        }

                        else
                        {
                            break;
                        }
                    }

                    else
                    {
                        break;
                    }

                    placeholder = placeholders.pop_front();
                }

                let _ = writer.write(line.as_bytes());
                let _ = writer.write(b"\n");
            }
        }

        Ok(())
    }

    pub fn parse_all_build_files(&mut self) -> CprjResult<()>
    {
        let build_files = self.project_settings.build_files.clone();

        for build_file in &build_files
        {
            let placeholders = self.parse_file(self.project_dir.join(build_file))?;
            self.write_file(build_file, placeholders)?;
        }

        Ok(())
    }

    pub fn unknown_var_names(&self) -> &Vec<String>
    {
        &self.unknown_var_names
    }

    pub fn project_settings(&self) -> &ProjectSettings
    {
        &self.project_settings
    }

    fn resolve_var_name(&mut self, name: &str) -> String
    {
        match name
        {
            "version" => self.project_settings.version.clone(),
            "name" => self.project_settings.name.clone(),
            name =>
            {
                self.unknown_var_names.push(name.to_string());
                name.to_string()
            },
        }
    }

    fn read<T: AsRef<Path>>(path: T) -> CprjResult<BufReader<File>>
    {
        let file = match File::open(&path)  
        {
            Ok(file) => file,
            Err(e) => return Err(CprjError::Placeholder(e.to_string())), 
        };

        Ok(BufReader::new(file))
    }

    fn write<T: AsRef<Path>>(path: T) ->CprjResult<BufWriter<File>>
    {
        let file = match File::create(&path)
        {
            Ok(file) => file,
            Err(e) => return Err(CprjError::Placeholder(e.to_string())),
        };

        Ok(BufWriter::new(file))
    }
}
