use std::path::{Path, PathBuf};

pub fn get_app_root() -> PathBuf {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            return parent.to_path_buf();
        }
    }
    PathBuf::from(".")
}

pub fn ensure_www(root: &Path) -> std::io::Result<PathBuf> {
    let www = root.join("www");
    let health_dir = www.join("__vano_health");

    std::fs::create_dir_all(&health_dir)?;

    let index_php = health_dir.join("index.php");
    if !index_php.exists() {
        std::fs::write(&index_php, "<?php echo \"ok\";")?;
    }

    let gitkeep = www.join(".gitkeep");
    if !gitkeep.exists() {
        std::fs::write(&gitkeep, "")?;
    }

    Ok(www)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_get_app_root_not_empty() {
        let root = get_app_root();
        assert!(!root.as_os_str().is_empty());
    }

    #[test]
    fn test_ensure_www_creates_structure() {
        let tmp = std::env::temp_dir().join(format!("vanompp_test_{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let www = ensure_www(&tmp).unwrap();
        assert!(www.exists());
        assert!(www.join("__vano_health").exists());
        assert!(www.join("__vano_health/index.php").exists());
        assert!(www.join(".gitkeep").exists());

        let content = fs::read_to_string(www.join("__vano_health/index.php")).unwrap();
        assert!(content.contains("ok"));

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_ensure_www_idempotent() {
        let tmp = std::env::temp_dir().join(format!("vanompp_test_idemp_{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        fs::create_dir_all(&tmp).unwrap();

        let www1 = ensure_www(&tmp).unwrap();
        let www2 = ensure_www(&tmp).unwrap();
        assert_eq!(www1, www2);

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn test_ensure_www_preserves_existing_index() {
        let tmp = std::env::temp_dir().join(format!("vanompp_test_preserve_{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        let health = tmp.join("www/__vano_health");
        fs::create_dir_all(&health).unwrap();
        let custom = "<?php echo \"custom\";";
        fs::write(health.join("index.php"), custom).unwrap();

        let www = ensure_www(&tmp).unwrap();
        let content = fs::read_to_string(www.join("__vano_health/index.php")).unwrap();
        assert_eq!(content, custom, "should not overwrite existing index.php");

        let _ = fs::remove_dir_all(&tmp);
    }
}
