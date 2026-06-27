---
title: util
---

This module contains utility functions for conversions, shortcuts and language features. 

**Include using**

```kyro
var util = use("std:util");
```

### to_string

```kyro
to_string(value: Any) -> String
```

Converts a value into its string representation.

* **Parameters:**
    * `value` *(Any)*: The value to convert into a string.
* **Returns:** *(String)* The string representation of the given value.

```kyro
var util = use("std:util");

var s = util.to_string(42);

print(s); // "42"
```

### to_number

```kyro
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

```kyro
var util = use("std:util");

print(util.to_number("123")); // 123
print(util.to_number(true));  // 1
print(util.to_number(nil));   // 0
```

### info

```kyro
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

```kyro
var util = use("std:util");

var info = util.info();

print(info.language);
print(info.version);
```

### type_of

```kyro
type_of(value: Any) -> Class
```

Inspects the provided value and returns its corresponding namespace class object (type constructor).

* **Parameters:**
    * `value` *(Any)*: The value to inspect.
* **Returns:** *(Class)* The matching class object. Possible return values include:
    * Built-in type classes (`Nil`, `Bool`, `Number`, `String`, `List`, `Dict`, `Class`, `Callable`)
    * Custom class definitions (for class instances)

```kyro
var util = use("std:util");

// Check primitive types against global namespace classes
print(util.type_of("hello") == String); // true
print(util.type_of(42) == Number);      // true
print(util.type_of([1, 2]) == List);    // true

// Check custom classes
class Hello {}
var h = Hello();

print(util.type_of(h) == Hello);        // true
print(util.type_of(Hello) == Class);    // true
```

### range

```kyro
range(start: Number, end: Number, step: Number = 1) -> List
```

Generates a sequential list of numbers from the `start` value up to (but excluding) the `end` value, progressing by the `step` size.

* **Parameters:**
    * `start` *(Number)*: The starting boundary of the range sequence.
    * `end` *(Number)*: The exclusive end boundary of the range sequence.
    * `step` *(Number, optional)*: The step increment value. Must be a non-zero number. Defaults to `1`.
* **Returns:** *(List)* A sequential list of numbers.

```kyro
var util = use("std:util");

// Range from 0 to 5 using default step size of 1
var r1 = util.range(0, 5); 
print(r1); // [0, 1, 2, 3, 4]

// Range with custom step size
var r2 = util.range(0, 10, step = 2);
print(r2); // [0, 2, 4, 6, 8]

// Reverse range using negative step size
var r3 = util.range(5, 0, step = -1);
print(r3); // [5, 4, 3, 2, 1]
```