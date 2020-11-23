use serde::{Serialize, Deserialize};
use crate::mach;

#[derive(Serialize, Deserialize, Debug)]
pub struct Rev {
    pub rev_id: String,
    
    pub parent_trunk: String,
    pub parent_other: String,
    
    pub files: Vec<String>,
}

impl Rev {
    pub fn save(&self, rev_path: &String) {
        let serialized = serde_json::to_string(self).unwrap();
        mach::write_string(&rev_path, &String::from("rev.json"), &serialized);
    }
}

pub fn open_rev(rev_path: &String) -> Rev {
    // TODO: check if rev_path exists
    
    let json = mach::read_line(&rev_path, &String::from("rev.json"));
    let r: Rev = serde_json::from_str(&json).unwrap();
    r
}

