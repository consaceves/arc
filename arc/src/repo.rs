use crate::mach;
use crate::revid;

pub struct Repo {
    pub root_path: String,
    pub revs: Vec<String>,
    pub cur_rev: String,
}


//impl Repo {
//    
//}

pub fn init_repo(root_path: &String) {
    println!("Init repo @ {}", root_path);
    let already_inited = mach::check_repo_dir(root_path);
    if already_inited {
        println!("Must be an empty repo!");
    } else {
        mach::create_dir(root_path, &String::from(".arc_dvcs"));
        let p = mach::join_paths(root_path, &String::from(".arc_dvcs"));
        
        let init_rev_id = revid::gen_rev_id();
        let mut init_revs = Vec::new();
        init_revs.push(format!("{}, parent-trunk: None, parent-other: None", init_rev_id));
        mach::write_lines(&p, &String::from("revision.txt"), init_revs);
        
        let mut init_cur = Vec::new();
        init_cur.push(format!("revision: {}", init_rev_id));
        mach::write_lines(&p, &String::from("current.txt"), init_cur);
        
        //mach::write_lines(&p, &String::from("branch.txt"), Vec::new());
    }
}

pub fn open_repo(root_path: &String) -> Repo {
    let p = mach::join_paths(root_path, &String::from(".arc_dvcs"));
    // TODO: check if p exists
    
    let revs = mach::read_lines(&p, &String::from("revision.txt"));
    let cur = mach::read_lines(&p, &String::from("current.txt"));
    
    let repo = Repo { root_path: root_path.to_string(), revs: revs, cur_rev: cur[0].clone() };
    repo
}

pub fn print_repo(repo: &Repo) {
    println!("Repo info");
    println!("  Root @ {}", repo.root_path);
    for l in &repo.revs {
        println!("  Rev: {}", l);
    }
    println!("  Current: {}", repo.cur_rev);
}

