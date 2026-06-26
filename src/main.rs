mod help;
mod interpreter;
mod kyro;
mod parser;
mod primitives;
mod stdlib;

use kyro::Kyro;

fn main() -> std::io::Result<()> {
    let mut kyro = Kyro::new();
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            kyro.run_prompt()?;
        }
        2 => {
            let cmd = args[1].as_str();
            match cmd {
                "help" | "--help" | "-h" => {
                    help::print_help();
                }
                "--version" | "-v" => {
                    help::print_version();
                }
                "init" => {
                    help::init_project()?;
                }
                "fmt" => {
                    help::format_code(None)?;
                }
                file_path => {
                    kyro.run_file(file_path)?;
                }
            }
        }
        3 => {
            let cmd = args[1].as_str();
            if cmd == "fmt" {
                help::format_code(Some(&args[2]))?;
            } else {
                kyro.run_file(&args[1])?;
            }
        }
        _ => {
            kyro.run_file(&args[1])?;
        }
    }
    Ok(())
}
