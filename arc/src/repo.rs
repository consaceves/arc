use serde::{Serialize, Deserialize};
use std::fmt;
use crate::mach;
use crate::rev;
use crate::revid;
use crate::revid::RevID;
use crate::merge;

#[derive(Serialize, Deserialize, Debug)]
struct RepoInfo {
    upstream: String,
    root_path: String,
    tracked_files: Vec<String>,
    all_revs: Vec<RevID>,
    cur_rev: RevID,
}

pub struct Repo {
    pub root_path: String,
    pub arc_path: String,
    repo: RepoInfo,
}

impl fmt::Display for RepoInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Upstream @ {}\n", self.upstream)?;
        write!(f, "Root @ {}", self.root_path)?;
        write!(f, "Current Revision: {}", self.cur_rev)?;
        write!(f, "All Revisions:\n")?;
        for l in &self.all_revs {
            write!(f, "  {}\n", l.to_string())?;
        }
        write!(f, "Tracked files:\n")?;
        for l in &self.tracked_files {
            write!(f, "  {}\n", l)?;
        }
        Ok(())
    }
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root Path @ {}\n", self.root_path)?;
        write!(f, ".arc Path @ {}\n", self.arc_path)?;
        write!(f, "{}", self.repo)?;
        Ok(())
    }
}

impl Repo {
    pub fn set_upstream(&mut self, ups: &String) {
        self.repo.upstream = ups.to_string();
    }
    
    pub fn get_upstream(&self) -> &String {
        &self.repo.upstream
    }
    
    pub fn save(&self) {
        let serialized = serde_json::to_string(&self.repo).unwrap();
        mach::write_string(&self.arc_path, &String::from("repo.json"), &serialized);
    }
    
    pub fn set_head_rev(&mut self, rev_id: &RevID) {
        self.repo.cur_rev = rev_id.clone();
    }
    
    pub fn get_head_rev_str(&self) -> String {
        self.repo.cur_rev.to_string()
    }
    
    pub fn add_rev(&mut self, rev_id: &RevID) {
        self.repo.all_revs.push(rev_id.clone());
    }
    
    pub fn contains_rev(&self, rev_id: &RevID) -> bool {
        self.repo.all_revs.contains(&rev_id)
    }
    
    /*
     * Add/Remove tracked files
     */
    pub fn add_file(&mut self, rel_path: &String) {
        let full_path = mach::join_paths(&self.root_path, rel_path);
        assert!(mach::check_path(&full_path), "File doesn't exist!");
        
        if !self.repo.tracked_files.contains(rel_path) {
            self.repo.tracked_files.push(rel_path.clone());
        }
        
        println!("Added to tracked files @ {}", rel_path);
    }
    
    pub fn remove_file(&mut self, rel_path: &String) {
        self.repo.tracked_files.retain(|x| x != rel_path);
        
        let full_path = mach::join_paths(&self.root_path, rel_path);
        if mach::check_path(&full_path) {
            mach::del_file(&self.root_path, rel_path);
        }
        
        println!("Removed from tracked files @ {}", rel_path);
    }
    
    pub fn update_files(&mut self, files: &Vec<String>) {
        self.repo.tracked_files.clear();
        for f in files {
            self.repo.tracked_files.push(f.to_string());
        }
    }
    
    /*
     * Commit/Checkout
     */
    pub fn commit(&mut self) -> rev::Rev {
        let mut rev = rev::new(self, &self.repo.cur_rev, &revid::EMPTY);
        
        rev.commit(&self.repo.tracked_files);
        rev.save();
        
        self.add_rev(rev.get_id());
        self.set_head_rev(rev.get_id());
        self.save();
        
        println!("Committed -> {}", rev.get_id());
        rev
    }
    
