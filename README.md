# kyro
<p align="center">
  <img src="docs/assets/icon.svg" width="120" alt="kyro logo"><br/><br>
 <a href="https://github.com/externref/kyro/blob/main/LICENSE">
    <img src="https://img.shields.io/github/license/externref/kyro?style=flat-square&logo=github&logoColor=white&color=09090b&labelColor=27272a" alt="License">
  </a>
  <a href="https://github.com/externref/kyro/stargazers">
    <img src="https://img.shields.io/github/stars/externref/kyro?style=flat-square&logo=github&logoColor=white&color=09090b&labelColor=27272a" alt="GitHub stars">
  </a>
  <a href="https://github.com/externref/kyro/commits">
    <img src="https://img.shields.io/github/last-commit/externref/kyro?style=flat-square&logo=github&logoColor=white&color=09090b&labelColor=27272a" alt="GitHub last commit">
  </a>
</p>


Kyro is a lightweight, toy programming language built in Rust, inspired by the [Lox language from Crafting Interpreters](https://craftinginterpreters.com/the-lox-language.html). It features a tree-walk interpreter architecture with modern enhancements such as a static resolution pass, compiled string interpolation, and a namespace-isolated standard library.

The language is designed for simplicity and extensibility, supporting object-oriented programming (OOP), structured exception handling, and first-class collection types like lists and dictionaries

The [GRAMMAR.md](./GRAMMAR.md) features used grammar rules for the language, you can read [examples](examples/) here.

## installation

### unix (linux, macos, wsl)

1. make the installation script executable:
   ```bash
   chmod +x install.sh
   ```

2. execute the script to compile the binary, establish the folder structures, and configure the path variables:
   ```bash
   ./install.sh
   ```

3. reload your shell configuration file to apply the environment paths immediately:
   ```bash
   source ~/.bashrc
   ```
   *(or `source ~/.zshrc` if you are using zsh)*

---

### windows

1. open powershell and navigate to the project directory.

2. bypass the local execution policy for the current terminal session:
   ```powershell
   Set-ExecutionPolicy -ExecutionPolicy Bypass -Scope Process
   ```

3. execute the script to compile the binary, copy the executable, and write the path variables:
   ```powershell
   .\install.ps1
   ```

4. restart your powershell or command prompt window to activate the path changes.


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
$ cargo run main.kyro
```

## features

### 1. syntax and keywords

* **function declarations (`fn`)**: functions are declared using the `fn` keyword.
* **output statement (`echo`)**: the built-in printing statement is `echo` (freeing up the `print` identifier for modular callables).
* **oop context**: class methods refer to the current instance using the `this` keyword, evaluated inside the `Interpreter` struct via the `visit_this` visitor method.
* **inheritance**: subclasses inherit from parent classes using `<` and refer to overridden parent methods using the `super` keyword.

---

### 2. built-in primitives

* **lists (`[...]`)**: 
  * literal construction: `var list = [1, 2, 3];`
  * subscript indexing: `list[0]`
  * index assignment: `list[1] = "mutated"`
  * bound methods: `.len()`, `.push(val)`, `.pop()`. represented by the `Value::List` enum variant wrapping a heap-allocated `Rc<RefCell<Vec<Value>>>`.
* **dictionaries (`{...}`)**:
  * literal construction: `var dict = {"key": "val"};`
  * subscript indexing: `dict["key"]`
  * index assignment: `dict["new_key"] = 42`
  * bound methods: `.len()`, `.keys()`, `.remove(key)`. represented by the `Value::Dict` enum variant wrapping `Rc<RefCell<HashMap<String, Value>>>`.
* **strings**:
  * bound methods: `.len()`, `.slice(start, end)`, `.split(separator)`. represented by the `Value::String` enum variant.
* **numbers**:
  * bound methods: `.floor()`, `.ceil()`, `.round()`, `.abs()`, `.to_string()`. represented by the `Value::Number` enum variant.

---

### 3. standard library namespaces

all standard library modules are namespace-isolated and must be loaded explicitly using the `use()` function (backed by the `Use` rust struct). calling `use` returns a `KyroInstance` wrapping a dynamic `KyroClass` whose fields point to native `KyroCallable` structs.

* **`use("std:io")`** (maps to `src/stdlib/io.rs`):
  * `print(format)`: prints an interpolated string without a trailing newline, backed by the `Print` rust struct.
  * `println(format)`: prints an interpolated string with a trailing newline, backed by the `Println` rust struct.
  * `input(prompt)`: displays a prompt, flushes stdout, and reads a line from stdin as a string, backed by the `Input` rust struct.
* **`use("std:time")`** (maps to `src/stdlib/time.rs`):
  * `clock()`: returns a fractional number representing seconds since the unix epoch, backed by the `Clock` rust struct.
  * `now()`: evaluates a fast, zero-dependency gregorian calendar algorithm and returns a dictionary with `"year"`, `"month"`, `"day"`, `"hour"`, `"minute"`, and `"second"` fields, backed by the `Now` rust struct.
  * `format(ts, format_str)`: formats a numeric timestamp using specifiers like `%Y-%m-%d %H:%M:%S`, backed by the `Format` rust struct.
* **`use("std:fs")`** (maps to `src/stdlib/fs.rs`):
  * `read_file(path)`: reads an entire file on disk and returns its contents, backed by the `ReadFile` rust struct.
  * `write_file(path, content)`: writes a string to a file, creating it if missing or overwriting it, backed by the `WriteFile` rust struct.
  * `exists(path)`: returns a boolean representing whether a file or directory exists, backed by the `Exists` rust struct.
  * `remove_file(path)`: deletes a file from the disk, backed by the `RemoveFile` rust struct.
* **`use("std:util")`** (maps to `src/stdlib/util.rs`):
  * `to_string(value)`: casts any value type to a string, backed by the `ToStringFn` rust struct.
  * `to_number(value)`: casts a string, boolean, or nil value to a number, backed by the `ToNumber` rust struct.
  * `info()`: returns a class instance of type `LanguageInfo` containing metadata fields, backed by the `InfoFn` rust struct.
* **`use("std:os")`** (maps to `src/stdlib/os.rs`):
  * `args()`: returns a list of string arguments used to run the current process, backed by the `ArgsFn` rust struct.

---

### 4. global runtime reflection

* **`id(item)`**: returns the unique memory address of an item, backed by the `IdFn` rust struct.
* **`dir(item)`**: returns a list containing all keys, methods, and attributes of the item, backed by the `DirFn` rust struct.
* **`__name__`**: a pre-loaded global variable that stores the module name. for the currently running main script, it evaluates to `"__main__"`.
* **`instance.__class__`**: accessing `__class__` on any class instance returns its underlying class definition as a class object, allowing you to chain `.name` lookups (e.g. `instance.__class__.__name__` evaluates to the `PascalCase` class name).
* **`callable.__name__`**: accessing `__name__` on any class or function callable returns its string identifier.

---

### 5. advanced compilation & runtime features

* **compiled string interpolation**: format strings containing `${expression}` are compiled at parse-time into `Expr::Interpolate` AST nodes rather than being processed dynamically. any valid expression can be nested inside the braces and evaluated cleanly.
* **lexical scope resolution**: a static resolver pass (implemented in the `Resolver` rust struct) traverses the compiled AST before execution to establish local variable bindings, validate scopes, check early return positions, and map exact scope distances.
* **isolated module imports**: calling `use("user_file.kyro")` dynamically compiles, resolves, and runs the target file in an isolated nested `Environment` (wrapped inside `EnvRef`). it extracts all definitions from that scope, bundles them inside a `KyroInstance`, and returns it.
* **structured exception handling**: 
  * `throw expression;`: dynamically raises custom runtime errors, represented by the `RuntimeError::Error` enum variant.
  * `try { ... } catch (err) { ... }`: catches native runtime exceptions and custom throws.
  * **oop exceptions**: supports throwing and catching any custom class instances (such as a custom `HttpException` class) or primitives, allowing caught exceptions to preserve full property access (e.g., `err.status` or `err[0]`).
* **centralized contextual diagnostics**: lexical, grammatical, variable resolution, and interpreter errors are caught and formatted inside a central diagnostics engine implemented inside the `Kyro` rust struct. it outputs a bold colorized error header, the offending file line, and a red caret pointer (`^`) aligned directly underneath the offending token.
