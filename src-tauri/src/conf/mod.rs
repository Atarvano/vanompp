use std::path::PathBuf;

pub const HTTPD_TEMPLATE: &str = include_str!("./httpd-vano.conf.template");
pub const MYINI_TEMPLATE: &str = include_str!("./my.ini.template");
pub const PHPINI_MINIMAL: &str = include_str!("./php.ini.minimal");

/// ROOT = bin parent, WWW_ROOT = actual www folder absolute
pub fn render_httpd(root_forward: &str, apache_port: u16, www_forward: &str) -> String {
    HTTPD_TEMPLATE
        .replace("{{ROOT}}", root_forward)
        .replace("{{WWW_ROOT}}", www_forward)
        .replace("{{APACHE_PORT}}", &apache_port.to_string())
}

pub fn render_myini(root_forward: &str, mysql_port: u16) -> String {
    MYINI_TEMPLATE
        .replace("{{ROOT}}", root_forward)
        .replace("{{MYSQL_PORT}}", &mysql_port.to_string())
}

pub fn render_phpini(root_forward: &str) -> String {
    PHPINI_MINIMAL.replace("{{ROOT}}", root_forward)
}

pub fn root_forward(root: &PathBuf) -> String {
    root.to_string_lossy().replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_render_httpd_contains_placeholders_replaced() {
        let out = render_httpd("C:/Vanompp", 8080, "C:/Vanompp/www");
        assert!(out.contains("C:/Vanompp"));
        assert!(out.contains("8080"));
        assert!(!out.contains("{{ROOT}}"));
        assert!(!out.contains("{{WWW_ROOT}}"));
        assert!(!out.contains("{{APACHE_PORT}}"));
        assert!(out.contains("php8apache2_4.dll"));
        assert!(out.contains("/phpmyadmin"));
    }
    #[test]
    fn test_render_myini() {
        let out = render_myini("D:/Vanompp", 3306);
        assert!(out.contains("D:/Vanompp"));
        assert!(out.contains("3306"));
        assert!(!out.contains("{{"));
    }
    #[test]
    fn test_render_phpini() {
        let out = render_phpini("D:/Vanompp");
        assert!(out.contains("D:/Vanompp"));
        assert!(out.contains("mysqli"));
        assert!(out.contains("Asia/Jakarta"));
    }
    #[test]
    fn test_root_forward() {
        let p = PathBuf::from("C:\\Users\\test\\Vanompp");
        let f = root_forward(&p);
        assert!(!f.contains('\\'));
        assert!(f.contains("C:/"));
    }
}
