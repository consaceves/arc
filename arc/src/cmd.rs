use std::env;
//use std::path::PathBuf;

use crate::mach;
use crate::repo;

pub fn command() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    
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
        "open" => {
            let mut repo_root_path = &cwd;
            if args.len() >= 3 {
                repo_root_path = &args[2];
            }
            let r = repo::open_repo(repo_root_path);
            repo::print_repo(&r);
        },
        _ => println!("unknown command: {}", cmd_name),
    }
}

