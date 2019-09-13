extern crate flate2;
extern crate reqwest;
extern crate sys_info;
extern crate tar;

use std::env;
use std::fs;
use std::io;
use std::string::String;

pub fn get_architecture() -> Result<String, String> {
    // TODO: This is not sufficient and needs to care for more cases
    if cfg!(target_pointer_width = "64") {
        return Ok(String::from("amd64"));
    } else if cfg!(target_pointer_width = "32") {
        return Ok(String::from("386"));
    }

    Err(String::from("Unknown architecture"))
}

pub fn path_exists(filename: &String) -> bool {
    std::path::Path::new(&filename).exists()
}

pub fn get_args_as_string() -> String {
    let mut args: Vec<String> = Vec::new();

    for (index, arg) in env::args().enumerate() {
        if index == 0 {
            continue;
        }

        args.push(arg)
    }

    args.join(" ")
}

pub fn get_os_type() -> Result<String, String> {
    let os_type_result = sys_info::os_type();

    match os_type_result {
        Ok(os_type) => Ok(os_type.to_lowercase()),
        Err(_) => Err(String::from("Can't get operating system type")),
    }
}

pub fn generate_filename(os: String, arch: String) -> String {
    format!("ec-{}-{}", os, arch)
}

pub fn generate_base_url(version: String) -> String {
    let base_url = String::from(
        "https://github.com/editorconfig-checker/editorconfig-checker/releases/download",
    );
    format!("{}/{}", base_url, version)
}

pub fn download(base_url: String, filename: &String) {
    let filepath = format!("{}/{}.tar.gz", get_base_path(), filename);
    let url = format!("{}/{}.tar.gz", base_url, filename);
    let mut resp = reqwest::get(&url).expect("request failed");
    let mut out = fs::File::create(filepath).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}

pub fn unpack(tar_path: &String, base_path: String) -> Result<(), std::io::Error> {
    let tar_gz = fs::File::open(&tar_path)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(base_path)?;
    fs::remove_file(&tar_path)?;

    Ok(())
}

// TODO: Needs error handling
pub fn get_base_path() -> String {
    let path = env::current_exe();

    match path {
        Ok(p) => {
            let mut path = p.into_os_string().into_string().unwrap();
            let index = path.rfind('/');
            match index {
                Some(idx) => path.truncate(idx),
                None => println!(""),
            };

            path
        }

        Err(_) => String::from(""),
    }
}
