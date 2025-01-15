use std::path::Path;

use anyhow::Result;
use rusqlite::Connection;

use crate::models::{CodonUsage, Organism};

pub struct Database {
    conn: Connection,
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
    pub fn new<P>(db: P) -> Result<Database>
    where
        P: AsRef<Path>,
    {
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
    pub fn get_codon_usage_for_organism(&self, org_id: &i32) -> Result<CodonUsage> {
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
                Err(anyhow::anyhow!(msg))
            }
        }
    }

    /// Get the organism information for a particular organism ID
    ///
    /// # Arguments
    /// - `org_id` - The organism ID
    ///
    /// # Returns
    /// - `Organism` - The organism
    ///
    pub fn get_organism(&self, org_id: i32) -> Result<Organism> {
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
                Err(anyhow::anyhow!(msg))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use pretty_assertions::assert_eq;
    use rstest::{fixture, rstest};

    #[fixture]
    fn org_id() -> i32 {
        242
    }

    #[rstest]
    fn get_codon_usage(org_id: i32) {
        let db = Database::new("codon.db").unwrap();
        let usage = db.get_codon_usage_for_organism(&org_id).unwrap();

        let real_counts = vec![
            17199, 5873, 23827, 7444, 9137, 935, 2186, 3150, 29398, 6398, 14811, 12205, 19128,
            2254, 12914, 2590, 16094, 5377, 1112, 339, 6425, 2151, 7754, 4897, 27522, 8702, 38049,
            5074, 26980, 8136, 33697, 4394, 10005, 3613, 10002, 226, 8200, 773, 6128, 2234, 13743,
            3635, 10929, 368, 15623, 2225, 14369, 6128, 5324, 1707, 259, 3509, 2479, 383, 388, 145,
            8004, 3170, 9984, 2777, 14346, 2903, 16703, 2058,
        ];

        let codons = vec![
            "TTT", "TTC", "TTA", "TTG", "CTT", "CTC", "CTA", "CTG", "ATT", "ATC", "ATA", "ATG",
            "GTT", "GTC", "GTA", "GTG", "TAT", "TAC", "TAA", "TAG", "CAT", "CAC", "CAA", "CAG",
            "AAT", "AAC", "AAA", "AAG", "GAT", "GAC", "GAA", "GAG", "TCT", "TCC", "TCA", "TCG",
            "CCT", "CCC", "CCA", "CCG", "ACT", "ACC", "ACA", "ACG", "GCT", "GCC", "GCA", "GCG",
            "TGT", "TGC", "TGA", "TGG", "CGT", "CGC", "CGA", "CGG", "AGT", "AGC", "AGA", "AGG",
            "GGT", "GGC", "GGA", "GGG",
        ];

        let codons_with_count: Vec<(&str, i32)> = codons
            .iter()
            .zip(real_counts.iter())
            .map(|(&c, &r)| (c, r))
            .collect();

        for (codon, count) in codons_with_count {
            let pulled_codon_usage = usage.get(&codon.try_into().unwrap()).unwrap();
            assert_eq!(pulled_codon_usage, count);
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
