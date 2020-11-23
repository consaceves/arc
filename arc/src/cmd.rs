use std::env;
//use std::path::PathBuf;

use crate::mach;
use crate::repo;

pub fn command() {
    let args: Vec<String> = env::args().collect();
    let cwd = mach::get_cwd();
    
    let cmd_name = &args[1];
    match &cmd_name[..] {
        "init" => {
            let mut repo_root_path = &cwd;
            if args.len() >= 3 {
                repo_root_path = &args[2];
            }
            repo::init_repo(repo_root_path);
        },
        "print" => {
            let mut repo_root_path = &cwd;
            if args.len() >= 3 {
                repo_root_path = &args[2];
            }
            let r = repo::open_repo(repo_root_path);
            r.print_repo();
        },
        "add" => {
            if args.len() >= 3 {
                let file_abs_path = mach::join_paths(&cwd, &args[2]);
                let repo_root_path = mach::find_repo_root_path(&file_abs_path);
                let file_rel_path = mach::find_rel_path(&repo_root_path, &file_abs_path);
                
                let mut r = repo::open_repo(&repo_root_path);
                r.add(&file_rel_path);
                r.print_repo();
                r.save();
            }
        },
        "rm" => {
            if args.len() >= 3 {
                let file_abs_path = mach::join_paths(&cwd, &args[2]);
                let repo_root_path = mach::find_repo_root_path(&file_abs_path);
                let file_rel_path = mach::find_rel_path(&repo_root_path, &file_abs_path);
                
                let mut r = repo::open_repo(&repo_root_path);
                r.remove(&file_rel_path);
                r.print_repo();
                r.save();
            }
        },
        "commit" => {
            let repo_root_path = mach::find_repo_root_path(&cwd);
            let mut r = repo::open_repo(&repo_root_path);
            r.commit();
            r.save();
        },
        "checkout" => {
            if args.len() >= 3 {
                let repo_root_path = mach::find_repo_root_path(&cwd);
                let mut r = repo::open_repo(&repo_root_path);
                r.checkout(&args[2]);
                r.save();
            }
        },
        _ => println!("unknown command: {}", cmd_name),
    }
}

