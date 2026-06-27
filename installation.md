`kyro` can currently be installed only through source. Basic knowledge about using a terminal is expected. 

To install, follow these steps:

### Prerequisites

* **Rust** for compiling the binary: [https://rust-lang.org/tools/install/](https://rust-lang.org/tools/install/)
* **Git (Optional)** to clone the repo: [https://git-scm.com/](https://git-scm.com/), alternatively, download it manually from github.

### Clone the repository locally

```bash
git clone https://github.com/externref/kyro 
cd kyro
```

### Make the script executable in the shell

Make sure you're in the repo's directory and run the following command

/// tab | linux/macos

    :::bash
    chmod +x install.sh
///

/// tab | windows

    :::powershell
    Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process
///

### Build and setup the language binary (and library)

/// tab | linux/macos

    :::bash
    ./install.sh
///

/// tab | windows

    :::powershell
    .\install.ps1
///

!!! note "Note: "
    What does these commands do?

    * both scripts run `cargo build -r` to compile the language binary.
    * create a `.kyro` directory in the home folder of the user.
    * move the binary in `target/release/` and the `lib` directory to the `.kyro` directory.
    * add the directory as `KYRO_HOME` environment variable and add it to PATH for binary access systemwide. 


### Test 

Create a test file with any name, for this example we'll use `main.kyro`

```kyro
var io = use("std:io");

fn main(){
    io.println("hello, world!");
}

main();
```

Run this command 

```bash
kyro main.kyro
```

Expected Output: 
```
externref@MSI:~/projects/kyro$ kyro main.kyro
hello, world!
externref@MSI:~/projects/kyro$ 
```