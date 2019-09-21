use crate::error::{Error, Result};
use std::{fs, io, path::Path, path::PathBuf};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Result;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::env;
    use std::fs::File;
    use tempfile::NamedTempFile;

    #[test]
    fn test_generate_filename() {
        assert_eq!(generate_filename("abc", "def"), "ec-abc-def");

        assert_eq!(generate_filename("linux", "amd64"), "ec-linux-amd64");
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
    fn test_path_exists() {
        let file = NamedTempFile::new().unwrap();
        let path = format!("{}", file.path().display());
        assert_eq!(true, path_exists(&path));
        file.close().expect("Closing and deleting tempfile failed");
        assert_eq!(false, path_exists(&path));
    }

    #[test]
    fn test_get_base_path() {
        let pwd = env::current_dir().unwrap();
        let pwd = pwd.display();

        // this is a little hacky... it only works as long as debug assertions stay dissabled for
        // release builds. but for now `cargo test` and `cargo test --release` work, when executed
        // from the project root
        #[cfg(debug_assertions)]
        let profile = "debug";
        #[cfg(not(debug_assertions))]
        let profile = "release";

        let expected = format!("{}/target/{}/deps", pwd, profile);
        let base_path = get_base_path(env::current_exe().unwrap()).unwrap();
        assert_eq!(expected, base_path)
    }

    #[test]
    fn test_download() -> Result<()> {
        let base_url = generate_base_url("2.0.3");
        let base_path = get_base_path(env::current_exe().unwrap()).unwrap();
        let os_type = get_os_type(&sys_info::os_type()?)?;

        let filename = generate_filename(&os_type, get_architecture().unwrap());

        let result = download(&base_url, &base_path, &filename);
        let tar_path = format!("{}/{}.tar.gz", base_path, filename);
        assert!(result.is_ok());
        assert!(true, path_exists(&tar_path));
        assert!(fs::remove_file(&tar_path).is_ok());

        Ok(())
    }

    #[test]
    fn test_unpack() {
        let archive_name = "archive.tar.gz";
        let file_name = "file.txt";

        // This block ensures every opened file is closed after the scope ends
        {
            // create the tar.gz file an define the compression
            let tar_gz = File::create(archive_name).expect("Creating of tarball failed");
            let enc = GzEncoder::new(tar_gz, Compression::default());
            let mut tar = tar::Builder::new(enc);

            // create a file to add to the tar.gz
            fs::write(file_name, "content").expect("Creating of sample file failed");

            // add the file to the tar.gz
            let mut f = File::open(file_name).expect("Opening sample file failed");
            tar.append_file(file_name, &mut f)
                .expect("Appending sample file to tarball failed");

            // remove added  file
            fs::remove_file(file_name).expect("Removing sample file failed");
        }

        // actual testing
        let pwd = env::current_dir().unwrap();
        let pwd = pwd.to_str().unwrap();
        let tar_path = format!("{}/{}", pwd, archive_name);

        assert!(unpack(&tar_path, &pwd).is_ok());
        fs::remove_file(file_name).expect("Removing sample file failed");
    }

    #[test]
    fn test_get_os_type() {
        assert_eq!(get_os_type("HALLO").unwrap(), "hallo");
        assert_eq!(get_os_type("Linux").unwrap(), "linux");
        assert_eq!(get_os_type("Darwin").unwrap(), "darwin");
        assert_eq!(get_os_type("WiNdOwS").unwrap(), "windows");
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

pub fn path_exists(filename: impl AsRef<std::path::Path>) -> bool {
    filename.as_ref().exists()
}

pub fn get_os_type(os_type: &str) -> Result<String, Error> {
    Ok(os_type.to_lowercase())
}

pub fn generate_filename(os: &str, arch: &str) -> String {
    format!("ec-{}-{}", os, arch)
}

pub fn generate_base_url(version: &str) -> String {
    let base_url = "https://github.com/editorconfig-checker/editorconfig-checker/releases/download";
    format!("{}/{}", base_url, version)
}

pub fn download(base_url: &str, base_path: &str, filename: &str) -> Result<()> {
    let filepath = format!("{}/{}.tar.gz", base_path, filename);
    let url = format!("{}/{}.tar.gz", base_url, filename);
    let mut resp = reqwest::get(&url)?;
    let mut out = fs::File::create(filepath)?;
    io::copy(&mut resp, &mut out)?;
    Ok(())
}

pub fn unpack(tar_path: impl AsRef<Path>, base_path: impl AsRef<Path>) -> Result<()> {
    let tar_gz = fs::File::open(&tar_path)?;
    let tar = flate2::read::GzDecoder::new(tar_gz);
    let mut archive = tar::Archive::new(tar);
    archive.unpack(base_path)?;
    fs::remove_file(&tar_path)?;

    Ok(())
}

pub fn get_base_path(path: PathBuf) -> Result<String> {
    path.parent()
        .map(Path::display)
        .map(|path| path.to_string())
        .ok_or(Error::InvalidBasePath)
}
