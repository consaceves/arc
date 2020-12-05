use std::env;
use std::path::Path;
use std::fs;
use std::ffi;
//use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::Write;
//use std::io::{Write, BufReader, BufRead};
use std::path::PathBuf;

pub fn get_cwd() -> String { // cwd = current working directory
    let cwd = env::current_dir().unwrap().into_os_string().into_string().unwrap();
    //println!("{}", cwd);
    cwd
}

pub fn join_paths(path1: &String, path2: &String) -> String {
    let p = Path::new(path1).join(Path::new(path2));
    p.to_string_lossy().to_string()
}

pub fn check_repo_dir(path: &String) -> bool {
    Path::new(path).join(Path::new(".arc")).exists()
}

pub fn create_dir(path: &String, name: &String) {
    let p = Path::new(path).join(Path::new(name));
    fs::create_dir_all(p).expect("Unable to create dir");
}

//pub fn write_lines(path: &String, name: &String, lines: &Vec<String>) {
//    let p = Path::new(path).join(Path::new(name));
//    let mut f = File::create(p).expect("Unable to create file");
//    for l in lines {
//        f.write_all(l.as_bytes()).expect("Unable to write line");
//        f.write_all("\n".as_bytes()).expect("Unable to write line");
//    }
//}

pub fn write_string(path: &String, name: &String, s: &String) {
    let p = Path::new(path).join(Path::new(name));
    let mut f = File::create(p).expect("Unable to create file");
    f.write_all(s.as_bytes()).expect("Unable to write line");
}

//pub fn read_lines(path: &String, name: &String) -> Vec<String> {
//    let p = Path::new(path).join(Path::new(name));
//    let f = File::open(p).expect("Unable to open file");
//    let buf = BufReader::new(f);
//    buf.lines().map(|l| l.expect("Unable to read line")).collect()
//}

pub fn read_line(path: &String, name: &String) -> String {
    let p = Path::new(path).join(Path::new(name));
    let mut f = File::open(p).expect("Unable to open file");
    let mut l = String::new();
    f.read_to_string(&mut l).expect("Unable to read the file");
    l
}

pub fn is_empty_path(pbuf: &PathBuf) -> bool {
    // doesn't work on Windows bc assume "/" is root
    // Path::new("/") == pbuf

    // my attempted solution
    pbuf.as_os_str() == std::ffi::OsStr::new("");
}

pub fn find_repo_root_path(path: &String) -> String {
    let mut pbuf = PathBuf::from(path);
    while !is_empty_path(&pbuf) {
        let p = pbuf.to_string_lossy().to_string();
        //println!("p: {}", p);
        if check_repo_dir(&p) {
            break;
        } else {
            pbuf.pop();
        }
    }
    pbuf.to_string_lossy().to_string()
}

pub fn find_rel_path(base_path: &String, full_path: &String) -> String {
    let b = full_path.starts_with(base_path);
    if b {
        full_path[base_path.len() + 1..].to_string()
    } else {
        "".to_string()
    }
}

pub fn copy_files(dst_path: &String, src_path: &String, files: &Vec<String>) {
    for f in files {
        let d = join_paths(dst_path, &f);
        let s = join_paths(src_path, &f);
        println!("Copy {} -> {}", s, d);
        fs::copy(&s, &d).expect("Unable to copy file");
    }
}

pub fn copy_file(dst_path: &String, src_path: &String, f: &String) {
    let d = join_paths(dst_path, &f);
    let s = join_paths(src_path, &f);
    println!("Copy {} -> {}", s, d);
    fs::copy(&s, &d).expect("Unable to copy file");
}

pub fn del_files(base_path: &String, files: &Vec<String>) {
    for f in files {
        let p = join_paths(base_path, &f);
        if Path::new(&p).exists() {
            println!("Del {}", p);
            fs::remove_file(&p).expect("Unable to delete file");
        }
    }
}

