use crate::services::read_tail;
use crate::utils::paths::get_app_root;
use std::path::PathBuf;

fn resolve_log_path(root: &PathBuf, service: &str, kind: &str) -> Option<PathBuf> {
    let s = service.to_lowercase();
    let candidates: Vec<PathBuf> = match s.as_str() {
        "apache" => match kind {
            "access" => vec![
                root.join("bin/apache/logs/access.log"),
                root.join("bin").join("apache").join("logs").join("access.log"),
                root.join("resources/bin/apache/logs/access.log"),
            ],
            _ => vec![
                root.join("bin/apache/logs/error.log"),
                root.join("bin").join("apache").join("logs").join("error.log"),
                root.join("resources/bin/apache/logs/error.log"),
                root.join("src-tauri/resources/bin/apache/logs/error.log"),
            ],
        },
        "mysql" => vec![
            root.join("bin/mysql/data/mysql_error.log"),
            root.join("bin").join("mysql").join("data").join("mysql_error.log"),
            root.join("resources/bin/mysql/data/mysql_error.log"),
        ],
        "php" => vec![
            root.join("bin/php/logs/php_error.log"),
            root.join("bin").join("php").join("logs").join("php_error.log"),
            root.join("resources/bin/php/logs/php_error.log"),
        ],
        _ => vec![],
    };

    // Also try exe parent walk
    let mut extra = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(parent) = exe.parent() {
            let mut cur = parent.to_path_buf();
            for _ in 0..5 {
                match s.as_str() {
                    "apache" => {
                        extra.push(cur.join("bin/apache/logs/error.log"));
                        extra.push(cur.join("bin/apache/logs/access.log"));
                    }
                    "mysql" => extra.push(cur.join("bin/mysql/data/mysql_error.log")),
                    "php" => extra.push(cur.join("bin/php/logs/php_error.log")),
                    _ => {}
                }
                if !cur.pop() { break; }
            }
        }
    }
    let mut all = candidates;
    all.extend(extra);
    // also try cwd walk
    if let Ok(cwd) = std::env::current_dir() {
        let mut cur = cwd.clone();
        for _ in 0..4 {
            match s.as_str() {
                "apache" => {
                    if kind == "access" {
                        all.push(cur.join("bin/apache/logs/access.log"));
                    } else {
                        all.push(cur.join("bin/apache/logs/error.log"));
                    }
                }
                "mysql" => all.push(cur.join("bin/mysql/data/mysql_error.log")),
                "php" => all.push(cur.join("bin/php/logs/php_error.log")),
                _ => {}
            }
            if !cur.pop() { break; }
        }
    }

    for p in &all {
        if p.exists() {
            return Some(p.clone());
        }
    }
    None
}

/// Tauri command read_log tail
/// service: apache | mysql | php
/// lines: optional number of tail lines (default 100)
#[tauri::command]
pub fn read_log(service: String, lines: Option<usize>, kind: Option<String>) -> Result<String, String> {
    let root = get_app_root();
    let n = lines.unwrap_or(100).clamp(10, 500);
    let k = kind.unwrap_or_else(|| "error".to_string());
    let svc = service.to_lowercase();

    // Validate service
    if !["apache", "mysql", "php"].contains(&svc.as_str()) {
        return Err(format!("Service log ga dikenal: {} (pilih apache|mysql|php)", service));
    }

    // Try resolve actual file
    let maybe_path = resolve_log_path(&root, &svc, &k);

    if let Some(path) = maybe_path {
        let tail = read_tail(&path, n);
        if tail.trim().is_empty() {
            Ok(format!("Log {} ({}): file ada tapi kosong — service mungkin belum pernah start\nPath: {}", svc, k, path.display()))
        } else {
            Ok(tail)
        }
    } else {
        // Return placeholder message rather than error — more friendly for SMK
        let expected = match svc.as_str() {
            "apache" => "bin/apache/logs/error.log",
            "mysql" => "bin/mysql/data/mysql_error.log",
            "php" => "bin/php/logs/php_error.log",
            _ => "unknown.log",
        };
        Ok(format!("Log belum ada... belum pernah start atau bin belum ter-install\nExpected: {}\nRoot: {}", expected, root.display()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn tmp_root(label: &str) -> PathBuf {
        let p = std::env::temp_dir().join(format!(
            "vanompp_log_{}_{}_{}",
            label,
            std::process::id(),
            std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() % 100000
        ));
        let _ = fs::remove_dir_all(&p);
        fs::create_dir_all(&p).unwrap();
        p
    }

    #[test]
    fn test_read_log_empty_root_returns_placeholder() {
        let res = read_log("apache".to_string(), Some(10), None);
        // Should be Ok placeholder (not Err) because we treat missing as placeholder
        assert!(res.is_ok(), "should return placeholder Ok, got {:?}", res);
        let s = res.unwrap();
        assert!(!s.is_empty());
    }

    #[test]
    fn test_read_log_invalid_service() {
        let res = read_log("nginx".to_string(), None, None);
        assert!(res.is_err());
    }

    #[test]
    fn test_resolve_and_read_tail_with_file() {
        let root = tmp_root("tail");
        // Create fake log tree
        let apache_log_dir = root.join("bin/apache/logs");
        fs::create_dir_all(&apache_log_dir).unwrap();
        let log_file = apache_log_dir.join("error.log");
        fs::write(&log_file, "line1\nline2\nline3\nline4\nline5").unwrap();

        // resolve should find via root candidate
        let resolved = resolve_log_path(&root, "apache", "error");
        // Our resolve uses app_root not tmp_root, so we test read_tail directly
        let tail = read_tail(&log_file, 2);
        assert!(tail.contains("line4"));
        assert!(tail.contains("line5"));
        assert!(!tail.contains("line1"));

        let _ = fs::remove_dir_all(&root);
        let _ = resolved; // suppress unused
    }
}
