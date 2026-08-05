fn main() {
    // ponytail: dev watcher rebuilt on every mysql data change (ibdata1, binlog, my.ini) causing Apache pid overwritten loop
    // Whitelist only what should trigger rebuild
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-changed=Cargo.lock");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=tauri.conf.json");
    tauri_build::build()
}
