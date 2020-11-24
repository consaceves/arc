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
        "merge" => {
            if args.len() >= 3 {
                let repo_root_path = mach::find_repo_root_path(&cwd);
                let mut r = repo::open_repo(&repo_root_path);
                r.merge(&r.cur_rev.to_string(), &args[2]);
                r.save();
            } else if args.len() >= 4 {
                let repo_root_path = mach::find_repo_root_path(&cwd);
                let mut r = repo::open_repo(&repo_root_path);
                r.merge(&args[2], &args[3]);
                r.save();
            }
        },
        "clone" => {
            if args.len() >= 3 {
                let src_repo_path = mach::find_repo_root_path(&args[2]);
                let dst_repo_path = if args.len() >= 4 { &args[3] } else { &cwd };
                
                repo::init_repo(&dst_repo_path);
                
                let src_r = repo::open_repo(&src_repo_path);
                let mut dst_r = repo::open_repo(&dst_repo_path);
                
                dst_r.set_upstream(&src_repo_path);
                dst_r.copy_from(&src_r);
                dst_r.checkout(&src_r.cur_rev);
                dst_r.save();
            }
        },
        "push" => {
            let local_repo_path = mach::find_repo_root_path(&cwd);
            let local_r = repo::open_repo(&local_repo_path);
            
            let mut upstream_r = repo::open_repo(&local_r.upstream);
            upstream_r.copy_from(&local_r);
            upstream_r.merge(&upstream_r.cur_rev.to_string(), &local_r.cur_rev.to_string());
            upstream_r.save();
        },
        "pull" => {
            let local_repo_path = mach::find_repo_root_path(&cwd);
            let mut local_r = repo::open_repo(&local_repo_path);
            
            let upstream_r = repo::open_repo(&local_r.upstream);
            local_r.copy_from(&upstream_r);
            local_r.merge(&upstream_r.cur_rev.to_string(), &local_r.cur_rev.to_string());
            local_r.save();
        },
        _ => println!("unknown command: {}", cmd_name),
    }
}

