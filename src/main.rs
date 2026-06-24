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
            kyro.run_file(&args[1])?;
        }
        _ => {
            eprintln!("Usage: kyro [script].kyro");
            std::process::exit(64);
        }
    }
    Ok(())
}