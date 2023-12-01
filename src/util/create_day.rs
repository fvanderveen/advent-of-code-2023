use crate::days::get_day;
use std::fs::{read_to_string, write,};
use std::path::{Path};
use std::env::{current_dir};
use regex::{Captures, RegexBuilder};
use handlebars::{Handlebars};
use serde_json::{json};

pub fn create_day(day: i32) -> Result<(), String> {
    match get_day(day) {
        Err(_) => {
            let main_dir = match current_dir() {
                Ok(dir) => dir,
                Err(e) => { return Err(format!("Could not get working directory: {}", e)); }
            };
            let source_file_name = format!("src/days/day{:02}.rs", day);
            let source_path = main_dir.join(Path::new(&source_file_name));
            let template_file_name = format!("resources/day{:02}.txt", day);
            let input_path = main_dir.join(Path::new(&template_file_name));
            let module_file_name = "src/days.rs".to_string();
            let module_path = main_dir.join(Path::new(&module_file_name));

            if source_path.exists() {
                return Err(format!("Source file for day {} already exists.", day));
            }
            if input_path.exists() {
                return Err(format!("Input file for day {} already exists.", day));
            }

            let template = match read_to_string(main_dir.join("resources/day.rs.hbs")) {
                Ok(v) => { v }
                Err(e) => { return Err(format!("Could not read day template: {}", e)); }
            };
            let days_mod_content = match read_to_string(&module_path) {
                Ok(v) => { v }
                Err(e) => { return Err(format!("Could not read days module file: {}", e)); }
            };

            let import_regex = match RegexBuilder::new("^(\\s*)(// « add day import »)").multi_line(true).build() {
                Ok(r) => { r }
                Err(e) => { return Err(format!("{}", e)); }
            };
            let match_regex = match RegexBuilder::new("^(\\s*)(// « add day match »)").multi_line(true).build() {
                Ok(r) => { r }
                Err(e) => { return Err(format!("{}", e)); }
            };

            if !import_regex.is_match(days_mod_content.as_str()) {
                println!("{}", days_mod_content);
                return Err("Could not find import comment in days module".to_string());
            }
            if !match_regex.is_match(days_mod_content.as_str()) {
                return Err("Could not find match comment in days module".to_string());
            }

            let res1 = import_regex.replace(days_mod_content.as_str(), |caps: &Captures| {
                format!("{ws}mod day{day:02};\n{ws}use day{day:02}::DAY{day};\n{ws}{comm}", ws = &caps[1], comm = &caps[2], day = day)
            });
            let module_content = match_regex.replace(res1.as_ref(), |caps: &Captures| {
                format!("{ws}{day} => Ok(DAY{day}),\n{ws}{comm}", ws = &caps[1], comm = &caps[2], day = day)
            });

            let handlebars = Handlebars::new();
            let day_content = match handlebars.render_template(template.as_str(), &json!({ "day": day })) {
                Ok(v) => { v }
                Err(e) => { return Err(format!("{}", e)); }
            };

            match write(&input_path, "TODO: Add Content Here") { Err(e) => { return Err(format!("Could not write input file: {:?}\nError: {}", input_path, e)); }, _ => {} }
            match write(&source_path, day_content) { Err(e) => { return Err(format!("Could not write day file: {:?}\nError: {}", source_path, e)); }, _ => {} }
            match write(&module_path, module_content.to_string()) { Err(e) => { return Err(format!("Could not write module file: {:?}\nError: {}", module_path, e)); }, _ => {} }

            Ok(())
        }
        Ok(_) => {
            Err(format!("Day {} already exists!", day))
        }
    }
}