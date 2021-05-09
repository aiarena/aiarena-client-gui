use directories::ProjectDirs;

#[allow(dead_code)]
pub fn project_directory() -> Result<String, Box<dyn std::error::Error>> {
    let project_dirs = ProjectDirs::from("org", "AIArena", "GUI")
        .ok_or_else(|| "Could not find Project Directory".to_string())?;
    if !project_dirs.data_local_dir().exists() {
        std::fs::create_dir_all(project_dirs.data_local_dir())?;
    }
    Ok(project_dirs.data_local_dir().display().to_string())
}

pub fn get_non_empty_string(string: String) -> anyhow::Result<String> {
    if string.is_empty() {
        bail!(" string is empty")
    } else {
        Ok(string)
    }
}
