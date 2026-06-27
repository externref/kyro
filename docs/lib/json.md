---
title: "json"
---

The `lib:json` library module provides utility functions to encode kyro data structures into JSON strings and decode JSON strings back into native kyro objects (such as dictionaries, lists, strings, numbers, booleans, and `nil`).

To use this module, load it using the `lib` prefix:

```javascript
var json = use("lib:json");
```

### loads

```kyro
loads(json_str: String) -> Any
```

Parses a JSON-encoded string and returns its representation as a native kyro value.

* **Parameters:**
    * `json_str` *(String)*: The JSON data to parse.
* **Returns:** *(Any)* A kyro dictionary, list, string, number, boolean, or `nil`.

```kyro
var json = use("lib:json");

var data = json.loads("{\"name\": \"kyro\", \"version\": 1.0, \"active\": true}");
print(data["name"]);     // "kyro"
print(data["version"]);  // 1.0
print(data["active"]);   // true
```

### dumps

```kyro
dumps(value: Any, indent: Number | Nil = Nil) -> String
```

Serializes a kyro value into a JSON-formatted string.

* **Parameters:**
    * `value` *(Any)*: The kyro value to serialize.
    * `indent` *(Number | Nil, optional)*: The number of spaces to use for indentation. Defaults to `nil`, which outputs a compact single-line string.
* **Returns:** *(String)* The serialized JSON string.

```kyro
var json = use("lib:json");

var config = {
    "port": 8080,
    "tags": ["web", "production"]
};

// Compact serialization (omitting the optional indent parameter)
var compact = json.dumps(config);
print(compact); // {"port": 8080, "tags": ["web", "production"]}

// Pretty-printed serialization with 4 spaces of indentation
var pretty = json.dumps(config, indent = 4);
print(pretty);
```

### load

```kyro
load(filepath: String) -> Any
```

Reads a file from the local file system and parses its contents as JSON.

* **Parameters:**
    * `filepath` *(String)*: The path to the JSON file.
* **Returns:** *(Any)* The parsed kyro value representing the JSON file's contents.

```kyro
var json = use("lib:json");

var config = json.load("config.json");
print(config["port"]);
```

### dump

```kyro
dump(value: Any, filepath: String, indent: Number | Nil = Nil) -> Nil
```

Serializes a kyro value into a JSON string and writes it directly to a file at the specified path.

* **Parameters:**
    * `value` *(Any)*: The kyro value to serialize and write.
    * `filepath` *(String)*: The path to the output destination file.
    * `indent` *(Number | Nil, optional)*: The indentation level for formatting. Defaults to `nil` for compact single-line output.
* **Returns:** *(Nil)*

```kyro
var json = use("lib:json");

var settings = {
    "theme": "dark",
    "fontSize": 14
};

// Writes the settings dictionary in compact format (omitting indent)
json.dump(settings, "settings.json");

// Writes the settings dictionary with 2-space indentation
json.dump(settings, "settings_pretty.json", indent = 2);
```