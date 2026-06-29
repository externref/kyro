---
title: std modules
---

## built-in modules

The `std` module group consists of built-in modules for the language that are written and constructed natively in rust.

These can be imported into your scripts using the `use` function with a `std:<module name>` prefix and stored in a namespace. 

**Example**
```javascript
var io = use("std:io");
```

!!! info "Quick Note:"
    You can view module content using the `dir` function for undocumented components of the module.

        :::javascript
        var io = use("std:io");

        echo dir(io); // [__name__, input, print, println]

## index of `std` modules

* [std:ffi](./ffi.md): implementation of foreign function interfaces to call C functions.
* [std:fs](./fs.md): file system operation related functions and variables.
* [std:io](./io.md): module with functions for taking inputs and outputs to and from console.
* [std:os](./os.md): functions and classes handling OS and environment related operations.
* [std:time](./time.md): module with system clock and time related functions.
* [std:util](./util.md): utility functions for conversions, shortcuts and language features.
