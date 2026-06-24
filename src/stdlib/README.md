# kyro standard library (stdlib)

this directory contains the native system modules for the kyro language. 

unlike standard interpreters that clutter the global environment by default, kyro isolates all standard library functions inside dedicated namespaces. you must explicitly import a module using the `use()` function to access its capabilities.

---

## how it works

when you call `use("std:name")` in a script, the interpreter intercepts the import in `mod.rs`. instead of reading a file from disk, it invokes a rust function inside the corresponding module file (e.g., `io::get_module()`) which returns a class instance. the public functions of that module are bound as fields on this instance, allowing you to invoke them using standard dot-notation.

---

## available modules

### std:io
handles terminal input and output operations.
* `print(format)`: prints a formatted string without a trailing newline.
* `println(format)`: prints a formatted string with a trailing newline.
* `input(prompt)`: displays a prompt, flushes stdout, and reads a line of input.

```javascript
var io = use("std:io");
var name = io.input("enter name: ");
io.println("hello ${name}!");
```

### std:time
handles clock cycles and calendar dates.
* `clock()`: returns a fractional number representing seconds since the unix epoch.
* `now()`: returns a dictionary containing `"year"`, `"month"`, `"day"`, `"hour"`, `"minute"`, and `"second"` fields (utc).
* `format(timestamp, format_str)`: formats a numeric timestamp into a string using specifiers like `%y-%m-%d %h:%m:%s`.

```javascript
var io = use("std:io");
var time = use("std:time");

var date = time.now();
io.println("current year: ${date['year']}");

var formatted = time.format(time.clock(), "%y-%m-%d");
io.println("today: ${formatted}");
```

### std:fs
handles local file storage operations.
* `exists(path)`: returns a boolean representing whether a file exists.
* `read_file(path)`: reads a text file and returns its contents.
* `write_file(path, content)`: writes a string to a file, overwriting existing contents.
* `remove_file(path)`: deletes a file from the disk.

```javascript
var io = use("std:io");
var fs = use("std:fs");

if (fs.exists("log.txt")) {
  var data = fs.read_file("log.txt");
  io.println(data);
}
```

### std:util
contains casting and inspection helpers.
* `version`: static string representing the current language version.
* `to_string(value)`: converts any value type to a string.
* `to_number(value)`: parses strings, booleans, or nil values into numbers.
* `info()`: returns a metadata class instance containing language information.

```javascript
var io = use("std:io");
var util = use("std:util");

var parsed = util.to_number("123.45");
io.println("type of conversion: ${parsed}");
```

### std:os
contains operating-system and runtime execution context utilities.
* `args()`: returns a list of string arguments used to run the current process.

```javascript
var io = use("std:io");
var os = use("std:os");

var params = os.args();
io.println("total arguments: ${params.len()}");
```

---

## how to add a new system module

to add a new built-in module (e.g., `"std:sys"`):

1. **create the file**: add `src/stdlib/sys.rs`.
2. **define the module structure**: inside the new file, implement your functions as structs conforming to `kyrocallable` (from `crate::interpreter::callable::kyrocallable`), and expose a public `get_module()` function returning a `value`:
   ```rust
   pub fn get_module() -> Value {
       let class = Rc::new(KyroClass {
           name: "sys".to_string(),
           superclass: None,
           methods: HashMap::new(),
       });
       let mut fields = HashMap::new();
       fields.insert("my_fn".to_string(), Value::Callable(Rc::new(MyFn)));

       let instance = KyroInstance { class, fields };
       Value::Instance(Rc::new(RefCell::new(instance)))
   }
   ```
3. **register in mod.rs**: open `src/stdlib/mod.rs` and:
   * declare the submodule: `pub mod sys;`
   * add the intercept match condition inside `Use::call`:
     ```rust
     if filename == "sys" || filename == "std:sys" {
         return Ok(sys::get_module());
     }
     ```