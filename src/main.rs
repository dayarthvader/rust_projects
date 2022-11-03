use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::{env, process::exit};
use tar::Archive;

fn usage() {
    println!("Usage: ./tar2csv input_tar_file output_csv_file");
    exit(1);
}

fn write_to_csv(filename: String, contents: &mut String, csv_file: String) {
    let mut csv = OpenOptions::new()
        .append(true)
        .write(true)
        .open(&csv_file)
        .expect("Unable to open the file");
    let write_string = filename + "," + contents;
    csv.write_all(write_string.as_bytes()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        usage();
    }
    let ipt_tar_file = &args[1];
    let opt_csv_file = &args[2];
    if Path::new(ipt_tar_file).exists() != true {
        println!("Error: {}: No such file found", ipt_tar_file);
        exit(1);
    }

    if Path::new(ipt_tar_file).extension().unwrap() != "tar" {
        println!("Error: {}: Doesn't seem to be a tar file", ipt_tar_file);
        exit(1);
    }

    if !Path::exists(opt_csv_file.as_ref()) {
        File::create(opt_csv_file).unwrap();
    }

    if !Path::new(opt_csv_file).has_root() {
        println!("Error: {}: can not be created", opt_csv_file);
        exit(1);
    }

    let mut archive = Archive::new(File::open(ipt_tar_file).unwrap());
    let mut buffer = String::new();
    for file in archive.entries().unwrap() {
        let mut data = file.unwrap();
        data.read_to_string(&mut buffer).unwrap();
        let filename = data.path().unwrap().to_str().unwrap().to_string();
        if filename.ends_with("txt") {
            write_to_csv(filename, &mut buffer, opt_csv_file.to_string());
        }
    }
    println!("{}, {}", ipt_tar_file, opt_csv_file);
}
