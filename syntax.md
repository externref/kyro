The language uses a C-style syntax with modern language features, including function declarations with `fn`, `class`-based OOP with inheritance using `<`, and structured exception handling with `try/catch/throw`.

## Declarations

### Variables
Variables are declared with the `var` keyword and can be optionally initialized. Uninitialized variables default to `nil`.

```rust
var x = 10;
var uninitialized;
```

### Functions
Functions are defined using the `fn` keyword, followed by a name, parameter list in parentheses, and a body block.

```rust
fn multiply(x, y) {
    return x * y;
}

multiply(2, 3) // 6 
```

### Classes
Classes are declared with the `class` keyword and can optionally inherit from a parent class using `<`. Methods are defined within the class body.

```rust
var io = use("std:io");

class Animal {
    do_sound(){
        io.println("generic animal sound");
    }
}

class Cat < Animal {
    init(name){
        this.name = name;
    }
    do_sound(){
        io.println("${this.name} says meow");
    }
}

fn main(){
    var cat = Cat("bob");
    cat.do_sound();
}
```

## Statements

### Control Flow
Kyro supports standard C-style control flow structures with conditions enclosed in parentheses.

```rust
// if/else
if (x > 10) {
    echo "greater";
} else {
    echo "less or equal";
}

// while loop
while (x > 0) {
    x = x - 1;
}

// for loop
for (var i = 0; i < 5; i = i + 1) {
    echo i;
}
```

### Exception Handling
Exceptions are handled with `try`/`catch` blocks, and can be thrown with any value type.

```rust
try {
    var value = list[10];
} catch (err) {
    echo "failed: " + err;
}

throw "something went wrong";
```

### Output
The `echo` statement prints values followed by a newline.

```rust
echo "hello world";
```

## Expressions

### Literals and Collections
Kyro supports lists, dictionaries, and string interpolation with compile-time parsing.

```rust
// lists
var fruits = ["apple", "banana"];

// dictionaries
var user = { "name": "bob", "role": "dev" };

// string interpolation
var label = "progress";
var percent = 50;
echo "${label}: ${percent}%"; // progress: 50%
```

### Operators
The language uses standard C-style operators for arithmetic, comparison, and logic.

---

/// details | For a quick overview on the supported keywords here's the `TokenType` enum.
```rust
#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
    LeftBracket, RightBracket, Colon,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star,
    Bang, BangEqual, Equal, EqualEqual,
    Greater, GreaterEqual, Less, LessEqual,
    Identifier, String, Number,
    And, Class, Else, False, Fn, For, If, Nil, Or,
    Echo, Return, Super, This, True, Var, While,
    Try, Catch, Throw, 
    Break, Continue,
    Eof,
}
```
///
