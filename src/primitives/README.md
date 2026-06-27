# kyro primitive methods

this directory contains the native method implementations for kyro's built-in primitive data types: lists, dictionaries, strings, and numbers.

---

## how it works

when dot-notation is accessed on a primitive value (e.g., `list.len()`), the interpreter's `visit_get` method delegates to a dispatcher function inside this folder (such as `get_list_method`).

instead of defining bulky custom structs and `KyroCallable` trait implementations for every single method, we define a single, generic `PrimitiveMethod` struct inside `mod.rs`. this struct implements `KyroCallable` using a boxed closure. 

this allows every specific primitive method inside this folder to be written as a standard, standalone rust function. these functions simply capture the primitive's state (by value or pointer) and return a configured `PrimitiveMethod` instance, keeping the codebase modular and boilerplate-free.

---

## primitives reference

### lists (`list.rs`)
interacts with dynamic arrays.
* `len()`: returns the number of elements in the list.
* `push(value)`: appends an element to the end of the list.
* `pop()`: removes and returns the last element from the list.

```javascript
var list = [10, 20];
list.push(30);
echo list.len();
echo list.pop();
```

### dictionaries (`dict.rs`)
interacts with key-value map structures. keys are automatically stringified during lookups.
* `len()`: returns the number of key-value pairs in the dictionary.
* `keys()`: returns a list containing all keys present in the dictionary.
* `remove(key)`: deletes a key-value entry and returns the removed value.

```javascript
var dict = {"name": "alice", "role": "dev"};
echo dict.len();
echo dict.keys();
echo dict.remove("role");
```

### strings (`string.rs`)
interacts with string slices. operations are safe against multi-byte utf-8 characters.
* `len()`: returns the character count of the string.
* `slice(start, end)`: returns a substring from the start index (inclusive) to the end index (exclusive).
* `split(separator)`: splits the string by a separator string and returns a list of parts.

```javascript
var text = "a,b,c";
var comma = ",";
var list = text.split(comma);
echo list;
echo text.slice(0, 3);
```

### numbers (`number.rs`)
interacts with double-precision floating-point numbers.
* `floor()`: rounds the number down to the nearest integer.
* `ceil()`: rounds the number up to the nearest integer.
* `round()`: rounds the number to the nearest integer.
* `abs()`: returns the absolute value of the number.
* `to_string()`: converts the number to its string representation.

```javascript
var num = -3.14;
echo num.abs();
echo num.floor();
```

---

## how to add a new primitive method

to add a new method to a primitive type (for example, adding `.reverse()` to lists):

1. **create the standard rust function**: open `list.rs` and write a standalone function that accepts the list pointer, captures it in a closure, and returns a `PrimitiveMethod`:
   ```kyro
   fn reverse(list: Rc<RefCell<Vec<Value>>>) -> PrimitiveMethod {
       PrimitiveMethod::new("reverse", 0, move |_, _| {
           list.borrow_mut().reverse();
           Ok(Value::Nil)
       })
   }
   ```
2. **register in the dispatcher**: update the `get_list_method` dispatcher match block inside `list.rs` to map the name to your new function:
   ```kyro
   pub fn get_list_method(list: Rc<RefCell<Vec<Value>>>, name: &Token) -> Result<Value, RuntimeError> {
       match name.lexeme.as_str() {
           "len" => Ok(Value::Callable(Rc::new(len(list)))),
           "push" => Ok(Value::Callable(Rc::new(push(list)))),
           "pop" => Ok(Value::Callable(Rc::new(pop(list)))),
           "reverse" => Ok(Value::Callable(Rc::new(reverse(list)))),
           _ => Err(RuntimeError::new(
               name.clone(),
               format!("undefined list method '{}'.", name.lexeme),
           )),
       }
   }
   ```