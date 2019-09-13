use std::process;
use std::string::String;

mod lib;

fn main() {
    let version: String = String::from("2.0.2");
    let architecture: String;
    let os_type: String;

    match lib::get_architecture() {
        Ok(result) => architecture = result,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }

    match lib::get_os_type() {
        Ok(result) => os_type = result,
        Err(err) => {
            println!("{}", err);
            process::exit(1);
        }
    }

    let filename = lib::generate_filename(os_type, architecture);
    let base_path = lib::get_base_path();
    let tar_path = format!("{}/{}.tar.gz", base_path, filename);
    let binary_path = format!("{}/bin/{}", base_path, filename);

    if !lib::path_exists(&binary_path) {
        let base_url: String = lib::generate_base_url(version);

        lib::download(base_url, &filename);
        match lib::unpack(&tar_path, base_path) {
            Ok(()) => (),
            Err(err) => {
                println!("{}", err);
                process::exit(1);
            }
        }
    }

    let command = process::Command::new(binary_path)
        .arg(lib::get_args_as_string(std::env::args()))
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
}
