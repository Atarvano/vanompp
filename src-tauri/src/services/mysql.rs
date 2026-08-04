use std::path::PathBuf;
use std::time::Duration;

use crate::conf::{render_myini, root_forward};
use crate::services::{is_pid_alive, kill_pid, read_tail, resolve_mysql_bin, ServiceState};
use crate::utils::port::{is_port_free, suggest_next_free};

fn ensure_mysql_dirs(data_dir: &PathBuf, logs_tmp: &PathBuf) -> Result<(), String> {
    std::fs::create_dir_all(data_dir).map_err(|e| format!("Gagal bikin data dir mysql: {}", e))?;
    std::fs::create_dir_all(logs_tmp).map_err(|e| format!("Gagal bikin tmp mysql: {}", e))?;
    Ok(())
}

/// Render my.ini to appropriate location, init data dir if needed
pub fn render_myini_file(root: &PathBuf, mysql_port: u16) -> Result<PathBuf, String> {
    // Resolve actual mysql bin location first, so ROOT in template can be
    // derived from the real location (fixes dev path .../target/debug/resources/bin/mysql
    // vs prod C:/Vanompp/bin/mysql — previously always used D:/Vanompp which broke secure-file-priv).
    let mysqld_path_opt = resolve_mysql_bin(root).ok();

    // tmp dir must NOT be inside data/ — otherwise mysqld --initialize fails (data must be empty)
    // and runtime init error "Cant get stat of data/tmp" when my.ini points inside data but folder missing.
    let (my_ini_path, data_dir, tmp_dir, fwd): (PathBuf, PathBuf, PathBuf, String) =
        if let Some(exe_path) = mysqld_path_opt.as_ref() {
            let bin_dir = exe_path.parent().unwrap().to_path_buf();
            let mysql_root = bin_dir.parent().unwrap().to_path_buf(); // .../bin/mysql
            let data = mysql_root.join("data");
            let tmp = mysql_root.join("tmp"); // sibling, not data/tmp!
            let ini = mysql_root.join("my.ini");
            let root_for_template = mysql_root
                .parent()
                .and_then(|p| p.parent())
                .unwrap_or(&mysql_root)
                .to_path_buf();
            let f = root_forward(&root_for_template);
            (ini, data, tmp, f)
        } else {
            let mysql_root = root.join("bin").join("mysql");
            let f = root_forward(root);
            (
                mysql_root.join("my.ini"),
                mysql_root.join("data"),
                mysql_root.join("tmp"),
                f,
            )
        };

    let rendered = render_myini(&fwd, mysql_port);

    // Only ensure data dir exists (no tmp yet!) — tmp inside data makes
    // mysqld --initialize-insecure fail with "data directory has files"
    std::fs::create_dir_all(&data_dir).map_err(|e| format!("Gagal bikin data dir mysql: {}", e))?;

    std::fs::write(&my_ini_path, rendered).map_err(|e| format!("Gagal tulis my.ini: {}", e))?;

    Ok(my_ini_path)
}

/// Check if data dir is initialized — must have mysql system schema.
/// Old heuristic that returned true for any file (e.g. tmp/) caused
/// --initialize-insecure to be skipped or to abort, looping init fail.
pub fn is_data_initialized(data_dir: &PathBuf) -> bool {
    if !data_dir.exists() {
        return false;
    }
    data_dir.join("mysql").exists() && data_dir.join("mysql").is_dir()
        || data_dir.join("ibdata1").exists()
}

fn is_dir_empty_ignoring_tmp(data_dir: &PathBuf) -> bool {
    let Ok(rd) = std::fs::read_dir(data_dir) else {
        return true;
    };
    for entry in rd.flatten() {
        let name = entry.file_name().to_string_lossy().to_lowercase();
        // ignore our own helper dirs/files
        if name == "tmp" || name == "logs" || name == "mysql_error.log" || name == "mysql.pid" {
            continue;
        }
        // any other file means not empty
        return false;
    }
    true
}

fn run_initialize_insecure(mysqld_path: &PathBuf, data_dir: &PathBuf, mysql_root: &PathBuf) -> Result<(), String> {
    // If already initialized with mysql schema, skip
    if is_data_initialized(data_dir) {
        return Ok(());
    }

    // If data dir contains only tmp/logs from previous failed init,
    // clean it so --initialize-insecure can succeed (MYSQL errors if dir not empty).
    if data_dir.exists() && is_dir_empty_ignoring_tmp(data_dir) {
        // remove stray tmp/logs/error.log/pid produced by earlier ensure_dirs
        if let Ok(rd) = std::fs::read_dir(data_dir) {
            for e in rd.flatten() {
                let p = e.path();
                let n = e.file_name().to_string_lossy().to_lowercase();
                if n == "tmp" || n == "logs" || n == "mysql_error.log" || n == "mysql.pid" {
                    let _ = if p.is_dir() {
                        std::fs::remove_dir_all(&p)
                    } else {
                        std::fs::remove_file(&p)
                    };
                }
            }
        }
        // after cleaning helper files, dir should be truly empty — required by mysqld --initialize
        // remove dir itself to let mysqld create fresh, or leave empty
        if is_dir_empty_ignoring_tmp(data_dir) {
            // recreate empty data dir (mysqld wants empty dir, not necessarily non-existing)
            let _ = std::fs::create_dir_all(data_dir);
        }
    } else if data_dir.exists() && !is_dir_empty_ignoring_tmp(data_dir) {
        // data dir has real files but no mysql schema — corrupt. Bail with helpful msg.
        return Err(format!(
            "Data MySQL corrupt/unusable 😅 Folder {} ada file tapi bukan DB valid. Hapus isi data/ (backup dulu) lalu Start lagi biar auto-init — atau pakai [Repair].",
            data_dir.display()
        ));
    }

    // Ensure data empty + ensure tmp sibling exists BEFORE init (my.ini tmpdir points to it)
    let _ = std::fs::create_dir_all(data_dir);
    let _ = std::fs::create_dir_all(mysql_root.join("tmp"));

    #[allow(unused_mut)]
    let mut cmd = std::process::Command::new(mysqld_path);
    cmd.arg("--initialize-insecure")
        .arg(format!("--datadir={}", data_dir.display()))
        .arg(format!("--basedir={}", mysql_root.display()));

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    let output = cmd.output().map_err(|e| {
        format!(
            "Gagal init data MySQL 😅 {} — cek permission folder data",
            e
        )
    })?;

    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        // If data already initialized, mysql returns error but folder exists — treat as ok if mysql folder exists after attempt
        if is_data_initialized(data_dir) && data_dir.join("mysql").exists() {
            return Ok(());
        }
        return Err(format!(
            "Gagal init data MySQL 😅\nstdout: {}\nstderr: {}",
            stdout.chars().take(500).collect::<String>(),
            stderr.chars().take(1000).collect::<String>()
        ));
    }
    Ok(())
}

