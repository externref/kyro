---
title: fs
---

This module contains functions for interacting with the file system.

**Include using**

```kyro
var fs = use("std:fs");
```

### read_file

```kyro
read_file(path: String) -> String
```

Reads the contents of a file and returns it as a string.

* **Parameters:**
    * `path` *(String)*: The path to the file to read.
* **Returns:** *(String)* The contents of the file.

```kyro
var fs = use("std:fs");

var text = fs.read_file("hello.txt");
```

### write_file

```kyro
write_file(path: String, content: Any = "") -> Nil
```

Writes data to a file. If the file already exists, its contents are replaced.

* **Parameters:**
    * `path` *(String)*: The path to the target file.
    * `content` *(Any, optional)*: The data to write. Non-string types are converted to their string representations. Defaults to an empty string `""` (acting like a touch utility).
* **Returns:** *(Nil)*

```kyro
var fs = use("std:fs");

// Writes data to the file
fs.write_file("hello.txt", "Hello, world!");

// Omit content to create/touch an empty file
fs.write_file("empty.txt");
```

### exists

```kyro
exists(path: String) -> Bool
```

Checks whether a file or directory exists.

* **Parameters:**
    * `path` *(String)*: The path to check.
* **Returns:** *(Bool)* `true` if the path exists, otherwise `false`.

```kyro
var fs = use("std:fs");

if (fs.exists("hello.txt")) {
    print("File exists!");
}
```

### remove_file

```kyro
remove_file(path: String) -> Nil
```

Deletes a file from the file system.

* **Parameters:**
    * `path` *(String)*: The path to the file to be deleted.
* **Returns:** *(Nil)*

```kyro
var fs = use("std:fs");

fs.remove_file("hello.txt");
```

### create_dir

```kyro
create_dir(path: String, recursive: Bool = false) -> Nil
```

Creates a new directory at the specified path. 

* **Parameters:**
    * `path` *(String)*: The path of the directory to create.
    * `recursive` *(Bool, optional)*: If `true`, recursively creates any missing parent directories. Defaults to `false`.
* **Returns:** *(Nil)*

```kyro
var fs = use("std:fs");

// Creates a single directory
fs.create_dir("new_folder");

// Creates missing parent directories recursively
fs.create_dir("parent/child/nested", recursive = true);
```

### read_dir

```kyro
read_dir(path: String) -> List
```

Reads and lists the entries of a directory.

* **Parameters:**
    * `path` *(String)*: The path of the directory to read.
* **Returns:** *(List)* A list of strings containing the file and directory names.

```kyro
var fs = use("std:fs");

var files = fs.read_dir("parent");
for (var i = 0; i < files.len(); i = i + 1) {
    print(files[i]);
}
```