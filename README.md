# kyro

`kyro` is toy language inspired by [The Lox Language](https://craftinginterpreters.com/the-lox-language.html) built in rust, it uses similar base components but diverts from implementation. 

The [GRAMMAR.md](./GRAMMAR.md) features uses grammaer rules for the language, you can read [examples](./examples/) here.

### Hello, world!

```javascript
// main.kyro
var io = use("std:io")

fn main(){
    io.println("hello, world!");
}

main();
```

```bash
$cargo run main.kyro
```

## Features

### 1. syntax and keywords

* **function declarations (`fn`)**: functions are declared using the `fn` keyword.
* **output statement (`echo`)**: the built-in printing statement is `echo` (freeing up the `print` identifier for modular callables).
* **oop context**: class methods refer to the current instance using `this`.
* **inheritance**: subclasses inherit from parent classes using `<` and refer to overridden parent methods using the `super` keyword.

---

### 2. built-in primitives

* **lists (`[...]`)**: 
  * literal construction: `var list = [1, 2, 3];`
  * subscript indexing: `list[0]`
  * index assignment: `list[1] = "mutated"`
  * bound methods: `.len()`, `.push(val)`, `.pop()`
* **dictionaries (`{...}`)**:
  * literal construction: `var dict = {"key": "val"};`
  * subscript indexing: `dict["key"]`
  * index assignment: `dict["new_key"] = 42`
  * bound methods: `.len()`, `.keys()`, `.remove(key)`
* **strings**:
  * bound methods: `.len()`, `.slice(start, end)`, `.split(separator)`
* **numbers**:
  * bound methods: `.floor()`, `.ceil()`, `.round()`, `.abs()`, `.to_string()`

---

### 3. standard library namespaces

all standard library modules are namespace-isolated and must be loaded explicitly using the `use()` function.

* **`use("std:io")`**:
  * `print(format)`: prints an interpolated string without a trailing newline.
  * `println(format)`: prints an interpolated string with a trailing newline.
  * `input(prompt)`: displays a prompt, flushes stdout, and reads a line from stdin as a string.
* **`use("std:time")`**:
  * `clock()`: returns a fractional number representing seconds since the unix epoch.
  * `now()`: evaluates a fast, zero-dependency gregorian calendar algorithm and returns a dictionary with `"year"`, `"month"`, `"day"`, `"hour"`, `"minute"`, and `"second"` fields.
  * `format(ts, format_str)`: formats a numeric timestamp using specifiers like `%Y-%m-%d %H:%M:%S`.
* **`use("std:fs")`**:
  * `read_file(path)`: reads an entire file on disk and returns its contents as a string.
  * `write_file(path, content)`: writes a string to a file, creating it if missing or overwriting it if present.
  * `exists(path)`: returns a boolean representing whether a file or directory exists at the path.
  * `remove_file(path)`: deletes a file from the disk.
* **`use("std:core")`**:
  * `version`: a static attribute returning the compiler version string.
  * `to_string(value)`: casts any value type to a string.
  * `to_number(value)`: casts a string, boolean, or nil value to a number.
  * `info()`: returns a class instance of type `LanguageInfo` containing metadata fields.

---

### 4. advanced compilation & runtime features

* **compiled string interpolation**: format strings containing `${expression}` are compiled at parse-time into `Expr::Interpolate` AST nodes rather than being processed dynamically. any valid expression can be nested inside the braces and evaluated cleanly.
* **lexical scope resolution**: a static resolver pass traverses the compiled AST before execution to establish local variable bindings, validate scopes, check early return positions, and map exact scope distances.
* **isolated module imports**: calling `use("user_file.kyro")` dynamically compiles, resolves, and runs the target file in a temporary, isolated environment. it extracts all definitions from that scope, bundles them inside a namespace instance, and returns it.
* **structured exception handling**: 
  * `throw expression;`: dynamically raises custom runtime errors.
  * `try { ... } catch (err) { ... }`: catches native runtime exceptions and custom throws.
  * **oop exceptions**: supports throwing and catching any custom class instances or primitives, allowing caught exceptions to preserve full property access (e.g., `err.status` or `err[0]`).
* **centralized contextual diagnostics**: lexical, grammatical, variable resolution, and interpreter errors are caught and formatted inside a central diagnostics engine. it outputs a bold colorized error header, the offending file line, and a red caret pointer (`^`) aligned directly underneath the offending token.