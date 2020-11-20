use std::env;
mod cmd;
mod mach;
mod revid;
mod repo;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    
    println!("ARC DVCS");
    cmd::command();
}
