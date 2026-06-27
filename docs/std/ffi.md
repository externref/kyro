---
title: ffi
---

This module contains functions for interacting with Foreign Function Interfaces (FFI), allowing kyro to dynamically load compiled C libraries and call their functions.

**Include using**

```kyro
var ffi = use("std:ffi");
```

### load

```kyro
load(path: String) -> FfiLibrary
```

Loads a compiled C shared library (`.so`, `.dll`, or `.dylib` file) dynamically at runtime.

* **Parameters:**
    * `path` *(String)*: The file path to the compiled shared library.
* **Returns:** *(FfiLibrary)* An object instance representing the loaded library, which exposes binding methods.

```kyro
var ffi = use("std:ffi");

// Loads a local shared library
var my_lib = ffi.load("./libtest.so");
```

---

### FfiLibrary (Class)

The object representing a loaded shared library.

#### bind

```kyro
bind(name: String, return_type: String, param_types: List) -> Callable
```

Binds a compiled symbol from the shared library to a callable kyro function by specifying its signature.

Supported FFI data types:
* `"int"`: Maps to a 32-bit signed integer.
* `"double"`: Maps to a double-precision floating-point number.
* `"string"`: Maps to a null-terminated character pointer (`char*`).
* `"void"` / `"nil"`: Maps to no return value (only valid as a return type).

* **Parameters:**
    * `name` *(String)*: The exact compiled symbol/function name inside the library.
    * `return_type` *(String)*: The FFI return type of the function.
    * `param_types` *(List)*: A list of strings representing the expected parameter FFI types.
* **Returns:** *(Callable)* A callable kyro function that routes arguments to the C function.

```kyro
var ffi = use("std:ffi");
var my_lib = ffi.load("./libtest.so");

// Bind a C function: int add(int x, int y)
var add = my_lib.bind("add", "int", ["int", "int"]);
print(add(10, 20)); // 30

// Bind a C function: const char* greet(const char* name)
var greet = my_lib.bind("greet", "string", ["string"]);
print(greet("kyro")); // "Hello, kyro!"
```