use std::env;
use std::path::Path;
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
//use std::path::PathBuf;

pub fn get_cwd() -> String {
    let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    //println!("{}", cwd);
    cwd
}

pub fn join_paths(path1: &String, path2: &String) -> String {
    let p = Path::new(path1).join(Path::new(path2));
    p.to_string_lossy().to_string()
}

pub fn check_repo_dir(path: &String) -> bool {
    Path::new(path).join(Path::new(".arc_dvcs")).exists()
}

pub fn create_dir(path: &String, name: &String) -> bool {
    let p = Path::new(path).join(Path::new(name));
    fs::create_dir_all(p).expect("Unable to create dir");
    true
}

pub fn write_lines(path: &String, name: &String, lines: Vec<String>) -> bool {
    let p = Path::new(path).join(Path::new(name));
    let mut f = File::create(p).expect("Unable to create file");
    for l in &lines {
        f.write_all(l.as_bytes()).expect("Unable to write line");
        f.write_all("\n".as_bytes()).expect("Unable to write line");
    }
    true
}

pub fn read_lines(path: &String, name: &String) -> Vec<String> {
    let p = Path::new(path).join(Path::new(name));
    let f = File::open(p).expect("Unable to open file");
    let buf = BufReader::new(f);
    buf.lines().map(|l| l.expect("Unable to read line")).collect()
}

