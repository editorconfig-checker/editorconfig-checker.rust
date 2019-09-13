mod checker;
mod error;

use crate::error::Result;
use std::process;

fn main() -> Result<()> {
    let version = "2.0.2";
    let architecture = checker::get_architecture()?;
    let os_type = checker::get_os_type()?;

    let filename = checker::generate_filename(&os_type, architecture);
    let base_path = checker::get_base_path()?;
    let tar_path = format!("{}/{}.tar.gz", base_path, filename);
    let binary_path = format!("{}/bin/{}", base_path, filename);

    if !checker::path_exists(&binary_path) {
        let base_url: String = checker::generate_base_url(version);

        checker::download(&base_url, &filename)?;
        checker::unpack(&tar_path, &base_path)?;
    }

    let command = process::Command::new(binary_path)
        .arg(checker::get_args_as_string(std::env::args()))
        .output()
        .expect("failed to run binary");

    println!("{}", std::str::from_utf8(&command.stdout)?);

    process::exit(command.status.code().unwrap_or_default());
}
