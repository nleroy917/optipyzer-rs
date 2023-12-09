use std::collections::HashMap;

use rusqlite::{Connection, Result};

use crate::models::CodonUsage;
use crate::consts::AACodonLibrary;

pub struct Database {
    pub conn: Connection
}

impl Database {
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
    pub fn get_codon_usage_for_organism(&self, org_id: i32) -> Result<CodonUsage, Box<dyn std::error::Error>> {

        let mut stmt = self.conn.prepare("SELECT * FROM codon_usage WHERE org_id = ?")?;
        let mut rows = stmt.query([org_id])?;
        
        let res = rows.next()?;

        match res {
            Some(row) => {
                Ok(CodonUsage {
                    org_id: row.get(0)?,
                    ttt: row.get(1)?,
                    ttc: row.get(2)?,
                    tta: row.get(3)?,
                    ttg: row.get(4)?,
                    ctt: row.get(5)?,
                    ctc: row.get(6)?,
                    cta: row.get(7)?,
                    ctg: row.get(8)?,
                    att: row.get(9)?,
                    atc: row.get(10)?,
                    ata: row.get(11)?,
                    atg: row.get(12)?,
                    gtt: row.get(13)?,
                    gtc: row.get(14)?,
                    gta: row.get(15)?,
                    gtg: row.get(16)?,
                    tat: row.get(17)?,
                    tac: row.get(18)?,
                    taa: row.get(19)?,
                    tag: row.get(20)?,
                    cat: row.get(21)?,
                    cac: row.get(22)?,
                    caa: row.get(23)?,
                    cag: row.get(24)?,
                    aat: row.get(25)?,
                    aac: row.get(26)?,
                    aaa: row.get(27)?,
                    aag: row.get(28)?,
                    gat: row.get(29)?,
                    gac: row.get(30)?,
                    gaa: row.get(31)?,
                    gag: row.get(32)?,
                    tct: row.get(33)?,
                    tcc: row.get(34)?,
                    tca: row.get(35)?,
                    tcg: row.get(36)?,
                    cct: row.get(37)?,
                    ccc: row.get(38)?,
                    cca: row.get(39)?,
                    ccg: row.get(40)?,
                    act: row.get(41)?,
                    acc: row.get(42)?,
                    aca: row.get(43)?,
                    acg: row.get(44)?,
                    gct: row.get(45)?,
                    gcc: row.get(46)?,
                    gca: row.get(47)?,
                    gcg: row.get(48)?,
                    tgt: row.get(49)?,
                    tgc: row.get(50)?,
                    tga: row.get(51)?,
                    tgg: row.get(52)?,
                    cgt: row.get(53)?,
                    cgc: row.get(54)?,
                    cga: row.get(55)?,
                    cgg: row.get(56)?,
                    agt: row.get(57)?,
                    agc: row.get(58)?,
                    aga: row.get(59)?,
                    agg: row.get(60)?,
                    ggt: row.get(61)?,
                    ggc: row.get(62)?,
                    gga: row.get(63)?,
                    ggg: row.get(64)?,
                })
                
            },
            None => {
                let msg = format!("No organism found at org_id: {org_id}");
                return Err(msg.into());
            }
        }
    }
}