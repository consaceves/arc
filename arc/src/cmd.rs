use std::env;
use crate::mach;
use crate::repo;

pub fn command(cmd_name: String, args: Vec<&String>) {
    let cwd = mach::get_cwd();

    match &cmd_name[..] {
        "init" => {
            let repo_root_path = &args[0];
            repo::init(repo_root_path);
        },
        "print" => {
            let repo_root_path = &args[0];
            let r = repo::open(repo_root_path);
            println!("{}", r);
        },
        "add" => {
            let file_abs_path = mach::join_paths(&cwd, &args[2]);
            let repo_root_path = mach::find_repo_root_path(&file_abs_path);
            let file_rel_path = mach::find_rel_path(&repo_root_path, &file_abs_path);
            
            let mut r = repo::open(&repo_root_path);
            r.add_file(&file_rel_path);
            r.save();
            
            println!("{}", r);
        },
        "rm" => {
            let file_abs_path = mach::join_paths(&cwd, &args[0]);
            let repo_root_path = mach::find_repo_root_path(&file_abs_path);
            let file_rel_path = mach::find_rel_path(&repo_root_path, &file_abs_path);
            
            let mut r = repo::open(&repo_root_path);
            r.remove_file(&file_rel_path);
            r.save();
            
            println!("{}", r);
        },
        "commit" => {
            let repo_root_path = mach::find_repo_root_path(&cwd);
            let mut repo = repo::open(&repo_root_path);
            repo.commit();
            repo.save();
        },
        "checkout" => {
            let repo_root_path = mach::find_repo_root_path(&cwd);
            let mut repo = repo::open(&repo_root_path);
            repo.checkout(&args[0]);
            repo.save();
        },
        "merge" => {
            if args.len() >= 2 {
                let repo_root_path = mach::find_repo_root_path(&cwd);
                let mut repo = repo::open(&repo_root_path);
                let rev = repo.merge(&args[0], &args[1]);
                repo.checkout(&rev.get_id_str());
                println!("Merged -> {}", rev.get_id_str());
            } else if args.len() >= 1 {
                let repo_root_path = mach::find_repo_root_path(&cwd);
                let mut repo = repo::open(&repo_root_path);
                let rev = repo.merge(&repo.get_head_rev_str(), &args[0]);
                repo.checkout(&rev.get_id_str());
                println!("Merged -> {}", rev.get_id_str());
            }
        },
        "clone" => {
            let src_repo_path = mach::find_repo_root_path(&args[0]);
            let dst_repo_path = &args[1];
            
            repo::init(&dst_repo_path);
            
            let src_r = repo::open(&src_repo_path);
            let mut dst_r = repo::open(&dst_repo_path);
            
            dst_r.set_upstream(&src_repo_path);
            dst_r.sync(&src_r);
            dst_r.checkout(&src_r.get_head_rev_str());
        },
        "push" => {
            let local_repo_path = mach::find_repo_root_path(&cwd);
            let local_repo = repo::open(&local_repo_path);
            
            let mut upstream_repo = repo::open(local_repo.get_upstream());
            upstream_repo.sync(&local_repo);
            
            let rev = upstream_repo.merge(&upstream_repo.get_head_rev_str(), &local_repo.get_head_rev_str());
            upstream_repo.checkout(&rev.get_id_str());
        },
        "pull" => {
            let local_repo_path = mach::find_repo_root_path(&cwd);
            let mut local_repo = repo::open(&local_repo_path);
            
            let upstream_repo = repo::open(local_repo.get_upstream());
            local_repo.sync(&upstream_repo);
            
            let rev = local_repo.merge(&upstream_repo.get_head_rev_str(), &local_repo.get_head_rev_str());
            local_repo.checkout(&rev.get_id_str());
        },
        _ => println!("unknown command: {}", cmd_name),
    }
}

