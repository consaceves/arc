use clap::{App, Arg};

mod cmd;
mod mach;
mod revid;
mod repo;
mod rev;
mod merge;

fn main() {
    let matches = App::new("arc")
        .version("1.0")
        .about("DVCS System in Rust")
        .author("Aceves, Zimmerman, Zheng")
        .subcommand(
            App::new("init")
                .about("Initialize repository")
                .arg(Arg::new("directory").about("Directory path").required(false))        
            )
        .subcommand(
            App::new("print")
                .about("Prints a repository")
                .arg(Arg::new("path").about("The repository user wants to print").required(false)),
        )
        .subcommand(
            App::new("clone")
                .about("Clone a repository")
                .arg(Arg::new("src").about("The source repository").required(true))
                .arg(Arg::new("dst").about("The destination repository").required(false))
        )
        .subcommand(
            App::new("add")
                .about("Add specific files that user wants to track")
                .arg(Arg::new("path").about("Path of the added file").required(true))
        )
        .subcommand(
            App::new("remove")
                .about("Remove specific files from tracking list")
                .arg(Arg::new("path").about("Path of the removed file").required(true))
        )
        .subcommand(
            App::new("heads")
                .about("Show the current heads")
        )
        .subcommand(
            App::new("diff")
                .about("Check the changes between revisions")
        )
        .subcommand(
            App::new("cat")
                .about("Inspect a file of a given revision")
                .arg(Arg::new("path").about("File path of inspected file").required(true))
        )
        .subcommand(
            App::new("checkout")
                .about("Check out a specific revision")
                .arg(Arg::new("rev").about("Revision to checkout to").required(true))
        )
        .subcommand(
            App::new("commit")
                .about("Commit changes")
        )
        .subcommand(
            App::new("merge")
                .about("Merge two revisions")
                .arg(Arg::new("rev1").about("First revision to merge").required(true))
                .arg(Arg::new("rev2").about("Second revision to merge").required(true))
        )
        .subcommand(
            App::new("push")
                .about("Push changes")
        )
        .subcommand(
            App::new("pull")
                .about("Pull changes")
        )
        .get_matches();

        match matches.subcommand() {
            Some(("init", init_matches)) => {
                let mut args = Vec::new();
                let mut directory = String::new();
                if init_matches.is_present("directory") {
                    directory = init_matches.value_of("directory").unwrap().to_string();
                } else {
                    directory = mach::get_cwd();
                }
                args.push(&directory);
                cmd::command("init".to_string(), args)
            }
            Some(("print", print_matches)) => {
                let mut args = Vec::new();
                let mut path = String::new();
                if print_matches.is_present("path") {
                    path = print_matches.value_of("path").unwrap().to_string();
                } else {
                    path = mach::get_cwd();
                }
                args.push(&path);
                cmd::command("print".to_string(), args)
            }
            Some(("clone", clone_matches)) => {
                let mut args = Vec::new();
                let mut src = String::new();
                let mut dst = String::new();
                if clone_matches.is_present("src") {
                    src = clone_matches.value_of("src").unwrap().to_string();
                }
                args.push(&src);
                if clone_matches.is_present("dst") {
                    dst = clone_matches.value_of("dst").unwrap().to_string();
                } else {
                    dst = mach::get_cwd();
                }
                args.push(&dst);
                cmd::command("clone".to_string(), args)
            }
            Some(("add", add_matches)) => {
                let mut args = Vec::new();
                let mut path = String::new();
                if add_matches.is_present("path") {
                    path = add_matches.value_of("path").unwrap().to_string();
                }
                args.push(&path);
                cmd::command("add".to_string(), args);
            }
            Some(("remove", remove_matches)) => {
                let mut args = Vec::new();
                let mut path = String::new();
                if remove_matches.is_present("path") {
                    path = remove_matches.value_of("path").unwrap().to_string();
                }
                args.push(&path);
                cmd::command("remove".to_string(), args);
            }
            Some(("heads", heads_matches)) => {
                println!("arc heads was used");
            }
            Some(("diff", diff_matches)) => {
                println!("arc diff was used");
            }
            Some(("cat", cat_matches)) => {
                println!("arc cat was used");
            }
            Some(("checkout", checkout_matches)) => {
                let mut args = Vec::new();
                let mut rev = String::new();
                if checkout_matches.is_present("rev") {
                    rev = checkout_matches.value_of("rev").unwrap().to_string();
                }
                args.push(&rev);
                cmd::command("checkout".to_string(), args);
            }
            Some(("commit", commit_matches)) => {
                let args = Vec::new();
                cmd::command("commit".to_string(), args)
            }
            Some(("merge", merge_matches)) => {
                let mut args = Vec::new();
                let mut rev1 = String::new();
                let mut rev2 = String::new();
                if merge_matches.is_present("rev1") {
                    rev1 = merge_matches.value_of("rev1").unwrap().to_string();
                }
                if merge_matches.is_present("rev2") {
                    rev2 = merge_matches.value_of("rev2").unwrap().to_string();
                }
                args.push(&rev1);
                args.push(&rev2);
                cmd::command("merge".to_string(), args);
            }
            Some(("push", push_matches)) => {
                let args = Vec::new();
                cmd::command("push".to_string(), args)
            }
            Some(("pull", pull_matches)) => {
                let args = Vec::new();
                cmd::command("pull".to_string(), args)
            }
            None => println!("No subcommand was used"),
            _ => unreachable!(), 
        }
}
