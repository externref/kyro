---
title: fs
---

This module contains functions for interacting with the file system.

**Include using**

```rust
var fs = use("std:fs");
```

### read_file

```rust
read_file(path: String) -> String
```

Reads the contents of a file and returns it as a string.

* **Parameters:**
    * `path` *(String)*: The path to the file to read.
* **Returns:** *(String)* The contents of the file.

```rust
var fs = use("std:fs");

var text = fs.read_file("hello.txt");
```

### write_file

```rust
write_file(path: String, content: Any) -> Nil
```

Writes data to a file. If the file already exists, its contents are replaced.

* **Parameters:**
    * `path` *(String)*: The path to the target file.
    * `content` *(Any)*: The data to write. Non-string types are converted to their string representations.
* **Returns:** *(Nil)*

```rust
var fs = use("std:fs");

fs.write_file("hello.txt", "Hello, world!");
```

### exists

```rust
exists(path: String) -> Bool
```

Checks whether a file or directory exists.

* **Parameters:**
    * `path` *(String)*: The path to check.
* **Returns:** *(Bool)* `true` if the path exists, otherwise `false`.

```rust
var fs = use("std:fs");

if (fs.exists("hello.txt")) {
    print("File exists!");
}
```

### remove_file

```rust
remove_file(path: String) -> Nil
```

Deletes a file from the file system.

* **Parameters:**
    * `path` *(String)*: The path to the file to be deleted.
* **Returns:** *(Nil)*

```rust
var fs = use("std:fs");

fs.remove_file("hello.txt");
```