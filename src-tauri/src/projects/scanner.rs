use std::path::Path;

use super::ProjectInfo;
use crate::services::db::check_db_exists_fs;

const SKIP_DIRS: &[&str] = &["__vano_health"];

fn check_file(dir: &Path, name: &str) -> bool {
    dir.join(name).is_file()
}

/// Scan `www_root` for project folders.
/// - Skips non-dir entries and `__vano_health`.
/// - Builds url as `http://localhost:{port}/{folderName}`
/// - Sorted alphabetically.
/// - db_exists checked via filesystem: bin/mysql/data/{db} exists.
pub fn scan_projects_fs(www_root: &Path, apache_port: u16) -> Result<Vec<ProjectInfo>, String> {
    if !www_root.exists() {
        return Ok(Vec::new());
    }

    let read_dir =
        std::fs::read_dir(www_root).map_err(|e| format!("Gagal baca www folder: {}", e))?;

    let mut projects = Vec::new();

    for entry_res in read_dir {
        let entry = match entry_res {
            Ok(e) => e,
            Err(_) => continue,
        };
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_string(),
            None => continue,
        };
        if SKIP_DIRS.contains(&file_name.as_str()) {
            continue;
        }
        if file_name.starts_with('.') {
            continue;
        }

        let has_index = check_file(&path, "index.php");
        let has_conn = check_file(&path, "conn.php");
        let has_gitignore = check_file(&path, ".gitignore");

        // db name heuristics: folder name with - replaced by _ (same as creator)
        let db_name_guess = file_name.replace('-', "_").to_lowercase();

        let (db_exists, db_name_final) = if has_conn {
            // Prefer reading conn.php $db value if present, else guess
            let db_from_conn = read_db_from_conn(&path);
            let db_to_check = db_from_conn.clone().unwrap_or_else(|| db_name_guess.clone());
            if db_to_check.is_empty() {
                (false, String::new())
            } else {
                let exists = check_db_exists_fs(www_root, &db_to_check);
                (exists, db_to_check)
            }
        } else {
            (false, String::new())
        };

        let url = format!("http://localhost:{}/{}", apache_port, file_name);

        projects.push(ProjectInfo {
            name: file_name.clone(),
            path: path.to_string_lossy().to_string(),
            url,
            has_index,
            has_conn,
            has_gitignore,
            db_exists,
            db_name: db_name_final,
        });
    }

    projects.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(projects)
}

fn read_db_from_conn(project_path: &Path) -> Option<String> {
    let conn_path = project_path.join("conn.php");
    let content = std::fs::read_to_string(conn_path).ok()?;
    // naive parse: $db = "xxx" or $db   = 'xxx' or $db   = "xxx";
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("$db") {
            if let Some(eq_idx) = trimmed.find('=') {
                let after = trimmed[eq_idx + 1..].trim();
                let after = after.trim_end_matches(';').trim();
                if after.starts_with('"') || after.starts_with('\'') {
                    let quote = after.chars().next().unwrap();
                    if let Some(end) = after[1..].find(quote) {
                        let db = &after[1..1 + end];
                        if !db.is_empty() {
                            return Some(db.to_string());
                        } else {
                            return Some(String::new());
                        }
                    }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmp_root(label: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!(
            "vanompp_scan_{}_{}_{}",
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
    fn test_scan_empty_folder() {
        let root = tmp_root("empty");
        let www = root.join("www");
        fs::create_dir_all(&www).unwrap();
        let res = scan_projects_fs(&www, 8080).unwrap();
        assert!(res.is_empty());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_scan_basic() {
        let root = tmp_root("basic");
        let www = root.join("www");
        fs::create_dir_all(&www).unwrap();
        fs::create_dir_all(www.join("my-app")).unwrap();
        fs::write(www.join("my-app/index.php"), "<?php").unwrap();
        fs::write(www.join("my-app/conn.php"), "<?php $c=null;").unwrap();
        fs::create_dir_all(www.join("__vano_health")).unwrap();
        fs::create_dir_all(www.join("project-b")).unwrap();

        let res = scan_projects_fs(&www, 8080).unwrap();
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].name, "my-app");
        assert_eq!(res[1].name, "project-b");
        assert!(res[0].has_index);
        assert!(res[0].has_conn);
        assert!(!res[0].has_gitignore);
        assert_eq!(res[0].url, "http://localhost:8080/my-app");
        assert!(!res.iter().any(|p| p.name == "__vano_health"));

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_scan_skips_files_and_hidden() {
        let root = tmp_root("skip");
        let www = root.join("www");
        fs::create_dir_all(&www).unwrap();
        fs::write(www.join(".gitkeep"), "").unwrap();
        fs::create_dir_all(www.join(".hidden")).unwrap();
        fs::create_dir_all(www.join("real")).unwrap();

        let res = scan_projects_fs(&www, 8080).unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].name, "real");

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_scan_port_in_url() {
        let root = tmp_root("porturl");
        let www = root.join("www");
        fs::create_dir_all(www.join("a")).unwrap();
        let res = scan_projects_fs(&www, 3100).unwrap();
        assert_eq!(res[0].url, "http://localhost:3100/a");
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_scan_db_exists_fs() {
        let root = tmp_root("dbexists");
        let www = root.join("www");
        let data_dir = root.join("bin").join("mysql").join("data").join("my_app");
        fs::create_dir_all(www.join("my-app")).unwrap();
        fs::write(www.join("my-app/conn.php"), r#"<?php $db = "my_app"; "#).unwrap();
        fs::create_dir_all(&data_dir).unwrap();

        let res = scan_projects_fs(&www, 8080).unwrap();
        assert_eq!(res.len(), 1);
        assert!(res[0].db_exists, "should detect db folder present");
        assert_eq!(res[0].db_name, "my_app");

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_scan_db_not_exists_when_folder_missing() {
        let root = tmp_root("dbmissing");
        let www = root.join("www");
        fs::create_dir_all(www.join("my-app")).unwrap();
        fs::write(www.join("my-app/conn.php"), r#"<?php $db = "my_app";"#).unwrap();

        let res = scan_projects_fs(&www, 8080).unwrap();
        assert!(!res[0].db_exists);
        assert_eq!(res[0].db_name, "my_app");

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_scan_db_name_empty_when_no_conn() {
        let root = tmp_root("nodb");
        let www = root.join("www");
        fs::create_dir_all(www.join("a")).unwrap();
        let res = scan_projects_fs(&www, 8080).unwrap();
        assert_eq!(res[0].db_name, "");
        assert!(!res[0].db_exists);
        let _ = fs::remove_dir_all(&root);
    }
}
