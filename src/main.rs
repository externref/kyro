// MIT License

// Copyright (c) 2026 sarthak

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
    let arg_slices: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    match arg_slices.as_slice() {
        [_] => {
            kyro.run_prompt()?;
        }
        [_, "help" | "--help" | "-h"] => {
            help::print_help();
        }
        [_, "--version" | "-v"] => {
            help::print_version();
        }
        [_, "init"] => {
            help::init_project()?;
        }
        [_, "fmt"] => {
            help::format_code(None)?;
        }
        [_, "fmt", path] => {
            help::format_code(Some(path))?;
        }
        [_, file_path, ..] => {
            kyro.run_file(file_path)?;
        }
        _ => {
            kyro.run_prompt()?;
        }
    }

    Ok(())
}
