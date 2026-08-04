use crate::services::db::create_database_fs;
use crate::utils::paths::get_app_root;

/// Create DATABASE IF NOT EXISTS via mysql.exe client.
/// Called from frontend [Create DB] button.
#[tauri::command]
pub fn create_database(db_name: String, mysql_port: Option<u16>) -> Result<(), String> {
    let root = get_app_root();
    let port = mysql_port.unwrap_or(3306);
    create_database_fs(&root, &db_name, port)
}

/// Alias with explicit snake naming compat
#[tauri::command]
pub fn create_db(db_name: String, mysql_port: Option<u16>) -> Result<(), String> {
    create_database(db_name, mysql_port)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmp_root(label: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!(
            "vanompp_cmd_db_{}_{}_{}",
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
    fn test_create_db_invalid_name() {
        let root = tmp_root("invalid");
        let err = create_database_fs(&root, "", 59999).unwrap_err();
        assert!(!err.is_empty());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_create_db_no_mysql_client() {
        let root = tmp_root("noclient");
        // Use high free port to avoid hitting real mysqld 3306 running in dev
        let err = create_database_fs(&root, "mydb", 59999).unwrap_err();
        assert!(!err.is_empty());
        let _ = fs::remove_dir_all(&root);
    }
}
