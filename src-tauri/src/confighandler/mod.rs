use platform_dirs::AppDirs;
use configparser::ini::Ini;

#[tauri::command]
pub fn load_config() {
    let mut config = Ini::new();
    let app_dirs = AppDirs::new(Some("ChaMan"), true).unwrap();
    let config_path = app_dirs.config_dir;
    let config_file = config_path.join("config.ini");

    if !config_file.exists() {
        // Initialize config file with default values
        config.set("General", "theme", "light");
    }
}

#[tauri::command]
pub fn save_config() {
    let app_dirs = AppDirs::new(Some("ChaMan"), true).unwrap();
    let config_path = app_dirs.config_dir;
    let config_file = config_path.join("config.ini");
    let mut config = Ini::new();
}
