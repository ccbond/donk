use crate::config::global_config::GlobalConfig;
use std::process::Command;

pub fn process(config: &GlobalConfig) {
    let vsc_settings_path = config.vsc_settings_path.clone();

    let mut path = dirs_next::home_dir().unwrap().to_str().unwrap().to_string();

    path += &vsc_settings_path;

    Command::new("code")
        .arg("-r")
        .arg(path)
        .spawn()
        .expect("Failed to open vsc settings.json");
}
