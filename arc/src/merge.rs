use diffy;
use crate::repo;
use crate::rev;
use crate::revid;
use crate::revid::RevID;

pub fn can_reach_root(repo: &repo::Repo, from_id: &RevID, skip_id: &RevID) -> bool {
    if from_id.is_empty() {
        true
    } else if from_id == skip_id {
        false
    } else {
        let rev = rev::open(repo, from_id);
        let parent_trunk_id = rev.get_parent_trunk_id();
        let parent_other_id = rev.get_parent_other_id();
        can_reach_root(repo, parent_trunk_id, skip_id) ||
            (!parent_other_id.is_empty() && can_reach_root(repo, &parent_other_id, skip_id))
    }
}

pub fn find_trunk_parents(repo: &repo::Repo, from_id: &RevID) -> Vec<RevID> {
    let mut parents = Vec::new();
    let mut id = *from_id;
    while !id.is_empty() {
        if !can_reach_root(repo, from_id, &id) {
            parents.push(id.clone());
        }
        
        let r = rev::open(repo, &id);
        id = *r.get_parent_trunk_id();
        //println!("id: {}", id);
    }
    parents
}

pub fn find_common_ancestor(repo: &repo::Repo, trunk_id: &RevID, other_id: &RevID) -> RevID {
    let parents_trunk = find_trunk_parents(repo, trunk_id);
    let parents_other = find_trunk_parents(repo, other_id);
    
    //println!("parents_trunk: {:?}", parents_trunk);
    //println!("parents_other: {:?}", parents_other);
    
    let mut anc_id = revid::EMPTY;
    for p in &parents_other {
        if parents_trunk.contains(p) && p != trunk_id && p != other_id {
            anc_id = *p;
            break;
        }
    }
    anc_id
}

pub fn can_reach_rev(repo: &repo::Repo, cur_id: &RevID, target_id: &RevID) -> bool {
    if cur_id.is_empty() {
        false
    } else if cur_id == target_id {
        true
    } else {
        let rev = rev::open(repo, cur_id);
        let parent_trunk_id = rev.get_parent_trunk_id();
        let parent_other_id = rev.get_parent_other_id();
        can_reach_rev(repo, parent_trunk_id, target_id) ||
            (!parent_other_id.is_empty() && can_reach_rev(repo, parent_other_id, target_id))
    }
}

fn merge3_with_diffy(anc_s: &String, s1: &String, s2: &String) -> Option<String> {
    let m = diffy::merge(anc_s, s1, s2);
    let m = match m {
        Ok(mok) => {
            Some(mok.clone())
        },
        Err(mconf) => {
            println!("Conflict detected!\n{}", &mconf);
            None
        }
    };
    m
}

pub fn merge3(ancestor: Option<String>, trunk: Option<String>, other: Option<String>) -> Option<String> {
    if ancestor.is_some() {
        if trunk.is_some() {
            if other.is_some() {
                let m = merge3_with_diffy(&ancestor.unwrap(), &trunk.unwrap(), &other.unwrap());
                assert!(m.is_some(), "Merge failed");
                Some(m.unwrap().clone())
            } else {
                let m = merge3_with_diffy(&ancestor.unwrap(), &trunk.unwrap(), &"".to_string());
                assert!(m.is_some(), "Merge failed");
                let s = &m.unwrap();
                if s.is_empty() { None } else { Some(s.clone()) }
            }
        } else if other.is_some() {
            let m = merge3_with_diffy(&ancestor.unwrap(), &"".to_string(), &other.unwrap());
            assert!(m.is_some(), "Merge failed");
            let s = &m.unwrap();
            if s.is_empty() { None } else { Some(s.clone()) }
        } else {
            None
        }
    }
    
    else if trunk.is_some() {
        if other.is_some() {
            let m = merge3_with_diffy(&"".to_string(), &trunk.unwrap(), &other.unwrap());
            assert!(m.is_some(), "Merge failed");
            Some(m.unwrap().clone())
        } else {
            Some(trunk.unwrap().clone())
        }
    }
    
    else if other.is_some() {
        Some(other.unwrap().clone())
    }
    
    else {
        None
    }
}

pub fn find_all_files(v1: &Vec<String>, v2: &Vec<String>, v3: &Vec<String>) -> Vec<String> {
    let mut files = Vec::new();
    
    for f in v1 {
        if !files.contains(f) {
            files.push(f.clone());
        }
    }
    
    for f in v2 {
        if !files.contains(f) {
            files.push(f.clone());
        }
    }
    
    for f in v3 {
        if !files.contains(f) {
            files.push(f.clone());
        }
    }
    
    files
}

