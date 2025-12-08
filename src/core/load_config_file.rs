use crate::core::data_struct::Config;
use crate::core::normalize_path::normalize_path;
use std::fs;
use std::path::PathBuf;

pub fn load_config_from_file() -> Config {
    let file_path = "rc.config.json";

    let mut config: Config = Config { alias: None };

    // if the file exists, try to load config
    match fs::metadata(file_path) {
        Ok(_) => {
            let json_content = fs::read_to_string(file_path).unwrap();
            let config_in_file = serde_json::from_str::<Config>(&json_content).unwrap();
            let config_file_path = PathBuf::from(file_path).canonicalize().unwrap();
            if let Some(mut alias_config) = config_in_file.alias {
                for (_, value) in alias_config.iter_mut() {
                    *value =
                        normalize_path(&*config_file_path.join(PathBuf::from(value.to_string())))
                            .to_string_lossy()
                            .to_string();
                }
                config.alias = Some(alias_config);
            }
        }
        Err(_) => {}
    };

    config
}
