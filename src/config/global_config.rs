use serde::{Deserialize, Serialize};
use std::{fs::File, path::Path};

static CONFIG_FILE_NAME: &str = "donkey.yaml";

#[derive(Deserialize, Serialize, Clone)]
pub struct GlobalConfig {
    pub vsc_settings_path: String,
}

impl GlobalConfig {
    fn get_owner_path() -> String {
        let path = dirs_next::home_dir()
            .map(|mut path| {
                path.extend([".ccbond", &CONFIG_FILE_NAME]);
                path.to_str().unwrap().to_string()
            })
            .unwrap();
        path
    }

    pub fn init() -> GlobalConfig {
        let new_global_config = GlobalConfig {
            vsc_settings_path: "/Library/Application Support/Code/User/settings.json".to_string(),
        };
        let path = Self::get_owner_path();
        let new_config_file = File::create(path).unwrap();
        serde_yaml::to_writer(new_config_file, &new_global_config).unwrap();
        new_global_config
    }

    pub fn load() -> GlobalConfig {
        let path = Self::get_owner_path();

        let target_path = Path::new(&path);
        if !target_path.exists() {
            return Self::init();
        }

        let config_file = File::open(path).unwrap();

        if config_file.metadata().unwrap().len() == 0 {
            return Self::init();
        }

        let global_config: GlobalConfig = serde_yaml::from_reader(config_file).unwrap();
        global_config
    }
}
