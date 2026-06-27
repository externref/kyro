---
title: core
---

The core global functions, reflection utilities, and built-in literal types available in `kyro`.

**Also look at:**

* [`std`](std/index.md): built-in standard library (native in rust).
* [`lib`](lib/index.md): modules under `lib/` written in kyro.

## Built-in Types

### Lists (`[...]`)
Lists are ordered, mutable collections of values.
* **Internal Representation:** `Value::List` variant wrapping a heap-allocated `Rc<RefCell<Vec<Value>>>`.

#### Syntax & Manipulation
```rust
// Literal construction
var list = [1, 2, 3];

// Subscript indexing
var first = list[0];

// Index assignment
list[1] = "mutated";
```

#### Bound Methods
* **`.len()`**: Returns the number of elements in the list.
* **`.push(val)`**: Appends a value to the end of the list.
* **`.pop()`**: Removes and returns the last element of the list.

---

### Dictionaries (`{...}`)
Dictionaries are mutable key-value maps.
* **Internal Representation:** `Value::Dict` variant wrapping an `Rc<RefCell<HashMap<String, Value>>>`.

#### Syntax & Manipulation
```rust
// Literal construction
var dict = {"key": "val"};

// Subscript indexing
var value = dict["key"];

// Index assignment
dict["new_key"] = 42;
```

#### Bound Methods
* **`.len()`**: Returns the number of key-value pairs.
* **`.keys()`**: Returns a list of all keys in the dictionary.
* **`.remove(key)`**: Removes the specified key and returns its associated value.

---

### Strings
Strings represent UTF-8 encoded text sequences.
* **Internal Representation:** `Value::String` variant.

#### Bound Methods
* **`.len()`**: Returns the character length of the string.
* **`.slice(start, end)`**: Returns a substring from the `start` index up to (but excluding) the `end` index.
* **`.split(separator)`**: Splits the string into a list of substrings based on the provided separator.

---

### Numbers
Numbers represent numeric values in Kyro.
* **Internal Representation:** `Value::Number` variant.

#### Bound Methods
* **`.floor()`**: Rounds the number down to the nearest integer.
* **`.ceil()`**: Rounds the number up to the nearest integer.
* **`.round()`**: Rounds the number to the nearest integer.
* **`.abs()`**: Returns the absolute value of the number.
* **`.to_string()`**: Converts the numeric value to its string representation.

---

## Core Globals & Reflection

### `id(item)`
Returns the unique memory address of the given item. 
* **Backend Implementation:** `IdFn` Rust struct.

**Usage:**
```rust
var x = [1, 2, 3];
print(id(x)); // Prints the memory address of the list
```

### `dir(item)`
Returns a list of strings containing all keys, methods, and attributes associated with the specified item.
* **Backend Implementation:** `DirFn` Rust struct.

**Usage:**
```rust
var list = [1, 2];
print(dir(list)); // ["len", "push", "pop", ...]
```

### `use(module_name)`
Loads a standard library module. All standard library modules in Kyro are namespace-isolated and must be explicitly imported. Calling `use` returns a module instance containing its native functions and values.

**Backend Implementation:** `Use` Rust struct. Returns a `KyroInstance` wrapping a dynamic `KyroClass` whose fields point to native `KyroCallable` structs.

**Usage:**
```rust
var math = use("math");
print(math.floor(3.14));
```

### `__name__`
A pre-loaded global variable storing the current module's name. For the primary entrypoint script, this evaluates to the string `"__main__"`.

**Usage:**
```rust
if __name__ == "__main__" {
    print("Running as main script");
}
```

### `instance.__class__`
Accessing `__class__` on any class instance returns its underlying class definition as a class object. This allows you to inspect class metadata dynamically.

**Usage:**
```rust
// instance.__class__.__name__ evaluates to the PascalCase class name
var class_name = my_instance.__class__.__name__; 
```

### `callable.__name__`
Accessing `__name__` on any callable object (such as a class or function) returns its string identifier.

**Usage:**
```rust
print(my_function.__name__); // "my_function"
```

