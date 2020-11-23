use clap::{App, Arg};

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

        if let Some(subcommand) = matches.subcommand_name() {
            println!("'arc {}' was used", subcommand);
        }
}
