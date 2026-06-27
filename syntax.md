The language uses a C-style syntax with modern language features, including function declarations with `fn`, `class`-based OOP with inheritance using `<`, and structured exception handling with `try/catch/throw`.

## Declarations

### Variables
Variables are declared with the `var` keyword and can be optionally initialized. Uninitialized variables default to `nil`. kyro also supports sequence destructuring for both lists and dictionaries:

```kyro
var x = 10;
var uninitialized;

// List and Dictionary destructuring
var [first, second] = ["apple", "banana"];
var { name, role } = { "name": "bob", "role": "dev" };
```

### Functions
Functions are defined using the `fn` keyword, followed by a name, parameter list in parentheses, and a body block. Parameters can optionally declare default fallback values using the `=` operator.

```kyro
fn multiply(x, y = 2) {
    return x * y;
}

// Call using purely positional arguments
multiply(2, 3); // 6 

// Call relying on the default parameter value for y
multiply(5);    // 10

// Call using keyword arguments out of order
multiply(y = 4, x = 3); // 12
```

Functions can also be declared anonymously as expressions (lambdas) and assigned to variables or passed as callbacks:

```kyro
var double = fn(x) {
    return x * 2;
};

print(double(5)); // 10
```

### Classes
Classes are declared with the `class` keyword and can optionally inherit from a parent class using `<`. Methods are defined within the class body. Object initialization is driven internally by the class’s `__init__` constructor method.

```kyro
var io = use("std:io");

class Animal {
    do_sound(){
        io.println("generic animal sound");
    }
}

class Cat < Animal {
    __init__(name){
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
kyro supports standard C-style control flow structures with conditions enclosed in parentheses, as well as `for-in` iterator loops.

```kyro
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

// standard for loop
for (var i = 0; i < 5; i = i + 1) {
    echo i;
}

// for-in loop (supports Lists, Dicts, and Custom Iterators)
var fruits = ["apple", "banana"];
for (var fruit in fruits) {
    echo fruit;
}
```

### Exception Handling
Exceptions are handled with `try`/`catch` blocks, and can be thrown with any value type.

```kyro
try {
    var value = list[10];
} catch (err) {
    echo "failed: " + err;
}

throw "something went wrong";
```

### Output
The `echo` statement prints values followed by a newline.

```kyro
echo "hello world";
```

## Expressions

### Literals and Collections
kyro supports lists, dictionaries, and string interpolation with compile-time parsing.

```kyro
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
The language uses standard C-style operators for arithmetic, comparison, logic, and bitwise operations (`&`, `|`, `^`, `~`, `<<`, `>>`).

---

/// details | For a quick overview on the supported keywords here's the `TokenType` enum.
```kyro
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
    In,
    Ampersand, Pipe, Caret, Tilde, LessLess, GreaterGreater,
    Eof,
}
```
///