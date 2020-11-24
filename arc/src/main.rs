use clap::{App, Arg};

mod cmd;
mod mach;
mod revid;
mod repo;
mod rev;

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
            App::new("clone")
                .about("Clone a repository")
                .arg(Arg::new("repository").about("The repository user wants to clone").required(true)),
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
            App::new("status")
                .about("Check the current status of the current repository")
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
        )
        .subcommand(
            App::new("commit")
                .about("Commit changes")
        )
        .get_matches();

        match matches.subcommand() {
            Some(("init", init_matches)) => {
                if init_matches.is_present("directory") {
                    println!("New repository at {}", init_matches.value_of("directory").unwrap());
                } else {
                    println!("init was used without directory");
                }
            }
            Some(("clone", clone_matches)) => {
                println!("arc clone was used");
            }
            Some(("add", add_matches)) => {
                println!("arc add was used");
            }
            Some(("remove", remove_matches)) => {
                println!("arc remove was used");
            }
            Some(("status", status_matches)) => {
                println!("arc status was used");
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
                println!("arc checkout was used");
            }
            Some(("commit", commit_matches)) => {
                println!("arc commit was used");
            }
            None => println!("No subcommand was used"),
            _ => unreachable!(), 
        }
}
