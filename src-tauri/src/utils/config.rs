use std::path::{Path, PathBuf};
use std::fs;
pub fn get_config_path(root: &Path) -> PathBuf { root.join("vanompp.toml") }
fn parse_ports(content: &str) -> (Option<u16>, Option<u16>) {
    let mut apache=None; let mut mysql=None;
    for line in content.lines() {
        let l=line.trim(); if l.is_empty()||l.starts_with('#'){continue;}
        if let Some((k,v))=l.split_once('='){
            let k=k.trim().to_lowercase(); let v=v.trim();
            if let Ok(p)=v.parse::<u16>(){
                if k=="apache_port"||k=="apache"{apache=Some(p);}
                if k=="mysql_port"||k=="mysql"{mysql=Some(p);}
            }
        }
    }
    (apache,mysql)
}
pub fn read_persisted_ports(root: &Path)->(Option<u16>,Option<u16>){
    let path=get_config_path(root);
    match fs::read_to_string(&path){Ok(c)=>parse_ports(&c),Err(_)=>(None,None)}
}
pub fn read_persisted_ports_effective(root: &Path, def_a: u16, def_m: u16)->(u16,u16){
    let (a,m)=read_persisted_ports(root);
    (a.unwrap_or(def_a), m.unwrap_or(def_m))
}
fn atomic_write(path: &Path, content: String)->Result<(),String>{
    // ponytail: unique tmp per write, avoid parallel test race same pid
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().subsec_nanos();
    let tmp = path.with_extension(format!("toml.tmp.{}.{}", std::process::id(), nanos));
    fs::write(&tmp, content).map_err(|e| e.to_string())?;
    fs::rename(&tmp, path).map_err(|e| e.to_string())?;
    Ok(())
}
pub fn write_persisted_port(root: &Path, name: &str, port: u16)->Result<(),String>{
    let path=get_config_path(root);
    let content=fs::read_to_string(&path).unwrap_or_default();
    let key=if name.to_lowercase().contains("apache"){"apache_port"}else{"mysql_port"};
    let mut lines:Vec<String>=Vec::new(); let mut found=false;
    for line in content.lines(){
        let t=line.trim(); if t.is_empty()||t.starts_with('#'){lines.push(line.to_string()); continue;}
        if let Some((k,_))=t.split_once('='){
            let kk=k.trim().to_lowercase();
            if (kk=="apache_port"||kk=="apache")&&key=="apache_port" {lines.push(format!("{}={}",key,port)); found=true; continue;}
            if (kk=="mysql_port"||kk=="mysql")&&key=="mysql_port" {lines.push(format!("{}={}",key,port)); found=true; continue;}
        }
        lines.push(line.to_string());
    }
    if !found {lines.push(format!("{}={}",key,port));}
    let new_content=lines.join("\n")+"\n";
    if let Some(p)=path.parent(){ let _=fs::create_dir_all(p); }
    atomic_write(&path, new_content)
}
pub fn reset_persisted_port(root: &Path, name: &str)->Result<(),String>{
    let path=get_config_path(root);
    let content=fs::read_to_string(&path).unwrap_or_default();
    let key=if name.to_lowercase().contains("apache"){"apache_port"}else{"mysql_port"};
    let mut lines:Vec<String>=Vec::new();
    for line in content.lines(){
        let t=line.trim(); if t.is_empty()||t.starts_with('#'){lines.push(line.to_string()); continue;}
        if let Some((k,_))=t.split_once('='){
            let kk=k.trim().to_lowercase();
            if (kk=="apache_port"||kk=="apache")&&key=="apache_port"{continue;}
            if (kk=="mysql_port"||kk=="mysql")&&key=="mysql_port"{continue;}
        }
        lines.push(line.to_string());
    }
    if lines.is_empty(){ let _=fs::remove_file(&path); return Ok(());}
    atomic_write(&path, lines.join("\n")+"\n")
}
#[cfg(test)]
mod tests {
    use super::*;
    fn unique_tmp(prefix: &str)->PathBuf{
        let n = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        std::env::temp_dir().join(format!("{}_{}_{}", prefix, std::process::id(), n))
    }
    #[test] fn test_config_write_read_roundtrip(){
        let tmp=unique_tmp("vanompp_test");
        std::fs::create_dir_all(&tmp).unwrap();
        write_persisted_port(&tmp,"mysql",3309).unwrap();
        let (ap,mp)=read_persisted_ports(&tmp);
        assert_eq!(mp,Some(3309)); assert_eq!(ap,None);
        std::fs::remove_dir_all(&tmp).ok();
    }
    #[test] fn test_config_corrupt_fallback(){
        let tmp=unique_tmp("vanompp_corrupt");
        std::fs::create_dir_all(&tmp).unwrap();
        std::fs::write(tmp.join("vanompp.toml"), "garbage!!!\napache_port=not_number").unwrap();
        let (ap,mp)=read_persisted_ports(&tmp);
        assert_eq!(ap,None); assert_eq!(mp,None);
        std::fs::remove_dir_all(&tmp).ok();
    }
    #[test] fn test_config_reset(){
        let tmp=unique_tmp("vanompp_reset");
        std::fs::create_dir_all(&tmp).unwrap();
        write_persisted_port(&tmp,"mysql",3309).unwrap();
        reset_persisted_port(&tmp,"mysql").unwrap();
        let (_ap,mp)=read_persisted_ports(&tmp);
        assert_eq!(mp,None);
        std::fs::remove_dir_all(&tmp).ok();
    }
}
