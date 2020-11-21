use std::env;
mod cmd;
mod mach;
mod revid;
mod repo;
mod rev;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Args: {:?}", args);
    cmd::command();
}
