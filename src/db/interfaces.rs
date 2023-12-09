use rusqlite::{Connection, Result};

use crate::models::{CodonUsage, Organism};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    ///
    /// Create a new database connection
    ///
    /// # Arguments
    /// - `db` - The path to the database
    ///
    /// # Returns
    /// - `Database` - The database connection
    ///
    pub fn new(db: &str) -> Result<Database, Box<dyn std::error::Error>> {
        let conn = Connection::open(db)?;

        Ok(Database { conn })
    }

    /// Get the codon usage for a particular organism and amino acid
    ///
    /// # Arguments
    /// - `org_id` - The organism ID
    /// - `aa` - The amino acid
    ///
    /// # Returns
    /// - `CodonUsage` - The codon usage for the organism and amino acid
    pub fn get_codon_usage_for_organism(
        &self,
        org_id: &i32,
    ) -> Result<CodonUsage, Box<dyn std::error::Error>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM codon_usage WHERE org_id = ?")?;
        let mut rows = stmt.query([org_id])?;

        let res = rows.next()?;

        // TODO: this is incorrect and the columns dont match up with the model
        // we need to fix this
        match res {
            Some(row) => Ok(CodonUsage::new(
                // row.get(0)?, skip org_id
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
                row.get(7)?,
                row.get(8)?,
                row.get(9)?,
                row.get(10)?,
                row.get(11)?,
                row.get(12)?,
                row.get(13)?,
                row.get(14)?,
                row.get(15)?,
                row.get(16)?,
                row.get(17)?,
                row.get(18)?,
                row.get(19)?,
                row.get(20)?,
                row.get(21)?,
                row.get(22)?,
                row.get(23)?,
                row.get(24)?,
                row.get(25)?,
                row.get(26)?,
                row.get(27)?,
                row.get(28)?,
                row.get(29)?,
                row.get(30)?,
                row.get(31)?,
                row.get(32)?,
                row.get(33)?,
                row.get(34)?,
                row.get(35)?,
                row.get(36)?,
                row.get(37)?,
                row.get(38)?,
                row.get(39)?,
                row.get(40)?,
                row.get(41)?,
                row.get(42)?,
                row.get(43)?,
                row.get(44)?,
                row.get(45)?,
                row.get(46)?,
                row.get(47)?,
                row.get(48)?,
                row.get(49)?,
                row.get(50)?,
                row.get(51)?,
                row.get(52)?,
                row.get(53)?,
                row.get(54)?,
                row.get(55)?,
                row.get(56)?,
                row.get(57)?,
                row.get(58)?,
                row.get(59)?,
                row.get(60)?,
                row.get(61)?,
                row.get(62)?,
                row.get(63)?,
                row.get(64)?,
            )),
            None => {
                let msg = format!("No organism found at org_id: {org_id}");
                Err(msg.into())
            }
        }
    }

    /// Get the organism for a particular organism ID
    ///
    /// # Arguments
    /// - `org_id` - The organism ID
    ///
    /// # Returns
    /// - `Organism` - The organism
    ///
    pub fn get_organism(&self, org_id: i32) -> Result<Organism, Box<dyn std::error::Error>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM organisms WHERE org_id = ?")?;
        let mut rows = stmt.query([org_id])?;

        let res = rows.next()?;

        match res {
            Some(row) => Ok(Organism {
                org_id: row.get(0)?,
                division: row.get(1)?,
                assembly: row.get(2)?,
                taxid: row.get(3)?,
                species: row.get(4)?,
                organelle: row.get(5)?,
                translation_table: row.get(6)?,
                num_cds: row.get(7)?,
                num_codons: row.get(8)?,
                gc_perc: row.get(9)?,
                gc1_perc: row.get(10)?,
                gc2_perc: row.get(11)?,
                gc3_perc: row.get(12)?,
            }),
            None => {
                let msg = format!("No organism found at org_id: {org_id}");
                Err(msg.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::{fixture, rstest};

    #[fixture]
    fn org_id() -> i32 {
        242
    }

    #[rstest]
    fn get_codon_usage(org_id: i32) {
        let db = Database::new("codon.db").unwrap();
        let usage = db.get_codon_usage_for_organism(&org_id).unwrap();

        for (_, count) in usage {
            assert!(count > 0);
        }
    }

    #[rstest]
    fn get_org(org_id: i32) {
        let db = Database::new("codon.db").unwrap();
        let org = db.get_organism(org_id).unwrap();

        assert_eq!(org.org_id, org_id);
        assert_eq!(org.division, "refseq");
        assert_eq!(org.assembly, "GCF_000016525.1");
        assert_eq!(org.taxid, 420247);
        assert_eq!(org.species, "Methanobrevibacter smithii ATCC 35061");
        assert_eq!(org.organelle, "genomic");
        assert_eq!(org.translation_table, 11);
        assert_eq!(org.num_cds, 1710);
        assert_eq!(org.num_codons, 543072);
        assert_eq!(org.gc_perc, 32.14);
        assert_eq!(org.gc1_perc, 43.53);
        assert_eq!(org.gc2_perc, 32.58);
        assert_eq!(org.gc3_perc, 20.32);
    }
}
