const DENY: &[&str] = &[
    "phpmyadmin", "mysql", "php", "__vano_health", "con", "prn", "aux", "nul", "com1", "com2", "lpt1",
];

pub fn slugify(input: &str) -> Result<String, String> {
    let trimmed = input.trim().to_lowercase();

    if trimmed.is_empty() {
        return Err("Nama tidak boleh kosong".to_string());
    }

    // Map: alphanumeric stays, space/_/- -> '-', else -> '-'
    let mut mapped = String::with_capacity(trimmed.len());
    for ch in trimmed.chars() {
        if ch.is_ascii_alphanumeric() {
            mapped.push(ch);
        } else if ch == ' ' || ch == '_' || ch == '-' {
            mapped.push('-');
        } else {
            mapped.push('-');
        }
    }

    // Collapse "--" loop
    let mut collapsed = String::with_capacity(mapped.len());
    let mut prev_dash = false;
    for ch in mapped.chars() {
        if ch == '-' {
            if !prev_dash {
                collapsed.push(ch);
            }
            prev_dash = true;
        } else {
            collapsed.push(ch);
            prev_dash = false;
        }
    }

    let s = collapsed.trim_matches('-').to_string();

    if s.is_empty() {
        return Err("Nama harus huruf/angka".to_string());
    }

    if s.len() > 32 {
        return Err("Max 32 karakter".to_string());
    }

    if !s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        return Err("Nama harus huruf/angka".to_string());
    }

    if DENY.contains(&s.as_str()) {
        return Err(format!("{} tidak boleh dipakai", s));
    }

    Ok(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_project() {
        assert_eq!(slugify("My Project").unwrap(), "my-project");
    }

    #[test]
    fn test_hello_world() {
        assert_eq!(slugify("hello_world").unwrap(), "hello-world");
    }

    #[test]
    fn test_trim_underscores() {
        assert_eq!(slugify("___test___").unwrap(), "test");
    }

    #[test]
    fn test_empty() {
        assert!(slugify("").is_err());
        assert!(slugify("   ").is_err());
    }

    #[test]
    fn test_deny_phpmyadmin() {
        let err = slugify("phpmyadmin").unwrap_err();
        assert!(err.contains("tidak boleh dipakai"));
    }

    #[test]
    fn test_long_string() {
        let long = "a".repeat(33);
        let err = slugify(&long).unwrap_err();
        assert!(err.contains("32"));
    }

    #[test]
    fn test_collapse_dashes() {
        assert_eq!(slugify("my--project").unwrap(), "my-project");
    }

    #[test]
    fn test_empty_after_sanitize() {
        let err = slugify("!!!").unwrap_err();
        assert!(err.contains("huruf/angka"));
    }

    #[test]
    fn test_deny_case_insensitive() {
        assert!(slugify("PHPMYADMIN").is_err());
    }

    #[test]
    fn test_exactly_32_chars() {
        let s = "a".repeat(32);
        assert!(slugify(&s).is_ok());
    }
}
