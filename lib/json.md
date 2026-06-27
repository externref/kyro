---
title: "json"
---

The `lib:json` library module provides utility functions to encode kyro data structures into JSON strings and decode JSON strings back into native kyro objects (such as dictionaries, lists, strings, numbers, booleans, and `nil`).

To use this module, load it using the `lib` prefix:

```javascript
var json = use("lib:json");
```

### loads

```rust
loads(json_str: String) -> Any
```

Parses a JSON-encoded string and returns its representation as a native kyro value.

* **Parameters:**
    * `json_str` *(String)*: The JSON data to parse.
* **Returns:** *(Any)* A kyro dictionary, list, string, number, boolean, or `nil`.

```rust
var json = use("lib:json");

var data = json.loads("{\"name\": \"kyro\", \"version\": 1.0, \"active\": true}");
print(data["name"]);     // "kyro"
print(data["version"]);  // 1.0
print(data["active"]);   // true
```


### dumps

```rust
dumps(value: Any, indent: Number | Nil) -> String
```

Serializes a kyro value into a JSON-formatted string.

* **Parameters:**
    * `value` *(Any)*: The kyro value to serialize.
    * `indent` *(Number | Nil)*: The number of spaces to use for indentation. If set to `nil`, the output will be compact and printed on a single line.
* **Returns:** *(String)* The serialized JSON string.

```rust
var json = use("lib:json");

var config = {
    "port": 8080,
    "tags": ["web", "production"]
};

// Compact serialization
var compact = json.dumps(config, nil);
print(compact); // {"port": 8080, "tags": ["web", "production"]}

// Pretty-printed serialization with 4 spaces of indentation
var pretty = json.dumps(config, 4);
print(pretty);
```


### load

```rust
load(filepath: String) -> Any
```

Reads a file from the local file system and parses its contents as JSON.

* **Parameters:**
    * `filepath` *(String)*: The path to the JSON file.
* **Returns:** *(Any)* The parsed kyro value representing the JSON file's contents.

```rust
var json = use("lib:json");

var config = json.load("config.json");
print(config["port"]);
```


### dump

```rust
dump(value: Any, filepath: String, indent: Number | Nil) -> Nil
```

Serializes a kyro value into a JSON string and writes it directly to a file at the specified path.

* **Parameters:**
    * `value` *(Any)*: The kyro value to serialize and write.
    * `filepath` *(String)*: The path to the output destination file.
    * `indent` *(Number | Nil)*: The indentation level for formatting (or `nil` for compact output).
* **Returns:** *(Nil)*

```rust
var json = use("lib:json");

var settings = {
    "theme": "dark",
    "fontSize": 14
};

// Writes the settings dictionary to settings.json with 2-space indentation
json.dump(settings, "settings.json", 2);
```