use std::path::Path;

use crate::projects::creator::create_project_fs;
use crate::projects::scanner::scan_projects_fs;
use crate::projects::ProjectInfo;
use crate::utils::paths::{ensure_www, get_app_root};

const DEFAULT_APACHE_PORT: u16 = 8080;

fn get_www_path() -> Result<std::path::PathBuf, String> {
    let root = get_app_root();
    ensure_www(&root)
        .map_err(|e| format!("Gagal siapkan folder www: {}", e))
}

#[tauri::command]
pub fn scan_projects() -> Result<Vec<ProjectInfo>, String> {
    let www_path = get_www_path()?;
    scan_projects_fs(&www_path, DEFAULT_APACHE_PORT)
}

/// Scan with custom port (used when apache port conflicts and switches)
#[tauri::command]
pub fn scan_projects_with_port(apache_port: u16) -> Result<Vec<ProjectInfo>, String> {
    let www_path = get_www_path()?;
    scan_projects_fs(&www_path, apache_port)
}

#[tauri::command]
pub fn create_project(name: String, create_db: bool, db_name: String) -> Result<ProjectInfo, String> {
    let www_path = get_www_path()?;
    create_project_fs(&name, create_db, &db_name, &www_path, DEFAULT_APACHE_PORT)
}

/// Create with explicit port
#[tauri::command]
pub fn create_project_with_port(
    name: String,
    create_db: bool,
    db_name: String,
    apache_port: u16,
) -> Result<ProjectInfo, String> {
    let www_path = get_www_path()?;
    create_project_fs(&name, create_db, &db_name, &www_path, apache_port)
}

#[tauri::command]
pub fn open_project_folder(name: String) -> Result<(), String> {
    let www_path = get_www_path()?;
    let target = www_path.join(&name);
    if !target.exists() {
        return Err(format!("Folder {} tidak ada", name));
    }
    open_folder_path(&target)
}

fn open_folder_path(path: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(path)
            .spawn()
            .map_err(|e| format!("Gagal buka folder: {}", e))?;
        Ok(())
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = path;
        Err("Buka folder hanya Windows".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::projects::creator::create_project_fs;
    use crate::projects::scanner::scan_projects_fs;
    use std::fs;

    fn tmp_root(label: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!(
            "vanompp_cmd_{}_{}_{}",
            label,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
                % 100000
        ));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn test_scan_and_create_integration() {
        let root = tmp_root("integration");
        let www = root.join("www");
        fs::create_dir_all(&www).unwrap();

        // empty scan
        let empty = scan_projects_fs(&www, 8080).unwrap();
        assert!(empty.is_empty());

        create_project_fs("hello-world", false, "", &www, 8080).unwrap();
        let list = scan_projects_fs(&www, 8080).unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "hello-world");
        assert!(list[0].has_index);

        let _ = fs::remove_dir_all(&root);
    }
}
