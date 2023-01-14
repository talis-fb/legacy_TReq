use std::path::PathBuf;
use directories::ProjectDirs;

pub struct FileUtils;
impl FileUtils {
    pub fn create_path_if_it_does_not_exist(path: &PathBuf) -> Result<(), String> {
        if !std::fs::metadata(path).is_ok() {
            std::fs::create_dir_all(path).map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    pub fn get_data_dir() -> Option<PathBuf> {
        let project = ProjectDirs::from("", "", "TReq")?;
        Some(project.data_dir().to_path_buf())
    }

    pub fn get_config_dir() -> Option<PathBuf> {
        let project = ProjectDirs::from("", "", "TReq")?;
        Some(project.config_dir().to_path_buf())
    }
}
