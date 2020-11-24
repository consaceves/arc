use serde::{Serialize, Deserialize};
use diffy;
use crate::mach;
use crate::revid;
use crate::rev;

#[derive(Serialize, Deserialize, Debug)]
pub struct Repo {
    pub upstream: String,
    pub root_path: String,
    pub tracked_files: Vec<String>,
    pub all_revs: Vec<String>,
    pub cur_rev: String,
}

impl Repo {
    pub fn print_repo(&self) {
        println!("Repo info");
        println!("  Upstream @ {}", self.upstream);
        println!("  Root @ {}", self.root_path);
        println!("  Current: {}", self.cur_rev);
        for l in &self.all_revs {
            println!("  Revision: {}", l);
        }
        for l in &self.tracked_files {
            println!("  Tracked file: {}", l);
        }
    }
    
    pub fn set_upstream(&mut self, ups: &String) {
        self.upstream = ups.to_string();
    }
    
    pub fn save(&self) {
        let p = mach::join_paths(&self.root_path, &String::from(".arc"));
        
        let serialized = serde_json::to_string(self).unwrap();
        mach::write_string(&p, &String::from("repo.json"), &serialized);
    }
    
    /*
     * Rev
     */
    pub fn new_rev(&mut self, trunk_id: &String, other_id: &String) -> rev::Rev {
        let r = rev::Rev {
            rev_id: revid::gen_rev_id(),
            parent_trunk: trunk_id.clone(),
            parent_other: other_id.clone(),
            files: Vec::new()
        };
        
        let p = mach::join_paths(&self.root_path, &String::from(".arc"));
        mach::create_dir(&p, &r.rev_id);
        
        self.all_revs.push(r.rev_id.clone());
        r
    }
    
    /*
     * Copy
     */
    pub fn copy_from(&mut self, other: &Repo) {
        for other_rev in &other.all_revs {
            if !self.all_revs.contains(other_rev) {
                let p = mach::join_paths(&self.root_path, &String::from(".arc"));
                let p2 = mach::join_paths(&p, other_rev);
                mach::create_dir(&p, other_rev);
                
                let op = mach::join_paths(&other.root_path, &String::from(".arc"));
                let op2 = mach::join_paths(&op, other_rev);
                
                let or = rev::open_rev(&op2);
                mach::copy_files(&p2, &op2, &or.files);
                mach::copy_file(&p2, &op2, &"rev.json".to_string());
                self.all_revs.push(other_rev.to_string());
            }
        }
    }
    
    /*
     * Add/Remove/Checkout/Commit
     */
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
            println!("Checkout: {}", rev_id);
            
            let p = mach::join_paths(&self.root_path, &String::from(".arc"));
            let p2 = mach::join_paths(&p, rev_id);
            let r = rev::open_rev(&p2);
            
            mach::del_files(&self.root_path, &self.tracked_files);
            mach::del_files(&self.root_path, &r.files);
            mach::copy_files(&self.root_path, &p2, &r.files);
            
