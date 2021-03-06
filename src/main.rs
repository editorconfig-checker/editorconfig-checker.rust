mod architecture;
mod checker;
mod error;
mod ostype;

use crate::{architecture::Architecture, error::Result, ostype::OsType};
use std::{
    env,
    env::consts,
    io::{self, Write},
    process,
};

fn main() -> Result<()> {
    let version = "2.0.3";

    let architecture = consts::ARCH.parse::<Architecture>()?;
    let os_type = consts::OS.parse::<OsType>()?;

    let base_path = checker::get_base_path(env::current_exe()?)?;
    let filename = checker::generate_filename(os_type, architecture);
    let tar_path = format!("{}/{}.tar.gz", base_path, filename);
    let binary_path = format!("{}/bin/{}", base_path, filename);

    if !checker::path_exists(&binary_path) {
        let base_url: String = checker::generate_base_url(version);

        checker::download(&base_url, &base_path, &filename)?;
        checker::unpack(&tar_path, &base_path)?;
    }

    let command = process::Command::new(binary_path)
        .args(std::env::args().skip(1))
        .output()
        .expect("failed to run binary");

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    writeln!(stdout, "{}", std::str::from_utf8(&command.stdout)?)?;

    process::exit(command.status.code().unwrap_or_default());
}
