use crate::error::{Error, Result};
use std::{env, fs, io};

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

    #[test]
    fn test_path_exists() {
    }
}

// TODO: How to use cfg to pass a value into this function to be able to test it?
// TODO: Test
pub fn get_architecture() -> Result<&'static str> {
    // TODO: This is not sufficient and needs to care for more cases
    if cfg!(target_pointer_width = "64") {
        Ok("amd64")
    } else if cfg!(target_pointer_width = "32") {
        Ok("386")
    } else {
        Err(Error::UnknownArch)
    }
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
pub fn get_os_type() -> Result<String> {
    Ok(sys_info::os_type().map(|os_type| os_type.to_lowercase())?)
}

pub fn generate_filename(os: &str, arch: &str) -> String {
    format!("ec-{}-{}", os, arch)
}

pub fn generate_base_url(version: &str) -> String {
    let base_url = "https://github.com/editorconfig-checker/editorconfig-checker/releases/download";
    format!("{}/{}", base_url, version)
}

// TODO: Test
pub fn download(base_url: &str, filename: &str) -> Result<()> {
    let filepath = format!("{}/{}.tar.gz", get_base_path()?, filename);
    let url = format!("{}/{}.tar.gz", base_url, filename);
    let mut resp = reqwest::get(&url).expect("request failed");
    let mut out = fs::File::create(filepath)?;
    io::copy(&mut resp, &mut out)?;
    Ok(())
}

// TODO: Test
pub fn unpack(tar_path: &str, base_path: &str) -> Result<()> {
    let tar_gz = fs::File::open(&tar_path)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(base_path)?;
    fs::remove_file(&tar_path)?;

    Ok(())
}

// TODO: Needs error handling
// TODO: Test
pub fn get_base_path() -> Result<String> {
    let path = env::current_exe()?;

    let mut path = path.into_os_string().into_string()?;
    if let Some(idx) = path.rfind('/') {
        path.truncate(idx);
    }

    Ok(path)
}
