use std::env;
use std::path::PathBuf;

use anyhow::Result;

pub fn get_database_file_path() -> Result<PathBuf> {
    let possible_paths = vec![
        env::var("CODON_DB_PATH").ok().map(PathBuf::from),
        env::current_dir().ok().map(|dir| dir.join("codon.db")),
    ];

    for path in possible_paths {
        if let Some(path) = path {
            if path.exists() {
                return Ok(path);
            }
        }
    }

    anyhow::bail!(
        "Could not find the database file! Have you set the CODON_DB_PATH environment variable?"
    );
}