    pub fn checkout(&mut self, rev_id_str: &String) -> rev::Rev {
        let rev_id = revid::parse(&rev_id_str);
        assert!(self.contains_rev(&rev_id), "Invalid revision!");
        
        mach::del_files(&self.root_path, &self.repo.tracked_files);
        let rev = rev::open(self, &rev_id);
        rev.checkout();
        
        self.update_files(rev.get_files());
        self.set_head_rev(rev.get_id());
        self.save();
        
        println!("Checked out {}", rev_id);
        rev
    }
    
    /*
     * Sync
     */
    pub fn sync(&mut self, other_repo: &Repo) {
        for other_rev_id in &other_repo.repo.all_revs {
            if !self.contains_rev(&other_rev_id) {
                let other_rev = rev::open(&other_repo, &other_rev_id);
                
                let dst_path = mach::join_paths(&self.arc_path, &other_rev_id.to_string());
                other_rev.copy_to(&dst_path);
                
                self.add_rev(&other_rev_id);
            }
        }
        self.save();
        println!("Synchronized {} with {}", self.root_path, other_repo.root_path);
    }
    
    /*
     * Merge
     */
    pub fn merge(&mut self, trunk_id_str: &String, other_id_str: &String) -> rev::Rev {
        let trunk_id = revid::parse(&trunk_id_str);
        let other_id = revid::parse(&other_id_str);
        assert!(self.contains_rev(&trunk_id), "Invalid rev id for trunk parent!");
        assert!(self.contains_rev(&other_id), "Invalid rev id for other parent!");
        
        if trunk_id == other_id {
            println!("Already update to date: {}", trunk_id);
            rev::open(self, &trunk_id)
        } else if merge::can_reach_rev(self, &other_id, &trunk_id) {
            println!("Fast-forward -> {}", other_id);
            rev::open(self, &other_id)
        } else if merge::can_reach_rev(self, &trunk_id, &other_id) {
            println!("Fast-forward -> {}", trunk_id);
            rev::open(self, &trunk_id)
        } else {
            let ancestor_id = merge::find_common_ancestor(self, &trunk_id, &other_id);
            assert!(!ancestor_id.is_empty(), "No common ancestor revision!");
            
            let ancestor_rev = rev::open(self, &ancestor_id);
            let trunk_rev = rev::open(self, &trunk_id);
            let other_rev = rev::open(self, &other_id);
            
            let mut rev = rev::new(self, &self.repo.cur_rev, &revid::EMPTY);
            rev.merge(&ancestor_rev, &trunk_rev, &other_rev);
            rev.save();
            
            self.add_rev(rev.get_id());
            self.save();
            
            println!("Merged {} and {} -> {}", trunk_id, other_id, rev.get_id());
            rev
        }
    }
}

pub fn init(root_path: &String) -> Repo {
    if !mach::check_path(root_path) {
        mach::create_dir_all(root_path);
    }
    
    let arc_path = mach::join_paths(root_path, &".arc".to_string());
    assert!(!mach::check_path(&arc_path), "Repo already initialized!");
    mach::create_dir_all(&arc_path);
    
    let repo = RepoInfo {
        upstream: "None".to_string(),
        root_path: root_path.clone(),
        tracked_files: Vec::new(),
        all_revs: Vec::new(),
        cur_rev: revid::EMPTY
    };
    
    let r = Repo {
        root_path: root_path.clone(),
        arc_path: arc_path.clone(),
        repo: repo
    };
    
    r.save();
    println!("Initialized repo @ {}", root_path);
    r
}

pub fn open(root_path: &String) -> Repo {
    let arc_path = mach::join_paths(root_path, &".arc".to_string());
    assert!(mach::check_path(&arc_path), "Repo doesn't exist!");
    
    let json = mach::read_line(&arc_path, &String::from("repo.json"));
    let repo: RepoInfo = serde_json::from_str(&json).expect("Unable to open repository, unexpected repo config file!");
    
    Repo {
        root_path: root_path.clone(),
        arc_path: arc_path.clone(),
        repo: repo
    }
}

