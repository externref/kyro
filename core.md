---
title: core
---

The core global functions, reflection utilities, and built-in literal types available in `kyro`.

**Also look at:**

* [`std`](std/index.md): built-in standard library (native in rust).
* [`lib`](lib/index.md): modules under `lib/` written in kyro.

## Built-in Types

Each built-in type has a corresponding global class object in the namespace (such as `List`, `Dict`, `String`, `Number`, `Bool`, `Nil`, `Callable`, and `Class`) which can be used for runtime type-checking and reflections.

### Lists (`[...]`)
Lists are ordered, mutable collections of values.
* **Internal Representation:** `Value::List` variant wrapping a heap-allocated `Rc<RefCell<Vec<Value>>>`.
* **Namespace Class:** `List`

#### Syntax & Manipulation
```kyro
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

### Dictionaries (`{...}`)
Dictionaries are mutable key-value maps.
* **Internal Representation:** `Value::Dict` variant wrapping an `Rc<RefCell<HashMap<String, Value>>>`.
* **Namespace Class:** `Dict`

#### Syntax & Manipulation
```kyro
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

### Strings
Strings represent UTF-8 encoded text sequences.
* **Internal Representation:** `Value::String` variant.
* **Namespace Class:** `String`

#### Bound Methods
* **`.len()`**: Returns the character length of the string.
* **`.slice(start, end)`**: Returns a substring from the `start` index up to (but excluding) the `end` index.
* **`.split(separator)`**: Splits the string into a list of substrings based on the provided separator.

### Numbers
Numbers represent numeric values in kyro.
* **Internal Representation:** `Value::Number` variant.
* **Namespace Class:** `Number`

#### Bound Methods
* **`.floor()`**: Rounds the number down to the nearest integer.
* **`.ceil()`**: Rounds the number up to the nearest integer.
* **`.round()`**: Rounds the number to the nearest integer.
* **`.abs()`**: Returns the absolute value of the number.
* **`.to_string()`**: Converts the numeric value to its string representation.

---

## Built-in Exception Classes

kyro features a structured, object-oriented exception hierarchy. The interpreter instantiates and throws these classes dynamically when runtime errors occur.

### Exception
The base class of the entire exception hierarchy. All built-in and user-defined exception structures inherit from `Exception`.
* **Properties:**
    * `message` *(String)*: A detailed message describing the cause of the error.
* **Methods:**
    * `init(message)`: Automatically assigns the provided message.
    * `__str__()`: Formats and returns the exception message as `ClassName: Message`.

### ValueError (inherits from `Exception`)
Thrown when an operation or function receives an argument of the correct type but an inappropriate value (e.g. attempting to convert an invalid string format using `to_number()`).

### TypeError (inherits from `Exception`)
Thrown when an operation or function is applied to an object of an inappropriate type (e.g., trying to perform mathematics on strings, or calling a non-callable value).

### AttributeError (inherits from `Exception`)
Thrown when an attribute reference or assignment fails on an object or class instance (e.g., accessing an undefined class property).

### IndexError (inherits from `Exception`)
Thrown when a subscript index is out of the bounds of a list.

---

## String Representation (`__str__`) Protocol

Whenever a class instance is printed using `print()`, `println()`, or converted via `to_string()`, the interpreter checks if the instance has a defined `__str__()` method. If present, it executes `__str__()` and uses the returned string representation.

```kyro
class CustomItem {
    init(val) {
        this.val = val;
    }
    __str__() {
        return "CustomItem(" + this.val.to_string() + ")";
    }
}

var item = CustomItem(100);
print(item); // Prints: "CustomItem(100)"
```

---

## Core Globals & Reflection

### `id(item)`
Returns the unique memory address of the given item. 
* **Backend Implementation:** `IdFn` Rust struct.

**Usage:**
```kyro
var x = [1, 2, 3];
print(id(x)); // Prints the memory address of the list
```

### `dir(item)`
Returns a list of strings containing all keys, methods, and attributes associated with the specified item.
* **Backend Implementation:** `DirFn` Rust struct.

**Usage:**
```kyro
var list = [1, 2];
print(dir(list)); // ["len", "push", "pop", ...]
```

### `is_instance(item, class)`
Checks whether the provided item is an instance of the specified class (or inherits from it). Supports both custom class structures and native built-in types.
* **Backend Implementation:** `IsInstanceFn` Rust struct.

**Usage:**
```kyro
// Checking primitive types using namespace classes
print(is_instance(42, Number));      // true
print(is_instance("hello", String));  // true

// Checking custom OOP hierarchies
class Animal {}
class Dog < Animal {}

var poppy = Dog();
print(is_instance(poppy, Dog));      // true
print(is_instance(poppy, Animal));   // true (honors inheritance)
print(is_instance(poppy, List));     // false
```

### `use(module_name)`
Loads a standard library module. All standard library modules in kyro are namespace-isolated and must be explicitly imported. Calling `use` returns a module instance containing its native functions and values.

**Backend Implementation:** `Use` Rust struct. Returns a `KyroInstance` wrapping a dynamic `KyroClass` whose fields point to native `KyroCallable` structs.

**Usage:**
```kyro
var math = use("math");
print(math.floor(3.14));
```

### `__name__`
A pre-loaded global variable storing the current module's name. For the primary entrypoint script, this evaluates to the string `"__main__"`.

**Usage:**
```kyro
if __name__ == "__main__" {
    print("Running as main script");
}
```

### `instance.__class__`
Accessing `__class__` on any class instance returns its underlying class definition as a class object. This allows you to inspect class metadata dynamically.

**Usage:**
```kyro
// instance.__class__.__name__ evaluates to the PascalCase class name
var class_name = my_instance.__class__.__name__; 
```

### `callable.__name__`
Accessing `__name__` on any callable object (such as a class or function) returns its string identifier.

**Usage:**
```kyro
print(my_function.__name__); // "my_function"
```