# kyro grammar reference

this document defines the formal syntactic grammar of the kyro language using a modified context-free bnf (backus-naur form) notation, accompanied by practical code examples.

---

## lexical tokens and keywords

kyro keywords are reserved and cannot be used as identifiers.

```text
keyword     -> "and" | "class" | "else" | "false" | "fn" | "for" 
             | "if" | "in" | "nil" | "or" | "echo" | "return" 
             | "super" | "this" | "true" | "var" | "while" 
             | "try" | "catch" | "throw" | "break" | "continue"

symbol      -> "(" | ")" | "{" | "}" | "[" | "]" | ":" | "," 
             | "." | "-" | "+" | ";" | "/" | "*" | "!" | "!=" 
             | "=" | "==" | "<" | "<=" | ">" | ">=" | "&" | "|" 
             | "^" | "~" | "<<" | ">>"
```

---

## program and declarations

a kyro program is a sequence of declarations executed from top to bottom.

```text
program     -> declaration* eof
declaration -> classdecl | fundecl | vardecl | statement
```

### class declaration
defines a class, with optional subclass inheritance using `<`.
```text
classdecl   -> "class" identifier ( "<" identifier )? "{" function* "}"
```
```javascript
class pastry {
  __init__(flavour) {
    this.flavour = flavour;
  }
}

class doughnut < pastry {}
```

### function declaration
defines a named block of code that can accept parameters. parameters can optionally declare default fallback values.
```text
fundecl     -> "fn" identifier "(" parameters? ")" block
parameters  -> parameter ( "," parameter )*
parameter   -> identifier ( "=" expression )?
```
```javascript
fn multiply(x, y = 2) {
  return x * y;
}
```

### variable declaration
declares a variable (with optional initial value) or binds multiple values using list and dictionary sequence destructuring.
```text
vardecl     -> "var" identifier ( "=" expression )? ";"
             | "var" "[" identifier ( "," identifier )* "]" "=" expression ";"
             | "var" "{" identifier ( "," identifier )* "}" "=" expression ";"
```
```javascript
var x = 10;
var [first, second] = ["apple", "banana"];
var { name, role } = { "name": "bob", "role": "dev" };
```

---

## statements

statements represent actions that do not produce values.

```text
statement   -> exprstmt | echostmt | block | ifstmt | whilestmt 
             | forstmt | forinstmt | returnstmt | trycatchstmt 
             | throwstmt
```

### expression statement
evaluates an expression and discards the result.
```text
exprstmt    -> expression ";"
```
```javascript
math.square(5);
```

### echo statement
evaluates an expression and prints its string representation followed by a newline.
```text
echostmt    -> "echo" expression ";"
```
```javascript
echo "hello world";
```

### block statement
defines a nested lexical scope for its inner declarations.
```text
block       -> "{" declaration* "}"
```
```javascript
{
  var local = "nested";
  echo local;
}
```

### if statement
conditional execution.
```text
ifstmt      -> "if" "(" expression ")" statement ( "else" statement )?
```
```javascript
if (x > 10) {
  echo "greater";
} else {
  echo "less or equal";
}
```

### while statement
pre-test loop execution.
```text
whilestmt   -> "while" "(" expression ")" statement
```
```javascript
while (x > 0) {
  x = x - 1;
}
```

### for statement
a loop construct containing initializer, condition, and increment. parsed and desugared into a `while` loop under the hood.
```text
forstmt     -> "for" "(" ( vardecl | exprstmt | ";" )
                         expression? ";"
                         expression? ")" statement
```
```javascript
for (var i = 0; i < 5; i = i + 1) {
  echo i;
}
```

### for-in statement
loops over elements of lists, keys of dictionaries, or custom iterators implementing a `__next__()` method.
```text
forinstmt   -> "for" "(" "var" identifier "in" expression ")" statement
```
```javascript
var list = ["a", "b", "c"];
for (var val in list) {
  print(val);
}
```

### return statement
exits a function call, optionally returning an evaluated value.
```text
returnstmt  -> "return" expression? ";"
```
```javascript
return true;
```

### try-catch statement
gracefully catches runtime exceptions thrown inside the `try` block.
```text
trycatchstmt -> "try" block "catch" "(" identifier ")" block
```
```javascript
try {
  var value = list[10];
} catch (err) {
  echo "failed: " + err;
}
```

