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