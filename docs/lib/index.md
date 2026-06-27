---
title: lib modules
---

## library modules

The `lib` module group consists of library files written in Kyro itself, located inside the `$KYRO_HOME/lib` directory. 

These can be imported into your scripts using the `use` function with a `lib:<module name>` prefix and stored in a namespace.

**Example**
```javascript
var math = use("lib:math");
```

!!! info "Quick Note:"
    You can view a module's public functions and variables at runtime using the `dir` function.

        :::javascript
        var json = use("lib:json");

        print(dir(json)); 

## index of `lib` modules

* [lib:json](./json.md): module for encoding and decoding JSON data structures.
* [lib:math](./math.md): mathematical constants and utility functions written in kyro.