            self.cur_rev = rev_id.clone();
            self.tracked_files.clear();
            for f in &r.files {
                self.tracked_files.push(f.to_string());
            }
        }
    }
    
    /*
     * Merge
     */
    pub fn can_reach_root(&self, from_id: &String, skip_id: &String) -> bool {
        if from_id == "None" {
            true
        } else if from_id == skip_id {
            false
        } else {
            let p = mach::join_paths(&self.root_path, &String::from(".arc"));
            let p2 = mach::join_paths(&p, from_id);
            let r = rev::open_rev(&p2);
            self.can_reach_root(&r.parent_trunk, skip_id) ||
                (r.parent_other != "None" && self.can_reach_root(&r.parent_other, skip_id))
        }
    }
    
    pub fn find_trunk_parents(&self, from_id: &String) -> Vec<String> {
        let mut parents = Vec::new();
        let mut id = from_id.to_string();
        while id != "None" {
            if !self.can_reach_root(from_id, &id) {
                parents.push(id.to_string());
            }
            
            let p = mach::join_paths(&self.root_path, &String::from(".arc"));
            let p2 = mach::join_paths(&p, &id);
            let r = rev::open_rev(&p2);
            
            id = r.parent_trunk.to_string();
            println!("id: {}", id);
        }
        parents
    }
    
    pub fn find_common_ancestor(&self, trunk_id: &String, other_id: &String) -> String {
        let parents_trunk = self.find_trunk_parents(trunk_id);
        let parents_other = self.find_trunk_parents(other_id);
        
        println!("parents_trunk: {:?}", parents_trunk);
        println!("parents_other: {:?}", parents_other);
        
        let mut anc_id = "None".to_string();
        for p in &parents_other {
            if parents_trunk.contains(p) && p != trunk_id && p != other_id {
                anc_id = p.to_string();
                break;
            }
        }
        anc_id
    }
    
    pub fn merge3(&self, f: &String, anc_p: &String, p1: &String, p2: &String, out_p: &String) -> bool {
        let sa = if anc_p.is_empty() { "".to_string() } else { mach::read_line(&anc_p, &f) };
        let s1 = if p1.is_empty() { "".to_string() } else { mach::read_line(&p1, &f) };
        let s2 = if p2.is_empty() { "".to_string() } else { mach::read_line(&p2, &f) };
        let so = diffy::merge(&sa, &s1, &s2).unwrap().to_string();
        mach::write_string(&out_p, &f, &so);
        true
    }
    
    pub fn merge(&mut self, trunk_id: &String, other_id: &String) {
        let p = mach::join_paths(&self.root_path, &String::from(".arc"));
        let anc_id = self.find_common_ancestor(trunk_id, other_id);
        
        let anc_p = mach::join_paths(&p, &anc_id);
        println!("anc_p: {}", anc_p);
        let anc_r = rev::open_rev(&anc_p);
        
        let tru_p = mach::join_paths(&p, &trunk_id);
        let tru_r = rev::open_rev(&tru_p);
        
        let oth_p = mach::join_paths(&p, &other_id);
        let oth_r = rev::open_rev(&oth_p);
        
        let mut r = self.new_rev(&trunk_id, &other_id);
        let p2 = mach::join_paths(&p, &r.rev_id);
        
        // Go through all files from ancestor
        for f in &anc_r.files {
            if tru_r.files.contains(&f) && oth_r.files.contains(&f) {
                // Try merge
                println!("3way merge");
                let ok = self.merge3(&f, &anc_p, &tru_p, &oth_p, &p2);
                if ok { println!("Merged @ {}", f); }
                else  { println!("Conflict @ {}", f); }
                r.files.push(f.to_string());
            } else if !tru_r.files.contains(&f) && !oth_r.files.contains(&f) {
                // Don't copy - the file has been deleted
                println!("Deleted @ {}", f);
            } else if !tru_r.files.contains(&f) && oth_r.files.contains(&f) {
                // File deleted in one parent, check conflicts
                let ok = self.merge3(&f, &anc_p, &"".to_string(), &oth_p, &p2);
                if ok { println!("Merged @ {}", f); }
                else  { println!("Conflict @ {}", f); }
            } else if tru_r.files.contains(&f) && !oth_r.files.contains(&f) {
                // File deleted in one parent, check conflicts
                let ok = self.merge3(&f, &anc_p, &tru_p, &"".to_string(), &p2);
                if ok { println!("Merged @ {}", f); }
                else  { println!("Conflict @ {}", f); }
            } else {
                println!("Conflict @ {}", f);
            }
        }
        
        // Go through all files from trunk parent
        for f in &tru_r.files {
            if !anc_r.files.contains(&f) && oth_r.files.contains(&f) {
                // Newly created file, try merge
                let ok = self.merge3(&f, &"".to_string(), &tru_p, &oth_p, &p2);
                if ok { println!("Merged @ {}", f); }
                else  { println!("Conflict @ {}", f); }
                r.files.push(f.to_string());
            } else if !anc_r.files.contains(&f) && !oth_r.files.contains(&f) {
                // Newly created file, copy
                mach::copy_file(&p2, &tru_p, &f);
                r.files.push(f.to_string());
                println!("Merged @ {}", f);
            }
        }
        
        // Go through all files from other parent
        for f in &oth_r.files {
            if !anc_r.files.contains(&f) && !tru_r.files.contains(&f) {
                // Newly created file, copy
                mach::copy_file(&p2, &oth_p, &f);
                r.files.push(f.to_string());
                println!("Merged @ {}", f);
            }
        }
        
        // Done
        r.save(&p2);
        self.cur_rev = r.rev_id.clone();
    }
}

pub fn init_repo(root_path: &String) {
    println!("Init repo @ {}", root_path);
    let already_inited = mach::check_repo_dir(root_path);
    if already_inited {
        println!("Must be an empty dir!");
    } else {
        mach::create_dir(root_path, &String::from(".arc"));
        let p = mach::join_paths(root_path, &String::from(".arc"));
        
        let mut repo = Repo {
            upstream: "None".to_string(),
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

