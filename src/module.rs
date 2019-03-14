use std::collections::HashMap;
use std::str::FromStr;
use std::fmt;

pub enum Severity {
    LOW,
    MODERATE,
    HIGH,
    CRITICAL
}

impl FromStr for Severity {
    type Err = &'static str;

    fn from_str(severity: &str) -> Result<Self, &'static str> {
        match severity {
            "low" => Ok(Severity::LOW),
            "moderate" => Ok(Severity::MODERATE),
            "high" => Ok(Severity::HIGH),
            "critical" => Ok(Severity::CRITICAL),
            _ => Err("Incorrect severity")
        }
    }
}

#[derive(Debug)]
pub struct ModuleStat {
    low_cnt: u16,
    moderate_cnt: u16,
    high_cnt: u16,
    critical_cnt: u16,
    directory_cnt: u16,
}

impl ModuleStat {
    //todo: store directories
    pub fn new()-> ModuleStat {
        ModuleStat { low_cnt: 0, moderate_cnt: 0, high_cnt: 0, critical_cnt: 0, directory_cnt: 1 }
    }

    pub fn add(&mut self, severity: Severity) {
        match severity {
            Severity::LOW => self.low_cnt += 1,
            Severity::MODERATE => self.moderate_cnt += 1,
            Severity::HIGH => self.high_cnt += 1,
            Severity::CRITICAL => self.critical_cnt += 1,
        };
    }

    // pub fn is_equal(&self, other: &ModuleStat)-> bool {
    //     self.low_cnt == other.low_cnt
    //     && self.moderate_cnt == other.moderate_cnt
    //     && self.high_cnt == other.high_cnt
    //     && self.critical_cnt == other.critical_cnt
    // }

    pub fn merge(&mut self, other: ModuleStat) {
        self.low_cnt += other.low_cnt;
        self.moderate_cnt += other.moderate_cnt;
        self.high_cnt += other.high_cnt;
        self.critical_cnt += other.critical_cnt;
        self.directory_cnt += other.directory_cnt;
    }
}

pub struct AuditResult {
    // todo: include package version into key
    map: HashMap<String, ModuleStat>,
}

impl AuditResult {
    pub fn new()-> AuditResult {
        AuditResult { map: HashMap::new() }
    }

    pub fn add_advisory(&mut self, module_name: &str, severity: Severity) {
        if let Some(existing_stat) = self.map.get_mut(module_name) {
            existing_stat.add(severity);
        } else {
            let mut new_stat = ModuleStat::new();
            new_stat.add(severity);

            self.map.insert(module_name.to_owned(), new_stat);
        }
    }

    pub fn merge(&mut self, other: AuditResult) {
        for (module_name, other_stat) in other.map {
            if let Some(existing_stat) = self.map.get_mut(&module_name) {
                // if !existing_stat.is_equal(&other_stat) {
                //     panic!(format!("The result of {} is different in some directories!", module_name));
                // }
                existing_stat.merge(other_stat);
            } else {
                self.map.insert(module_name, other_stat);
            }
        }
    }
}

impl fmt::Debug for AuditResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (module_name, stat) in &self.map {
            write!(f, "Module [ {} ]", module_name)?;
            dbg!(stat);
        }
        Ok(())
    }
}

impl fmt::Display for AuditResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (module_name, stat) in &self.map {
            write!(f, "Module [ {} ]", module_name)?;
            println!("{:?}", stat);
        }
        Ok(())
    }
}
