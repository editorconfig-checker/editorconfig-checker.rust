mod checker;
mod error;

use crate::error::Result;
use std::process;

fn main() -> Result<()> {
    let version = "2.0.2";
    let architecture: &str;
    let os_type: String;

    match checker::get_architecture() {
        Ok(result) => architecture = result,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }

    match checker::get_os_type() {
        Ok(result) => os_type = result,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }

    let filename = checker::generate_filename(&os_type, architecture);
    let base_path = checker::get_base_path()?;
    let tar_path = format!("{}/{}.tar.gz", base_path, filename);
    let binary_path = format!("{}/bin/{}", base_path, filename);

    if !checker::path_exists(&binary_path) {
        let base_url: String = checker::generate_base_url(version);

        checker::download(&base_url, &filename)?;
        match checker::unpack(&tar_path, &base_path) {
            Ok(()) => (),
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        }
    }

    let command = process::Command::new(binary_path)
        .arg(checker::get_args_as_string(std::env::args()))
        .output()
        .expect("failed to run binary");

    let output = std::str::from_utf8(&command.stdout);
    match output {
        Ok(res) => println!("{}", res),
        Err(err) => {
            print!("{}", err);
            process::exit(1);
        }
    }

    if !command.status.success() {
        process::exit(1);
    }
    Ok(())
}
