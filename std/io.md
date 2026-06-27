---
title: io
---

This module contains functions related to input/output operations for communicating with the console.

**Include using**

```kyro
var io = use("std:io");
```

### print

```kyro
print(value: Any = "") -> Nil
```

Prints a value to the console.

* **Parameters:**
    * `value` *(Any, optional)*: The value to be printed to the output stream. Defaults to `""`.
* **Returns:** *(Nil)*

```kyro
var io = use("std:io");

io.print("hello, world!\n");
```

### println

```kyro
println(value: Any = "") -> Nil
```

Same as `print`, but automatically appends a newline at the end.

* **Parameters:**
    * `value` *(Any, optional)*: The value to be printed to the output stream. Defaults to `""`.
* **Returns:** *(Nil)*

```kyro
var io = use("std:io");

io.println("hello, world!");

// Omit the parameter to print a blank line
io.println();
```

### input

```kyro
input(prompt: String = "") -> String
```

Prompts the user for input and returns the entered text as a string.

* **Parameters:**
    * `prompt` *(String, optional)*: The text displayed to the user before waiting for input. Defaults to `""`.
* **Returns:** *(String)* The text entered by the user.

```kyro
var io = use("std:io");

var name = io.input("enter your name: ");

io.println("hello, ${name}!");

// Omit prompt to wait for user input without a display prompt
var response = io.input();
```

### clear

```kyro
clear() -> Nil
```

Clears the terminal screen and resets the cursor to the top-left corner.

* **Parameters:** None
* **Returns:** *(Nil)*

```kyro
var io = use("std:io");

io.clear();
```

### write_err

```kyro
write_err(value: Any = "") -> Nil
```

Prints a value directly to the standard error stream (`stderr`), appending a newline at the end.

* **Parameters:**
    * `value` *(Any, optional)*: The value to write to standard error. Defaults to `""`.
* **Returns:** *(Nil)*

```kyro
var io = use("std:io");

io.write_err("error: process failed");
```