pub fn start_mysql(state: &ServiceState, root: &PathBuf, port: u16) -> Result<u32, String> {
    if let Ok(map) = state.childs.lock() {
        if let Some(pid) = map.get("mysql") {
            if is_pid_alive(*pid) {
                return Ok(*pid);
            }
        }
    }

    if !is_port_free(port) {
        let sugg = suggest_next_free(port + 1);
        return Err(format!("Port {} udah dipakai 😅 Coba {}", port, sugg));
    }

    let my_ini_path = render_myini_file(root, port)?;
    let mysqld_path = resolve_mysql_bin(root)?;

    let mysql_root = mysqld_path
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let data_dir = mysql_root.join("data");

    // Initialize if empty (needs tmp sibling existing — handled inside run_initialize)
    if !is_data_initialized(&data_dir) {
        if let Err(e) = run_initialize_insecure(&mysqld_path, &data_dir, &mysql_root) {
            return Err(e);
        }
    }

    // After init, ensure tmp sibling required by my.ini template (tmpdir=.../bin/mysql/tmp)
    let _ = std::fs::create_dir_all(mysql_root.join("tmp"));
    // also ensure data dir still exists (for log file)
    let _ = std::fs::create_dir_all(&data_dir);

    // error log path from template
    let error_log_path = data_dir.join("mysql_error.log");

    #[allow(unused_mut)]
    let mut cmd = std::process::Command::new(&mysqld_path);
    // Use defaults-file - this is important for portable
    cmd.arg(format!("--defaults-file={}", my_ini_path.display()));

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x08000000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }

    cmd.current_dir(&mysql_root);

    let child = cmd.spawn().map_err(|e| {
        format!(
            "Gagal start MySQL 😅 {} — cek apakah data folder corrupt? coba hapus data/* kecuali my.ini lalu restart",
            e
        )
    })?;

    let pid = child.id();

    if let Ok(mut map) = state.childs.lock() {
        map.insert("mysql".to_string(), pid);
    }

    // Poll
    let mut started = false;
    for _ in 0..20 {
        std::thread::sleep(Duration::from_millis(500));

        if !is_pid_alive(pid) {
            let tail = read_tail(&error_log_path, 30);
            if let Ok(mut map) = state.childs.lock() {
                map.remove("mysql");
            }
            if tail.is_empty() {
                return Err("MySQL gagal start 😅 Cek mysql_error.log — mysqld keluar".to_string());
            } else {
                return Err(format!("MySQL gagal start 😅 Cek mysql_error.log:\n{}", tail));
            }
        }

        if !is_port_free(port) {
            started = true;
            break;
        }

        if std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok() {
            started = true;
            break;
        }
    }

    if !started {
        let tail = read_tail(&error_log_path, 30);
        let _ = kill_pid(pid);
        if let Ok(mut map) = state.childs.lock() {
            map.remove("mysql");
        }
        if tail.is_empty() {
            return Err(format!(
                "MySQL gagal start 😅 Port {} masih free setelah 10 detik",
                port
            ));
        } else {
            return Err(format!("MySQL gagal start 😅 Cek mysql_error.log:\n{}", tail));
        }
    }

    Ok(pid)
}

pub fn stop_mysql(state: &ServiceState) -> Result<(), String> {
    let pid_opt = if let Ok(map) = state.childs.lock() {
        map.get("mysql").cloned()
    } else {
        None
    };

    if let Some(pid) = pid_opt {
        let _ = kill_pid(pid);
        #[cfg(target_os = "windows")]
        {
            let _ = std::process::Command::new("taskkill")
                .arg("/PID")
                .arg(pid.to_string())
                .arg("/F")
                .arg("/T")
                .output();
        }
        for _ in 0..10 {
            if !is_pid_alive(pid) {
                break;
            }
            std::thread::sleep(Duration::from_millis(300));
        }
        if let Ok(mut map) = state.childs.lock() {
            map.remove("mysql");
        }
        if is_pid_alive(pid) {
            return Err(format!("Gagal stop MySQL pid {} — coba taskkill manual", pid));
        }
        Ok(())
    } else {
        Ok(())
    }
}
