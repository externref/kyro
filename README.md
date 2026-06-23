# kyro

kyro is a tree-walk interpreter for the lox language written in rust. it is adapted from the scripting language designed in crafting interpreters, but diverges to include custom syntax adjustments and modern features like isolated module namespaces, list/dictionary primitives, and dynamic string interpolation.

## features

* **rust-backed architecture**: built on top of rust's memory model using reference counting and interior mutability to manage environments and lexical scopes.
* **static resolution pass**: evaluates variable declarations before execution to prevent lexical scope leaks and resolve variable binding depths.
* **isolated module loading**: allows running external files and importing built-in system namespaces cleanly without scope pollution.
* **list and dictionary primitives**: built-in literals with subscript indexing, index assignments, and dynamic method dispatches.
* **syntactic refinements**: declares functions using `fn` instead of `fun`, and uses `echo` as the built-in output statement to free up `print` as a modular function.

## standard library namespaces

there are no default global functions other than `use()`. calling `use()` returns a module namespace instance containing functions.

* **use("io")**: loads standard input/output functions.
  * `print(format)`: prints an interpolated string without a trailing newline.
  * `println(format)`: prints an interpolated string with a trailing newline.
  * `input(prompt)`: displays a prompt, flushes stdout, and reads a line from stdin as a string.
* **use("time")**: loads system time functions.
  * `clock()`: returns the current system timestamp as a fractional number.

## examples

### basic operations and standard io
to use i/o functions, you must import the `"io"` module first.

```rust
// main.kyro
var io = use("io");

var name = io.input("enter your name: ");
io.println("hello ${name}, welcome to kyro!");
```

### lists and dictionaries
kyro has built-in bracket-based data structures with OOP-style methods:

```rust
var list = [1, 2, "three"];
list[1] = "mutated";
list.push(4);
echo list; // [1, mutated, three, 4]
echo list.pop(); // 4

var dict = { "name": "bob", "age": 25 };
dict["job"] = "developer";
echo dict["job"]; // developer
echo dict.keys(); // [name, age, job]
```

### function declarations
functions are declared using the `fn` keyword.

```rust
fn square(n) {
  return n * n;
}

echo square(5); // 25
```

### object-oriented programming with inheritance
kyro supports classes, initializers, bound methods, the `this` context, and inheritance with superclass overrides.

```rust
var io = use("io");

class Pastry {
  init(flavour) {
    this.flavour = flavour;
  }

  describe() {
    io.println("a delicious ${this.flavour} pastry.");
  }
}

class Doughnut < Pastry {
  describe() {
    super.describe();
    io.println("it is fried and glazed.");
  }
}

var treat = Doughnut("chocolate");
treat.describe();
```

### loading custom module files
modules are evaluated inside their own environment so that they do not pollute the parent scope.

```rust
// math.kyro
fn add(a, b) {
  return a + b;
}

fn multiply(a, b) {
  return a * b;
}
```

```rust
// main.kyro
var io = use("io");
var math = use("math.kyro");

var sum = math.add(5, 10);
io.println("sum is ${sum}"); // sum is 15
```

## getting started

ensure you have rust and cargo installed.

### run a script
to execute a local script file:

```bash
cargo run -- path/to/script.kyro
```

### run the interactive repl
to launch the interactive prompt shell:

```bash
cargo run
```