extern crate flate2;
extern crate reqwest;
extern crate sys_info;
extern crate tar;

use std::env;
use std::fs;
use std::io;
use std::string::String;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_filename() {
        assert_eq!(
            generate_filename("abc", "def"),
            "ec-abc-def"
        );

        assert_eq!(
            generate_filename("linux", "amd64"),
            "ec-linux-amd64"
        );
    }

    #[test]
    fn test_generate_base_url() {
        assert_eq!(
            generate_base_url("2.0.2"),
            "https://github.com/editorconfig-checker/editorconfig-checker/releases/download/2.0.2"
        );

        assert_eq!(
            generate_base_url("1.0.0"),
            "https://github.com/editorconfig-checker/editorconfig-checker/releases/download/1.0.0"
        );
    }

    #[test]
    fn test_get_args_as_string() {
        assert_eq!(
            get_args_as_string(["arg1", "arg2"].iter().map(|s| s.to_string())),
            "arg2"
        );

        assert_eq!(
            get_args_as_string(["arg1", "arg2", "arg3"].iter().map(|s| s.to_string())),
            "arg2 arg3"
        );

        assert_eq!(
            get_args_as_string(["arg1"].iter().map(|s| s.to_string())),
            ""
        );
    }
}

// TODO: How to use cfg to pass a value into this function to be able to test it?
// TODO: Test
pub fn get_architecture() -> Result<&'static str, &'static str> {
    // TODO: This is not sufficient and needs to care for more cases
    if cfg!(target_pointer_width = "64") {
        return Ok("amd64");
    } else if cfg!(target_pointer_width = "32") {
        return Ok("386");
    }

    Err("Unknown architecture")
}

// TODO: Test
pub fn path_exists(filename: &str) -> bool {
    std::path::Path::new(filename).exists()
}

pub fn get_args_as_string<T>(args: T) -> String
where
    // Iterator<Item = String> matches to std::env::Args
    T: Iterator<Item = String>,
{
    args.skip(1).collect::<Vec<String>>().join(" ")
}

// TODO: Test
pub fn get_os_type() -> Result<String, &'static str> {
    let os_type_result = sys_info::os_type();

    match os_type_result {
        Ok(os_type) => Ok(os_type.to_lowercase()),
        Err(_) => Err("Can't get operating system type"),
    }
}

pub fn generate_filename(os: &str, arch: &str) -> String {
    format!("ec-{}-{}", os, arch)
}

pub fn generate_base_url(version: &str) -> String {
    let base_url = "https://github.com/editorconfig-checker/editorconfig-checker/releases/download";
    format!("{}/{}", base_url, version)
}

// TODO: Test
pub fn download(base_url: &str, filename: &str) {
    let filepath = format!("{}/{}.tar.gz", get_base_path(), filename);
    let url = format!("{}/{}.tar.gz", base_url, filename);
    let mut resp = reqwest::get(&url).expect("request failed");
    let mut out = fs::File::create(filepath).expect("failed to create file");
    io::copy(&mut resp, &mut out).expect("failed to copy content");
}

// TODO: Test
pub fn unpack(tar_path: &str, base_path: &str) -> Result<(), std::io::Error> {
    let tar_gz = fs::File::open(&tar_path)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(base_path)?;
    fs::remove_file(&tar_path)?;

    Ok(())
}

// TODO: Needs error handling
// TODO: Test
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
