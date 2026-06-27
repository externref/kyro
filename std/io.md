---
title: io
---

This module contains functions related to input/output operations for communicating with the console.

**Include using**

```rust
var io = use("std:io");
```

### print

```rust
print(value: Any) -> Nil
```

Prints a value to the console.

* **Parameters:**
    * `value` *(Any)*: The value to be printed to the output stream.
* **Returns:** *(Nil)*

```rust
var io = use("std:io");

io.print("hello, world!\n");
```

### println

```rust
println(value: Any) -> Nil
```

Same as `print`, but automatically appends a newline at the end.

* **Parameters:**
    * `value` *(Any)*: The value to be printed to the output stream.
* **Returns:** *(Nil)*

```rust
var io = use("std:io");

io.println("hello, world!");
```

### input

```rust
input(prompt: String) -> String
```

Prompts the user for input and returns the entered text as a string.

* **Parameters:**
    * `prompt` *(String)*: The text displayed to the user before waiting for input.
* **Returns:** *(String)* The text entered by the user.

```rust
var io = use("std:io");

var name = io.input("enter your name: ");

io.println("hello, ${name}!");
```