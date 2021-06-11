mod architecture;
mod checker;
mod error;
mod ostype;

use crate::{architecture::Architecture, error::Result, ostype::OsType};
use std::{
    env::consts,
    io::{self, Write},
    process,
};

fn main() -> Result<()> {
    let version = "2.0.3";

    // create XDG profile
    let xdg_dirs = xdg::BaseDirectories::with_profile("editorconfig-checker", version)?;
    let architecture = consts::ARCH.parse::<Architecture>()?;
    let os_type = consts::OS.parse::<OsType>()?;

    // create XDG cache dir in case it does not exist
    xdg_dirs.create_cache_directory("")?;
    // use XDG cache home as base path
    let base_path = xdg_dirs.get_cache_home();
    let filename = checker::generate_filename(os_type, architecture);
    let tar_path = format!("{}/{}.tar.gz", base_path.display(), filename);
    let binary_path = format!("{}/bin/{}", base_path.display(), filename);

    if !checker::path_exists(&binary_path) {
        let base_url: String = checker::generate_base_url(version);

        checker::download(&base_url, base_path.display(), &filename)?;
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
