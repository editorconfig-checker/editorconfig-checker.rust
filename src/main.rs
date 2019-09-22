mod checker;
mod error;

use crate::{checker::OsType, error::Result};
use std::{
    env,
    io::{self, Write},
    process,
};

fn main() -> Result<()> {
    let version = "2.0.3";

    let architecture = checker::get_architecture()?;
    let os_type = sys_info::os_type()?;
    let os_type = os_type.parse::<OsType>()?;
    // let os_type = checker::get_os_type(&sys_info::os_type()?)?;

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
