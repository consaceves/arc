use serde::{Serialize, Deserialize};
use std::fmt;
use crate::mach;
use crate::revid;
use crate::revid::RevID;
use crate::repo;
use crate::merge;

#[derive(Serialize, Deserialize, Debug)]
struct RevInfo {
    rev_id: RevID,
    parent_trunk: RevID,
    parent_other: RevID,
    files: Vec<String>,
}

pub struct Rev {
    pub root_path: String,
    pub arc_path: String,
    pub rev_path: String,
    rev: RevInfo,
}

impl fmt::Display for RevInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Revision ID: {}\n", self.rev_id)?;
        write!(f, "Parent ID (trunk): {}", self.parent_trunk)?;
        write!(f, "Parent ID (other): {}", self.parent_other)?;
        write!(f, "All Files:\n")?;
        for l in &self.files {
            write!(f, "  {}\n", l.to_string())?;
        }
        Ok(())
    }
}

impl fmt::Display for Rev {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Root Path @ {}\n", self.root_path)?;
        write!(f, ".arc Path @ {}\n", self.arc_path)?;
        write!(f, "Revision Path @ {}\n", self.rev_path)?;
        write!(f, "{}", self.rev)?;
        Ok(())
    }
}

impl Rev {
    pub fn save(&self) {
        let serialized = serde_json::to_string(&self.rev).unwrap();
        mach::write_string(&self.rev_path, &String::from("rev.json"), &serialized);
    }
    
    pub fn get_id(&self) -> &RevID {
        &self.rev.rev_id
    }
    
    pub fn get_id_str(&self) -> String {
        self.rev.rev_id.to_string()
    }
    
    pub fn get_files(&self) -> &Vec<String> {
        &self.rev.files
    }
    
    pub fn get_parent_trunk_id(&self) -> &RevID {
        &self.rev.parent_trunk
    }
    
    pub fn get_parent_other_id(&self) -> &RevID {
        &self.rev.parent_other
    }
    
    pub fn commit(&mut self, tracked_files: &Vec<String>) -> Vec<String> {
        let mut missing_files = Vec::new();
        for f_rel_path in tracked_files {
            if mach::check_path(&mach::join_paths(&self.root_path, &f_rel_path)) {
                mach::copy_file(&self.rev_path, &self.root_path, f_rel_path);
                self.rev.files.push(f_rel_path.clone());
            } else {
                missing_files.push(f_rel_path.clone());
            }
        }
        missing_files
    }
    
    pub fn checkout(&self) {
        for f_rel_path in &self.rev.files {
            assert!(mach::check_path(&mach::join_paths(&self.rev_path, &f_rel_path)), "File missing in a revision!");
            mach::copy_file(&self.root_path, &self.rev_path, f_rel_path);
        }
    }
    
    pub fn copy_to(&self, dst_path: &String) {
        mach::create_dir_all(dst_path);
        
        for f_rel_path in &self.rev.files {
            assert!(mach::check_path(&mach::join_paths(&self.root_path, &f_rel_path)), "File missing in a revision!");
            mach::copy_file(&dst_path, &self.rev_path, f_rel_path);
        }
        mach::copy_file(&dst_path, &self.rev_path, &"rev.json".to_string());
    }
    
    pub fn merge(&mut self, ancestor_rev: &Rev, trunk_rev: &Rev, other_rev: &Rev) {
        let ancestor_files = ancestor_rev.get_files();
        let trunk_files = trunk_rev.get_files();
        let other_files = other_rev.get_files();
        
        let files = merge::find_all_files(ancestor_files, trunk_files, other_files);
        for f in &files {
            let ancestor_content = if ancestor_files.contains(&f) { Some(mach::read_line(&ancestor_rev.rev_path, &f)) } else { None };
            let trunk_content = if trunk_files.contains(&f) { Some(mach::read_line(&trunk_rev.rev_path, &f)) } else { None };
            let other_content = if other_files.contains(&f) { Some(mach::read_line(&other_rev.rev_path, &f)) } else { None };
            
            let m = merge::merge3(ancestor_content, trunk_content, other_content);
            if m.is_some() {
                mach::write_string(&self.rev_path, &f, &m.unwrap());
                self.rev.files.push(f.clone());
            }
        }
    }
}

pub fn new(repo: &repo::Repo, trunk_id: &RevID, other_id: &RevID) -> Rev {
    let rev = RevInfo {
        rev_id: revid::new(),
        parent_trunk: trunk_id.clone(),
        parent_other: other_id.clone(),
        files: Vec::new(),
    };
    
    let rev_path = mach::join_paths(&repo.arc_path, &rev.rev_id.to_string());
    mach::create_dir_all(&rev_path);
    
    Rev {
        root_path: repo.root_path.clone(),
        arc_path: repo.arc_path.clone(),
        rev_path: rev_path.clone(),
        rev: rev
    }
}

pub fn open(repo: &repo::Repo, rev_id: &RevID) -> Rev {
    let rev_path = mach::join_paths(&repo.arc_path, &rev_id.to_string());
    assert!(mach::check_path(&rev_path), "Revision dir doesn't exist!");
    
    let json = mach::read_line(&rev_path, &String::from("rev.json"));
    let r: RevInfo = serde_json::from_str(&json).expect("Unable to open revision, bad rev file!");
    
    Rev {
        root_path: repo.root_path.clone(),
        arc_path: repo.arc_path.clone(),
        rev_path: rev_path.clone(),
        rev: r
    }
}

