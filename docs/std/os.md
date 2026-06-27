---
title: os
---

This module contains functions for interacting with the operating system, command-line arguments, and environment variables.

**Include using**

```rust
var os = use("std:os");
```

### args

```rust
args() -> List
```

Retrieves the command-line arguments passed to the running process.

* **Parameters:** None
* **Returns:** *(List)* A list of strings containing the command-line arguments.

```rust
var os = use("std:os");

var arguments = os.args();
for (var i = 0; i < arguments.len(); i = i + 1) {
    print(arguments[i]);
}
```

### load_dotenv

```rust
load_dotenv(path: String) -> Nil
```

Reads a `.env` file at the specified path and loads its contents into the current process's environment variables. Comments (lines starting with `#`) and empty lines are ignored.

* **Parameters:**
    * `path` *(String)*: The path to the `.env` file to read.
* **Returns:** *(Nil)*

```rust
var os = use("std:os");

os.load_dotenv(".env");
```

### get_env

```rust
get_env(key: String) -> String | Nil
```

Retrieves the value of an environment variable.

* **Parameters:**
    * `key` *(String)*: The name of the environment variable.
* **Returns:** *(String | Nil)* The value of the environment variable as a string, or `nil` if it is not set.

```rust
var os = use("std:os");

var port = os.get_env("PORT");
if (port == nil) {
    port = "8080";
}
print(port);
```

### set_env

```rust
set_env(key: String, value: String) -> Nil
```

Sets the value of an environment variable for the current process.

* **Parameters:**
    * `key` *(String)*: The name of the environment variable.
    * `value` *(String)*: The value to assign to the environment variable.
* **Returns:** *(Nil)*

```rust
var os = use("std:os");

os.set_env("DATABASE_URL", "sqlite://dev.db");
```

### get_envs

```rust
get_envs() -> List
```

Retrieves all environment variables currently set in the process.

* **Parameters:** None
* **Returns:** *(List)* A list containing key-value pairs, where each pair is a nested list represented as `[key: String, value: String]`.

```rust
var os = use("std:os");

var envs = os.get_envs();

for (var i = 0; i < envs.len(); i = i + 1) {
    var pair = envs[i];
    var key = pair[0];
    var val = pair[1];
    print(key + " = " + val);
}
```

### exit

```rust
exit(code: Number) -> Nil
```

Immediately terminates the current process with the specified exit status code.

* **Parameters:**
    * `code` *(Number)*: The exit code status to return to the operating system.
* **Returns:** *(Nil)*

```rust
var os = use("std:os");

// Terminate with a non-zero status indicating an error
os.exit(1);
```

### get_pid

```rust
get_pid() -> Number
```

Retrieves the Process ID (PID) of the current process.

* **Parameters:** None
* **Returns:** *(Number)* The numeric process identifier.

```rust
var os = use("std:os");

var pid = os.get_pid();
print("Running with PID: " + pid);
```

### platform

```rust
platform() -> String
```

Returns a string representing the target operating system family (e.g., `"windows"`, `"macos"`, `"linux"`).

* **Parameters:** None
* **Returns:** *(String)* The operating system name.

```rust
var os = use("std:os");

var current_platform = os.platform();
print("Host OS: " + current_platform);
```

### arch

```rust
arch() -> String
```

Returns a string representing the host CPU architecture (e.g., `"x86_64"`, `"aarch64"`).

* **Parameters:** None
* **Returns:** *(String)* The architecture name.

```rust
var os = use("std:os");

var architecture = os.arch();
print("Architecture: " + architecture);
```

### current_dir

```rust
current_dir() -> String
```

Returns the current working directory path of the running process.

* **Parameters:** None
* **Returns:** *(String)* The absolute path of the current directory.

```rust
var os = use("std:os");

var cwd = os.current_dir();
print("Current directory: " + cwd);
```

### set_current_dir

```rust
set_current_dir(path: String) -> Nil
```

Changes the current working directory of the process to the specified path.

* **Parameters:**
    * `path` *(String)*: The directory path to switch to.
* **Returns:** *(Nil)*

```rust
var os = use("std:os");

os.set_current_dir("/var/tmp");
```

### execute

```rust
execute(command: String, args: List) -> Dict
```

Spawns a shell command or executable as a subprocess, blocks until execution completes, and returns its output streams along with the exit status code.

* **Parameters:**
    * `command` *(String)*: The executable name or system command to run.
    * `args` *(List)*: A list of string arguments to pass to the executable.
* **Returns:** *(Dict)* A dictionary containing the following keys:
    * `exit_code`: *(Number)* The process exit status code (returns `-1` if the process was terminated by a signal).
    * `stdout`: *(String)* The captured stdout stream output.
    * `stderr`: *(String)* The captured stderr stream output.

```rust
var os = use("std:os");

var result = os.execute("git", ["--version"]);

if (result["exit_code"] == 0) {
    print("Success: " + result["stdout"]);
} else {
    print("Failed: " + result["stderr"]);
}
```