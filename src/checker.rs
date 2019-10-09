use crate::architecture::Architecture;
use crate::error::{Error, Result};
use crate::ostype::OsType;
use std::{fs, io, path::Path, path::PathBuf};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::architecture::get_architecture;
    use crate::error::Result;
    use flate2::write::GzEncoder;
    use flate2::Compression;
    use std::env;
    use std::fs::File;
    use tempfile::NamedTempFile;

    #[test]
    fn test_generate_filename() {
        assert_eq!(
            generate_filename(OsType::Plan9, Architecture::Amd64),
            "ec-plan9-amd64"
        );

        assert_eq!(
            generate_filename(OsType::Linux, Architecture::I386),
            "ec-linux-386"
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

        let expected = format!(
            "{}{sep}target{sep}{}{sep}deps",
            pwd,
            profile,
            sep = std::path::MAIN_SEPARATOR
        );
        let base_path = get_base_path(env::current_exe().unwrap()).unwrap();
        assert_eq!(expected, base_path)
    }

    #[test]
    fn test_download() -> Result<()> {
        let base_url = generate_base_url("2.0.3");
        let base_path = get_base_path(env::current_exe().unwrap()).unwrap();
        let os_type = sys_info::os_type()?.parse::<OsType>()?;

        let filename = generate_filename(os_type, get_architecture());

        let result = download(&base_url, &base_path, &filename);
        let tar_path = format!("{}/{}.tar.gz", base_path, filename);
        assert!(result.is_ok());
        assert!(true, path_exists(&tar_path));
        assert!(fs::remove_file(&tar_path).is_ok());

        Ok(())
    }

    #[test]
    fn test_unpack() {
        let creation_dir = tempfile::tempdir().expect("Cannot create temporary directory");
        let archive_path = format!("{}/archive.tar.gz", creation_dir.path().display());
        let file_name = "file.txt";
        let initinal_content = "content";

        // This block ensures every opened file is closed after the scope ends
        {
            // create the tar.gz file an define the compression
            let tar_gz = File::create(&archive_path).expect("Creating of tarball failed");
            let enc = GzEncoder::new(tar_gz, Compression::default());
            let mut tar = tar::Builder::new(enc);

            // create a file to add to the tar.gz
            fs::write(file_name, &initinal_content).expect("Creating of sample file failed");

            // add the file to the tar.gz
            let mut f = File::open(file_name).expect("Opening sample file failed");
            tar.append_file(file_name, &mut f)
                .expect("Appending sample file to tarball failed");

            // remove added  file
            fs::remove_file(file_name).expect("Removing sample file failed");
        }
        let extraction_dir = tempfile::tempdir().expect("Cannot create temporary directory");

        // unpack the file
        assert!(unpack(&archive_path, &extraction_dir.path()).is_ok());
        let unpacked_content =
            fs::read_to_string(format!("{}/{}", extraction_dir.path().display(), file_name))
                .expect("Cannot read extracted file");
        // check that the extracted file contains the same as the initial file
        assert_eq!(initinal_content, unpacked_content);
    }
}

pub fn path_exists(filename: impl AsRef<std::path::Path>) -> bool {
    filename.as_ref().exists()
}

pub fn generate_filename(os: OsType, arch: Architecture) -> String {
    format!("ec-{}-{}", os.stringify(), arch.stringify())
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
