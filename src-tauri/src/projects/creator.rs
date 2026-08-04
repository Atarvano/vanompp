use std::path::Path;

use super::ProjectInfo;
use crate::services::db::{check_db_exists_fs, create_database_fs, resolve_app_root_from_www};
use crate::utils::slug::slugify;

const INDEX_TEMPLATE: &str = r#"<!doctype html>
<html><head><meta charset="utf-8"><title>{SLUG}</title></head>
<body>
<h1>{SLUG} jalan!</h1>
<p>Url kamu: http://localhost:8080/{SLUG}</p>
<?php include 'conn.php'; if(isset($conn) && $conn){ echo "<p style='color:green'>DB Connected ke {DB}</p>"; } else { echo "<p>DB belum konek / gak pake DB - cek conn.php</p>"; } ?>
</body></html>"#;

const CONN_TEMPLATE: &str = r#"<?php
// koneksi database - mysqli procedural (belajar)
$host = "localhost";
$user = "root";
$pass = "";
$db   = "{DB}";
$conn = mysqli_connect($host, $user, $pass, $db);
if(!$conn && $db !== ""){
  die("Koneksi gagal: ".mysqli_connect_error()." - cek MySQL ON? DB {DB} ada?");
}
"#;

const CONN_TEMPLATE_NO_DB: &str = r#"<?php
$host = "localhost";
$user = "root";
$pass = "";
$db   = ""; // belum buat DB, isi nanti kalo butuh
$conn = null; // gak konek DB karena db kosong
"#;

const GITIGNORE_TEMPLATE: &str = r#"# file yang gak perlu di-push ke github
/vendor/
.env
*.log
conn.php
.DS_Store
Thumbs.db
"#;

