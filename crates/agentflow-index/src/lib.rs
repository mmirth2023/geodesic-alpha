use sled::Db;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct GdpTracker {
    db: Db,
}

impl GdpTracker {
    pub fn new(path: &str) -> Self {
        let db = sled::open(path).expect("Failed to open sled DB");
        Self { db }
    }

    pub fn add_agent_value(&self, _key: &str, value: u64) {
        let current = self.get_total_gdp();
        let new_total = current + value;
        
        // Save overall total
        self.db.insert("total_gdp", &new_total.to_be_bytes()).unwrap();
        
        // Save timestamped entry for Alpha Velocity detection
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        self.db.insert(format!("hist_{}", now), &value.to_be_bytes()).unwrap();
    }

    pub fn get_total_gdp(&self) -> u64 {
        match self.db.get("total_gdp").unwrap() {
            Some(val) => {
                let bytes: [u8; 8] = val.as_ref().try_into().unwrap();
                u64::from_be_bytes(bytes)
            }
            None => 0,
        }
    }
}

