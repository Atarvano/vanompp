use std::path::{Path, PathBuf};
use std::process::Command;

/// Resolve mysql client binary (mysql.exe) using same logic as resolve_bin.
fn resolve_mysql_client_bin(app_root: &Path) -> Option<PathBuf> {
    let mut candidates = Vec::new();
    candidates.push(app_root.join("bin").join("mysql").join("bin").join("mysql.exe"));
    candidates.push(app_root.join("resources").join("bin").join("mysql").join("bin").join("mysql.exe"));
    candidates.push(
        app_root
            .join("src-tauri")
            .join("resources")
            .join("bin")
            .join("mysql")
            .join("bin")
            .join("mysql.exe"),
    );
    // walk up ancestors
    let mut cur = app_root.to_path_buf();
    for _ in 0..6 {
        candidates.push(cur.join("bin").join("mysql").join("bin").join("mysql.exe"));
        candidates.push(
            cur.join("resources")
                .join("bin")
                .join("mysql")
                .join("bin")
                .join("mysql.exe"),
        );
        candidates.push(
            cur.join("src-tauri")
                .join("resources")
                .join("bin")
                .join("mysql")
                .join("bin")
                .join("mysql.exe"),
        );
        if !cur.pop() {
            break;
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        let mut cur = cwd;
        for _ in 0..4 {
            candidates.push(cur.join("bin").join("mysql").join("bin").join("mysql.exe"));
            candidates.push(
                cur.join("resources")
                    .join("bin")
                    .join("mysql")
                    .join("bin")
                    .join("mysql.exe"),
            );
            candidates.push(
                cur.join("src-tauri")
                    .join("resources")
                    .join("bin")
                    .join("mysql")
                    .join("bin")
                    .join("mysql.exe"),
            );
            if !cur.pop() {
                break;
            }
        }
    }
    for p in candidates {
        if p.exists() {
            return Some(p);
        }
    }
    None
}

/// Resolve mysql data dir (where database folders live).
fn resolve_mysql_data_dir(app_root: &Path) -> Option<PathBuf> {
    let mut candidates = Vec::new();
    candidates.push(app_root.join("bin").join("mysql").join("data"));
    candidates.push(app_root.join("resources").join("bin").join("mysql").join("data"));
    candidates.push(
        app_root
            .join("src-tauri")
            .join("resources")
            .join("bin")
            .join("mysql")
            .join("data"),
    );
    let mut cur = app_root.to_path_buf();
    for _ in 0..6 {
        candidates.push(cur.join("bin").join("mysql").join("data"));
        candidates.push(
            cur.join("resources")
                .join("bin")
                .join("mysql")
                .join("data"),
        );
        candidates.push(
            cur.join("src-tauri")
                .join("resources")
                .join("bin")
                .join("mysql")
                .join("data"),
        );
        if !cur.pop() {
            break;
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        let mut cur = cwd;
        for _ in 0..4 {
            candidates.push(cur.join("bin").join("mysql").join("data"));
            candidates.push(cur.join("resources").join("bin").join("mysql").join("data"));
            candidates.push(
                cur.join("src-tauri")
                    .join("resources")
                    .join("bin")
                    .join("mysql")
                    .join("data"),
            );
            if !cur.pop() {
                break;
            }
        }
    }
    for p in candidates {
        if p.exists() && p.is_dir() {
            return Some(p);
        }
    }
    None
}

/// Resolve app root from www path similar to previous logic.
pub fn resolve_app_root_from_www(www_root: &Path) -> PathBuf {
    // www is <app_root>/www → parent
    if let Some(parent) = www_root.parent() {
        return parent.to_path_buf();
    }
    www_root.to_path_buf()
}

/// Check if db folder exists on filesystem.
pub fn check_db_exists_fs(www_root: &Path, db_name: &str) -> bool {
    if db_name.trim().is_empty() {
        return false;
    }
    let app_root = resolve_app_root_from_www(www_root);
    if let Some(data_dir) = resolve_mysql_data_dir(&app_root) {
        let db_dir = data_dir.join(db_name.trim());
        return db_dir.exists() && db_dir.is_dir();
    }
    false
}

/// Alternative: check via app_root directly (used by commands).
pub fn check_db_exists_by_app_root(app_root: &Path, db_name: &str) -> bool {
    if db_name.trim().is_empty() {
        return false;
    }
    if let Some(data_dir) = resolve_mysql_data_dir(app_root) {
        let db_dir = data_dir.join(db_name.trim());
        return db_dir.exists() && db_dir.is_dir();
    }
    false
}

/// Validate DB name: must be same safe rules as slug but allow underscore.
fn validate_db_name(db: &str) -> Result<String, String> {
    let t = db.trim();
    if t.is_empty() {
        return Err("Nama DB kosong".to_string());
    }
    if t.len() > 64 {
        return Err("Nama DB max 64 karakter".to_string());
    }
    // allow a-z 0-9 _ and $ but for V1 restrict to lower alphanumeric underscore
    if !t.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
        return Err("Nama DB cuma huruf kecil, angka, underscore".to_string());
    }
    // cannot start with digit? MySQL allows but warn; we allow.
    Ok(t.to_string())
}

/// Execute mysql query via mysql.exe client.
/// Returns Ok(()) if success, Err Indonesian message otherwise.
pub fn exec_mysql_query(app_root: &Path, sql: &str, port: u16) -> Result<(), String> {
    let client_bin = resolve_mysql_client_bin(app_root)
        .ok_or_else(|| {
            // No client binary found → try fs? But return message as per spec that mysql client not found.
            "mysql.exe tidak ketemu 😅".to_string()
        })?;

    // Quick TCP check if MySQL port open — if closed, MySQL likely OFF
    if std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_err() {
        return Err(
            "MySQL belum ON, DB belum dibuat – Start MySQL dulu lalu klik [Create DB]".to_string(),
        );
    }

    let output = Command::new(&client_bin)
        .arg("-u")
        .arg("root")
        .arg("-h")
        .arg("127.0.0.1")
        .arg("-P")
        .arg(port.to_string())
        .arg("-e")
        .arg(sql)
        .output()
        .map_err(|e| format!("Gagal jalankan mysql client: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let combined = format!("{} {}", stdout, stderr).to_lowercase();
        if combined.contains("can't connect") || combined.contains("10061") || combined.contains("2003") {
            return Err(
                "MySQL belum ON, DB belum dibuat – Start MySQL dulu lalu klik [Create DB]".to_string(),
            );
        }
        Err(format!("Gagal buat DB: {}", stderr.trim()))
    }
}

/// Create database IF NOT EXISTS.
/// Validates db_name, then exec.
pub fn create_database_fs(app_root: &Path, db_name: &str, mysql_port: u16) -> Result<(), String> {
    let db = validate_db_name(db_name)?;
    let sql = format!(
        "CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_general_ci;",
        db
    );
    exec_mysql_query(app_root, &sql, mysql_port)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmp_root(label: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "vanompp_db_{}_{}_{}",
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
    fn test_validate_db_name_ok() {
        assert_eq!(validate_db_name("my_db").unwrap(), "my_db");
        assert_eq!(validate_db_name("toko_buku123").unwrap(), "toko_buku123");
    }

    #[test]
    fn test_validate_db_name_fail() {
        assert!(validate_db_name("").is_err());
        assert!(validate_db_name("MyDB").is_err()); // uppercase
        assert!(validate_db_name("my-db").is_err()); // dash not allowed
    }

    #[test]
    fn test_check_db_exists_false_when_no_data_dir() {
        let root = tmp_root("nofolder");
        let www = root.join("www");
        fs::create_dir_all(&www).unwrap();
        assert!(!check_db_exists_fs(&www, "mydb"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_check_db_exists_true_when_folder_present() {
        let root = tmp_root("exists");
        let www = root.join("www");
        let data = root.join("bin").join("mysql").join("data").join("mydb");
        fs::create_dir_all(&www).unwrap();
        fs::create_dir_all(&data).unwrap();
        assert!(check_db_exists_fs(&www, "mydb"));
        assert!(!check_db_exists_fs(&www, "other"));
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_resolve_app_root() {
        let p = PathBuf::from("C:/Vanompp/www");
        assert_eq!(resolve_app_root_from_www(&p), PathBuf::from("C:/Vanompp"));
    }

    #[test]
    fn test_exec_mysql_fails_when_no_client() {
        let root = tmp_root("no_client");
        let err = exec_mysql_query(&root, "SELECT 1", 3306).unwrap_err();
        assert!(!err.is_empty());
        let _ = fs::remove_dir_all(&root);
    }
}
