use serde::{Serialize, Deserialize};
use crate::mach;
use crate::revid;
use crate::rev;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub root_path: String,
    pub tracked_files: Vec<String>,
    pub all_revs: Vec<String>,
    pub cur_rev: String,
}

impl Repo {
    pub fn print_repo(&self) {
        println!("Repo info");
        println!("  Root @ {}", self.root_path);
        println!("  Current: {}", self.cur_rev);
        for l in &self.all_revs {
            println!("  Revision: {}", l);
        }
        for l in &self.tracked_files {
            println!("  Tracked file: {}", l);
        }
    }
    
    pub fn save(&self) {
        let p = mach::join_paths(&self.root_path, &String::from(".arc"));
        
        let serialized = serde_json::to_string(self).unwrap();
        mach::write_string(&p, &String::from("repo.json"), &serialized);
    }
    
    pub fn open_rev(&self, rev_id: &String) -> rev::Rev {
        rev::Rev {
            rev_id: rev_id.clone(),
            parent_trunk: String::from("None"),
            parent_other: String::from("None"),
            files: Vec::new()
        }
    }
    
    pub fn new_rev(&mut self, trunk_id: &String, other_id: &String) -> rev::Rev {
        let r = rev::Rev {
            rev_id: revid::gen_rev_id(),
            parent_trunk: trunk_id.clone(),
            parent_other: other_id.clone(),
            files: Vec::new()
        };
        
        let p = mach::join_paths(&self.root_path, &String::from(".arc"));
        let p2 = mach::join_paths(&p, &r.rev_id);
        mach::create_dir(&p, &r.rev_id);
        
        self.all_revs.push(r.rev_id.clone());
        r
    }
    
    pub fn add(&mut self, rel_path: &String) {
        if !self.tracked_files.contains(rel_path) {
            self.tracked_files.push(rel_path.clone())
        }
    }
    
    pub fn remove(&mut self, rel_path: &String) {
        self.tracked_files.retain(|x| x != rel_path);
    }
    
    pub fn commit(&mut self) {
        let mut r = self.new_rev(&self.cur_rev.clone(), &String::from("None"));
        r.files = self.tracked_files.clone();
        
        let p = mach::join_paths(&self.root_path, &String::from(".arc"));
        let p2 = mach::join_paths(&p, &r.rev_id);
        mach::copy_files(&p2, &self.root_path, &r.files);
        
        r.save(&p2);
        self.cur_rev = r.rev_id.clone();
    }
    
    pub fn checkout(&mut self, rev_id: &String) {
        if self.all_revs.contains(rev_id) {
            let p = mach::join_paths(&self.root_path, &String::from(".arc"));
            let p2 = mach::join_paths(&p, rev_id);
            let r = rev::open_rev(&p2);
            
            mach::del_files(&self.root_path, &self.tracked_files);
            mach::del_files(&self.root_path, &r.files);
            mach::copy_files(&self.root_path, &p2, &r.files);
            
            self.cur_rev = rev_id.clone();
        }
    }
}

pub fn init_repo(root_path: &String) {
    println!("Init repo @ {}", root_path);
    let already_inited = mach::check_repo_dir(root_path);
    if already_inited {
        println!("Must be an empty repo!");
    } else {
        mach::create_dir(root_path, &String::from(".arc"));
        let p = mach::join_paths(root_path, &String::from(".arc"));
        
        let mut repo = Repo {
            root_path: root_path.clone(),
            tracked_files: Vec::new(),
            all_revs: Vec::new(),
            cur_rev: "None".to_string()
        };
        
        let rev = repo.new_rev(&"None".to_string(), &"None".to_string());
        rev.save(&mach::join_paths(&p, &rev.rev_id));
        
        repo.cur_rev = rev.rev_id.clone();
        repo.save();
    }
}

pub fn open_repo(root_path: &String) -> Repo {
    let p = mach::join_paths(root_path, &String::from(".arc"));
    // TODO: check if p exists
    
    let json = mach::read_line(&p, &String::from("repo.json"));
    let r: Repo = serde_json::from_str(&json).unwrap();
    r
}

