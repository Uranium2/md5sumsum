extern crate walkdir;
use std::{env, process::Command};


fn work(f: &str) -> String {
    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(format!("md5sum \"{}\"", f));
    let output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
    let vec: Vec<&str> = output.split(" ").collect();
    String::from(vec[0])

}

fn main() {
    let mut paths: Vec<String> = vec![];
    for arg in env::args().skip(1) {
        for entry in walkdir::WalkDir::new(arg)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.file_type().is_dir())
        {
            paths.push(String::from(entry.path().to_string_lossy()))
        }
    }
    let mut hashcat = String::from("");

    paths.iter()
        .map(|file|{
                let s: String = work(file).to_owned();
                let s_slice: &str = &s[..];  // take a full slice of the string
                hashcat += s_slice;
            }
            );

    let mut cmd = Command::new("sh");
    cmd.arg("-c").arg(format!("echo \"{}\" | md5sum", hashcat));
    let output = String::from_utf8_lossy(&cmd.output().unwrap().stdout).to_string();
    print!("{}", output);
}