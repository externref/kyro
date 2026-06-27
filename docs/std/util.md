---
title: util
---

This module contains utility functions for conversions, shortcuts and language features. 

**Include using**

```rust
var util = use("std:util");
```

### to_string

```rust
to_string(value: Any) -> String
```

Converts a value into its string representation.

* **Parameters:**
    * `value` *(Any)*: The value to convert into a string.
* **Returns:** *(String)* The string representation of the given value.

```rust
var util = use("std:util");

var s = util.to_string(42);

print(s); // "42"
```

### to_number

```rust
to_number(value: Any) -> Number
```

Converts a value into a number.

Conversion rules:

* `Number` values are returned unchanged.
* `String` values are parsed as numbers.
* `true` converts to `1`.
* `false` converts to `0`.
* `nil` converts to `0`.
* Other types will raise a runtime error.

* **Parameters:**
    * `value` *(Any)*: The value to parse or convert to a number.
* **Returns:** *(Number)* The numeric representation of the given value.

```rust
var util = use("std:util");

print(util.to_number("123")); // 123
print(util.to_number(true));  // 1
print(util.to_number(nil));   // 0
```

### info

```rust
info() -> Object
```

Returns information about the current kyro runtime.

The returned object contains the following fields:

* `language`
* `version`

* **Parameters:** None
* **Returns:** *(Object)* An object containing the runtime parameters:
    * `language`: The string identifier of the language.
    * `version`: The current version of the kyro interpreter.

```rust
var util = use("std:util");

var info = util.info();

print(info.language);
print(info.version);
```