### throw statement
intentionally raises a runtime exception with an evaluated value (strings, class instances, or primitives).
```text
throwstmt   -> "throw" expression ";"
```
```javascript
throw "something went wrong";
```

---

## expressions

expressions evaluate to a single runtime value.

```text
expression  -> assignment
```

### assignment
assigns a value to a variable, an instance property, or a subscript collection index.
```text
assignment  -> ( call "." )? identifier "=" assignment
             | call "[" expression "]" "=" assignment
             | logic_or
```
```javascript
name = "bob";
instance.field = 42;
list[2] = "mutated";
```

### logic or
evaluates boolean short-circuiting disjunction.
```text
logic_or    -> logic_and ( "or" logic_and )*
```
```javascript
true or false;
```

### logic and
evaluates boolean short-circuiting conjunction.
```text
logic_and   -> bitwise_or ( "and" bitwise_or )*
```
```javascript
is_valid and x > 0;
```

### bitwise or
computes bitwise OR on numerical operands.
```text
bitwise_or  -> bitwise_xor ( "|" bitwise_xor )*
```
```javascript
var flags = status | 4;
```

### bitwise xor
computes bitwise XOR on numerical operands.
```text
bitwise_xor -> bitwise_and ( "^" bitwise_and )*
```
```javascript
var difference = mask1 ^ mask2;
```

### bitwise and
computes bitwise AND on numerical operands.
```text
bitwise_and -> equality ( "&" equality )*
```
```javascript
var is_active = status & 1;
```

### equality
comparisons for absolute equivalence or difference.
```text
equality    -> comparison ( ( "!=" | "==" ) comparison )*
```
```javascript
x == y;
"apple" != "banana";
```

### comparison
relational comparison operations.
```text
comparison  -> bitwise_shift ( ( ">" | ">=" | "<" | "<=" ) bitwise_shift )*
```
```javascript
x < 100;
```

### bitwise shift
computes arithmetic left and right bitwise shifts.
```text
bitwise_shift -> term ( ( "<<" | ">>" ) term )*
```
```javascript
var shifted = value << 2;
```

### term
addition and subtraction.
```text
term        -> factor ( ( "-" | "+" ) factor )*
```
```javascript
var sum = x + y;
```

### factor
multiplication and division.
```text
factor      -> unary ( ( "/" | "*" ) unary )*
```
```javascript
var product = x * y;
```

### unary
logical negation, arithmetic inversion, and bitwise tilde negation.
```text
unary       -> ( "!" | "-" | "~" ) unary | call
```
```javascript
!is_ready;
-5;
~mask;
```

### call
function calls (with optional keyword arguments), class instantiations, property gets, and subscript indexing accesses.
```text
call        -> primary ( "(" arguments? ")" | "." identifier | "[" expression "]" )*
arguments   -> argument ( "," argument )*
argument    -> expression | identifier "=" expression
```
```javascript
math.square(x);
json.dumps(config, indent = 4);
list[0];
```

### primary
base literals, parenthesized groupings, namespaces, structured collections, and anonymous functions.
```text
primary     -> NUMBER | STRING | "true" | "false" | "nil" | "this" | identifier
             | "super" "." identifier
             | "(" expression ")"
             | listliteral | dictliteral
             | lambda
```
```javascript
true;
this.field;
super.method();
(x + y);
```

### anonymous function (lambda)
declares a named or nameless inline closure expression.
```text
lambda      -> "fn" "(" parameters? ")" block
```
```javascript
var double = fn(x) { return x * 2; };
```

### list literal
constructs a list object with optional initial elements.
```text
listliteral -> "[" ( expression ( "," expression )* )? "]"
```
```javascript
var fruits = ["apple", "banana"];
```

### dictionary literal
constructs a key-value dictionary object with optional initial key-value mappings.
```text
dictliteral -> "{" ( expression ":" expression ( "," expression ":" expression )* )? "}"
```
```javascript
var user = { "name": "bob", "role": "dev" };
```

---

## string interpolation syntax

kyro evaluates dynamic format strings at compile-time by parsing nested braces.

```text
string      -> '"' ( char* | "${" expression "}" )* '"'
```
```javascript
var label = "progress";
var percent = 50;

// compiled at parse-time as a single format tree
echo "${label}: ${percent}%"; // progress: 50%
```