/// Pure FS logic for project creation.
/// Reuses existing utils::slug::slugify.
/// After writing files, attempts to CREATE DATABASE if create_db true.
/// If MySQL not running, keeps db_exists false — file creation still succeeds.
pub fn create_project_fs(
    name: &str,
    create_db: bool,
    db_name: &str,
    www_root: &Path,
    apache_port: u16,
) -> Result<ProjectInfo, String> {
    // slugify via existing util
    let slug = slugify(name)?;

    let proj_path = www_root.join(&slug);
    if proj_path.exists() {
        return Err(format!("Folder {} udah ada — mau buka yang ada?", slug));
    }

    std::fs::create_dir_all(&proj_path)
        .map_err(|e| format!("Gagal bikin folder {}: {}", slug, e))?;

    // Determine db_final per spec
    let db_final = if create_db {
        let trimmed = db_name.trim();
        if trimmed.is_empty() {
            slug.replace('-', "_")
        } else {
            trimmed.replace('-', "_").to_lowercase()
        }
    } else {
        String::new()
    };

    // Ensure www_root exists (idempotent)
    if !www_root.exists() {
        std::fs::create_dir_all(www_root).map_err(|e| format!("Gagal bikin www: {}", e))?;
    }

    let index_content = INDEX_TEMPLATE
        .replace("{SLUG}", &slug)
        .replace("{DB}", &db_final);

    let conn_content = if create_db {
        CONN_TEMPLATE.replace("{DB}", &db_final)
    } else {
        CONN_TEMPLATE_NO_DB.to_string()
    };

    std::fs::write(proj_path.join("index.php"), index_content)
        .map_err(|e| format!("Gagal tulis index.php: {}", e))?;
    std::fs::write(proj_path.join("conn.php"), conn_content)
        .map_err(|e| format!("Gagal tulis conn.php: {}", e))?;
    std::fs::write(proj_path.join(".gitignore"), GITIGNORE_TEMPLATE)
        .map_err(|e| format!("Gagal tulis .gitignore: {}", e))?;

    let url = format!("http://localhost:{}/{}", apache_port, slug);

    // Attempt to CREATE DATABASE if requested
    let mut db_exists = false;
    if create_db && !db_final.is_empty() {
        let app_root = resolve_app_root_from_www(www_root);
        // Use default mysql port 3306 for creation attempt — if running on different port,
        // user can still click [Create DB] button that uses actual port.
        let create_res = create_database_fs(&app_root, &db_final, 3306);
        if create_res.is_ok() {
            db_exists = true;
        } else {
            // Try fs check in case db folder already there (e.g., MySQL off but folder exists)
            db_exists = check_db_exists_fs(www_root, &db_final);
            // Don't fail entire creation — return Ok with db_exists false so frontend can show Create DB button
            // Logged to stderr for debugging
            eprintln!("[vano] create db attempt failed: {}", create_res.unwrap_err());
        }
    }

    Ok(ProjectInfo {
        name: slug.clone(),
        path: proj_path.to_string_lossy().to_string(),
        url,
        has_index: true,
        has_conn: true,
        has_gitignore: true,
        db_exists,
        db_name: db_final,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmp_root(label: &str) -> std::path::PathBuf {
        let p = std::env::temp_dir().join(format!(
            "vanompp_create_{}_{}_{}",
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
    fn test_create_basic_no_db() {
        let root = tmp_root("basic");
        let www = root.join("www");
        fs::create_dir_all(&www).unwrap();

        let info = create_project_fs("My Project", false, "", &www, 8080).unwrap();
        assert_eq!(info.name, "my-project");
        assert_eq!(info.url, "http://localhost:8080/my-project");
        assert!(www.join("my-project/index.php").exists());
        assert!(www.join("my-project/conn.php").exists());
        assert!(www.join("my-project/.gitignore").exists());
        // db not requested → false
        assert!(!info.db_exists);

        let conn = fs::read_to_string(www.join("my-project/conn.php")).unwrap();
        assert!(conn.contains("$db   = \"\""));

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_create_with_db_attempt() {
        let root = tmp_root("withdb");
        let www = root.join("www");
        fs::create_dir_all(&www).unwrap();

        // No MySQL bin nor running → db_exists should be false but creation succeeds
        let info = create_project_fs("Toko-Buku", true, "", &www, 8080).unwrap();
        assert_eq!(info.name, "toko-buku");
        let conn = fs::read_to_string(www.join("toko-buku/conn.php")).unwrap();
        assert!(conn.contains("toko_buku"));
        // db_exists will be false since no mysql data folder nor server
        assert!(!info.db_exists);

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_create_with_custom_db_name() {
        let root = tmp_root("dbcust");
        let www = root.join("www");
        fs::create_dir_all(&www).unwrap();

        let info = create_project_fs("MyProj", true, "My-DB-1", &www, 3000).unwrap();
        let conn = fs::read_to_string(www.join("myproj/conn.php")).unwrap();
        assert!(conn.contains("my_db_1"));
        assert_eq!(info.url, "http://localhost:3000/myproj");

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_create_duplicate_error() {
        let root = tmp_root("dup");
        let www = root.join("www");
        fs::create_dir_all(&www).unwrap();
        create_project_fs("dup-test", false, "", &www, 8080).unwrap();
        let err = create_project_fs("dup-test", false, "", &www, 8080).unwrap_err();
        assert!(err.contains("udah ada"));

        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_create_invalid_slug_error() {
        let root = tmp_root("invalid");
        let www = root.join("www");
        fs::create_dir_all(&www).unwrap();
        let err = create_project_fs("", false, "", &www, 8080).unwrap_err();
        // should be slug validation error Indonesian
        assert!(!err.is_empty());
        let _ = fs::remove_dir_all(&root);
    }

    #[test]
    fn test_templates_contain_required_markers() {
        assert!(INDEX_TEMPLATE.contains("{SLUG}"));
        assert!(INDEX_TEMPLATE.contains("{DB}"));
        assert!(CONN_TEMPLATE.contains("{DB}"));
        assert!(CONN_TEMPLATE.contains("mysqli_connect"));
        assert!(GITIGNORE_TEMPLATE.contains("/vendor/"));
        assert!(GITIGNORE_TEMPLATE.contains("conn.php"));
    }

    #[test]
    fn test_create_with_existing_data_folder_marks_exists() {
        let root = tmp_root("existcheck");
        let www = root.join("www");
        let data = root.join("bin").join("mysql").join("data").join("toko_buku");
        fs::create_dir_all(&www).unwrap();
        fs::create_dir_all(&data).unwrap();

        let info = create_project_fs("toko-buku", true, "", &www, 8080).unwrap();
        // Since data folder exists, db_exists true even if mysql client missing? Our impl tries client first then fs check.
        // If client missing, it falls back to fs check — should be true
        assert!(info.db_exists);

        let _ = fs::remove_dir_all(&root);
    }
}
