use multimizer::{db::Database, models::CodonUsage};

use anyhow::Result;

use crate::utils::get_database_file_path;

pub fn pull_codon_usage_for_org(org_id: i32) -> Result<CodonUsage> {
    let db_path = get_database_file_path()?;
    let db = Database::new(db_path)?;
    let codon_usage = db.get_codon_usage_for_organism(&org_id)?;

    Ok(codon_usage)
}
