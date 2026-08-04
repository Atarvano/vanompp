pub mod creator;
pub mod scanner;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProjectInfo {
    pub name: String,
    pub path: String,
    pub url: String,
    pub has_index: bool,
    pub has_conn: bool,
    pub has_gitignore: bool,
    pub db_exists: bool,
    #[serde(default)]
    pub db_name: String,
}
