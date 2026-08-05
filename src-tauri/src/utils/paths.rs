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
    // ponytail: __vano_health removed from release — debugging trace shouldn't ship to SMK laptops
    // ensure only www/ + .gitkeep, no health endpoint in prod
    std::fs::create_dir_all(&www)?;

    let gitkeep = www.join(".gitkeep");
    if !gitkeep.exists() {
        std::fs::write(&gitkeep, "")?;
    }

    // cleanup leftover health from old installs — silent, best-effort
    let _ = std::fs::remove_dir_all(www.join("__vano_health"));

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
        assert!(www.join(".gitkeep").exists());
        assert!(!www.join("__vano_health").exists(), "health removed in release");

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
    fn test_ensure_www_cleans_old_health() {
        let tmp = std::env::temp_dir().join(format!("vanompp_test_clean_{}", std::process::id()));
        let _ = fs::remove_dir_all(&tmp);
        let health = tmp.join("www/__vano_health");
        fs::create_dir_all(&health).unwrap();
        fs::write(health.join("index.php"), "<?php echo \"old\";").unwrap();

        let www = ensure_www(&tmp).unwrap();
        assert!(!www.join("__vano_health").exists(), "old health cleaned");

        let _ = fs::remove_dir_all(&tmp);
    }
}
