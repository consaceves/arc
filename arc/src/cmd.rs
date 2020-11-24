use std::env;
//use std::path::PathBuf;

use crate::mach;
use crate::repo;

pub fn command(cmd_name: String, args: Vec<&String>) {
    let cwd = mach::get_cwd();

    match &cmd_name[..] {
        "init" => {
            let repo_root_path = &args[0];
            repo::init_repo(repo_root_path);
        },
        "print" => {
            let repo_root_path = &args[0];
            let r = repo::open_repo(repo_root_path);
            r.print_repo();
        },
        "add" => {
            let file_abs_path = mach::join_paths(&cwd, &args[0]);
            let repo_root_path = mach::find_repo_root_path(&file_abs_path);
            let file_rel_path = mach::find_rel_path(&repo_root_path, &file_abs_path);
            
            let mut r = repo::open_repo(&repo_root_path);
            r.add(&file_rel_path);
            r.print_repo();
            r.save();
        },
        "remove" => {
            let file_abs_path = mach::join_paths(&cwd, &args[0]);
            let repo_root_path = mach::find_repo_root_path(&file_abs_path);
            let file_rel_path = mach::find_rel_path(&repo_root_path, &file_abs_path);
            
            let mut r = repo::open_repo(&repo_root_path);
            r.remove(&file_rel_path);
            r.print_repo();
            r.save();
        },
        "commit" => {
            let repo_root_path = mach::find_repo_root_path(&cwd);
            let mut r = repo::open_repo(&repo_root_path);
            r.commit();
            r.save();
        }, 
        "checkout" => {
            let repo_root_path = mach::find_repo_root_path(&cwd);
            let mut r = repo::open_repo(&repo_root_path);
            r.checkout(&args[0]);
            r.save();
        },
        "merge" => {
            let repo_root_path = mach::find_repo_root_path(&cwd);
            let mut r = repo::open_repo(&repo_root_path);
            r.merge(&args[0], &args[1]);
            r.save();
        },
        "clone" => {
            let src_repo_path = mach::find_repo_root_path(&args[0]);
            let dst_repo_path = &args[1];
            
            repo::init_repo(&dst_repo_path);
            
            let src_r = repo::open_repo(&src_repo_path);
            let mut dst_r = repo::open_repo(&dst_repo_path);
            
            dst_r.set_upstream(&src_repo_path);
            dst_r.copy_from(&src_r);
            dst_r.checkout(&src_r.cur_rev);
            dst_r.save();
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

