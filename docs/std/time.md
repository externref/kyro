---
title: time
---

This module contains functions for working with time and date values.

**Include using**

```kyro
var time = use("std:time");
```

### clock

```kyro
clock() -> Number
```

Returns the current Unix timestamp as a number representing the number of seconds elapsed since January 1, 1970 (UTC).

* **Parameters:** None
* **Returns:** *(Number)* The current Unix timestamp in seconds.

```kyro
var time = use("std:time");

var ts = time.clock();
print(ts);
```

### now

```kyro
now() -> Dict
```

Returns the current date and time as a dictionary containing the fields `year`, `month`, `day`, `hour`, `minute`, and `second`.

* **Parameters:** None
* **Returns:** *(Dict)* A dictionary containing calendar and time components:
    * `year`: The current year as a number.
    * `month`: The current month (1-12) as a number.
    * `day`: The current day of the month (1-31) as a number.
    * `hour`: The current hour (0-23) as a number.
    * `minute`: The current minute (0-59) as a number.
    * `second`: The current second (0-59) as a number.

```kyro
var time = use("std:time");

var now = time.now();

print(now["year"]);
print(now["month"]);
print(now["day"]);
```

### format

```kyro
format(timestamp: Number = clock(), format: String = "%Y-%m-%d %H:%M:%S") -> String
```

Formats a Unix timestamp using a format string.

Supported format specifiers:

* `%Y` - four-digit year
* `%m` - two-digit month
* `%d` - two-digit day
* `%H` - two-digit hour
* `%M` - two-digit minute
* `%S` - two-digit second

* **Parameters:**
    * `timestamp` *(Number, optional)*: The Unix timestamp (in seconds) to format. Defaults to the current system timestamp returned by `clock()`.
    * `format` *(String, optional)*: The template string containing the format specifiers. Defaults to `"%Y-%m-%d %H:%M:%S"`.
* **Returns:** *(String)* The formatted date and time string.

```kyro
var time = use("std:time");

// Formats the current time using default formatting (omitting both parameters)
var current_formatted = time.format();
print(current_formatted); // e.g. "2023-10-27 15:45:00"

// Formats a custom timestamp with a custom format
var custom_date = time.format(1698421500, format = "%d/%m/%Y");
print(custom_date); // "27/10/2023"
```

### sleep

```kyro
sleep(ms: Number) -> Nil
```

Pauses execution of the current thread for the specified duration of milliseconds.

* **Parameters:**
    * `ms` *(Number)*: The number of milliseconds to sleep. Must be a positive value.
* **Returns:** *(Nil)*

```kyro
var time = use("std:time");

print("waiting...");
// Pause execution for 1.5 seconds (1500 milliseconds)
time.sleep(1500);
print("done!");
```