use crate::kyro::Kyro;

const FORMATTER_SRC: &str = include_str!("formatter.kyro");
const VERSION: &str = include_str!(".version");

pub fn print_help() {
    println!(
        "\x1b[1;33mkyro [{}]\x1b[0m - an open source language based on lox interpreter",
        VERSION
    );
    println!();
    println!("\x1b[1;33mlinks:\x1b[0m");
    println!("  github  \x1b[4;36mhttps://github.com/externref/kyro\x1b[0m");
    println!("  docs    \x1b[4;36mhttps://kyro.externref.dev\x1b[0m");
    println!();
    println!("\x1b[1;33musage:\x1b[0m");
    println!("  kyro                     \x1b[90mstart the interactive repl\x1b[0m");
    println!("  kyro \x1b[1m<file.kyro>\x1b[0m         \x1b[90mrun a script file\x1b[0m");
    println!(
        "  kyro \x1b[1minit\x1b[0m                \x1b[90minitialize a new project setup\x1b[0m"
    );
    println!("  kyro \x1b[1mfmt\x1b[0m                 \x1b[90mformat code\x1b[0m");
    println!(
        "  kyro \x1b[1mhelp\x1b[0m | \x1b[1m--help\x1b[0m | \x1b[1m-h\x1b[0m  \x1b[90mshow this help menu\x1b[0m"
    );
    println!(
        "  kyro \x1b[1m--version\x1b[0m | \x1b[1m-v\x1b[0m      \x1b[90mshow kyro version\x1b[0m"
    );
    println!();
    println!("\x1b[1;33mcommands:\x1b[0m");
    println!(
        "  \x1b[1minit\x1b[0m    creates a standard directory layout with a default 'main.kyro'"
    );
    println!(
        "  \x1b[1mfmt\x1b[0m     formats the current directory's kyro source files (not implemented)."
    );
}

pub fn init_project() -> std::io::Result<()> {
    let default_main = "var io = use(\"std:io\");\n\nfn main() {\n    io.println(\"hello, world!\");\n}\n\nmain();\n";
    std::fs::write("main.kyro", default_main)?;
    println!("\x1b[1;32msuccess\x1b[0m: initialized a new kyro project with 'main.kyro'.");
    Ok(())
}

pub fn print_version() {
    println!("\x1b[1;33mkyro\x1b[0m version \x1b[1m{}\x1b[0m", VERSION);
}

pub fn format_code(specific_file: Option<&str>) -> std::io::Result<()> {
    let mut files = Vec::new();

    if let Some(file) = specific_file {
        let path = std::path::Path::new(file);
        if !path.exists() {
            println!("\x1b[1;31merror\x1b[0m: file '{file}' not found.");
            return Ok(());
        }
        files.push(file.to_string());
    } else {
        for entry in std::fs::read_dir(".")? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().map_or(false, |ext| ext == "kyro") {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    files.push(name.to_string());
                }
            }
        }
    }

    if files.is_empty() {
        println!("\x1b[1;33mwarning\x1b[0m: no .kyro files found to format.");
        return Ok(());
    }

    let mut script = String::new();
    script.push_str(FORMATTER_SRC);
    script.push_str("\n\nvar io = use(\"std:io\");\n");

    for file in &files {
        script.push_str(&format!("io.println(\"formatting {file}...\");\n"));
        script.push_str(&format!("format_file(\"{file}\");\n"));
    }

    let temp_file = ".kyro_tmp_fmt.kyro";
    std::fs::write(temp_file, &script)?;

    let mut kyro = Kyro::new();
    let result = kyro.run_file(temp_file);
    let _ = std::fs::remove_file(temp_file);

    match result {
        Ok(_) => {
            println!("\x1b[1;32msuccess\x1b[0m: all source files formatted successfully.");
        }
        Err(e) => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("formatter execution failed: {e}"),
            ));
        }
    }

    Ok(())
}
