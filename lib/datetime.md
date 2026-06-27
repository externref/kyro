---
title: datetime
---

This module contains high-level date and time manipulation utilities, extending the core `std:time` module with object-oriented representations.

**Include using**

```kyro
var datetime = use("lib:datetime");
```

### now

```kyro
now() -> DateTime
```

Returns a `DateTime` instance representing the current local date and time.

* **Parameters:** None
* **Returns:** *(DateTime)* A `DateTime` object initialized with the current system time.

```kyro
var datetime = use("lib:datetime");

var current = datetime.now();
print(current); // e.g. "2023-10-27 14:30:00" (automatically invokes __str__)
```

### from_timestamp

```kyro
from_timestamp(timestamp: Number) -> DateTime
```

Constructs a `DateTime` object from a numeric Unix timestamp.

* **Parameters:**
    * `timestamp` *(Number)*: The numeric Unix epoch timestamp (seconds since 1970-01-01).
* **Returns:** *(DateTime)* A new `DateTime` instance.

```kyro
var datetime = use("lib:datetime");

var dt = datetime.from_timestamp(1698417000);
print(dt.year); // 2023
```

### DateTime (Class)

The primary class representing a calendar date and time. Its initialization is driven internally by the class’s `__init__` constructor method.

#### Constructor

```kyro
DateTime(year: Number, month: Number = 1, day: Number = 1, hour: Number = 0, minute: Number = 0, second: Number = 0)
```

Constructs a new `DateTime` instance. Optional parameters default to their respective minimum bounds.

* **Parameters:**
    * `year` *(Number)*: The calendar year.
    * `month` *(Number, optional)*: The month (1-12). Defaults to `1`.
    * `day` *(Number, optional)*: The day of the month (1-31). Defaults to `1`.
    * `hour` *(Number, optional)*: The hour (0-23). Defaults to `0`.
    * `minute` *(Number, optional)*: The minute (0-59). Defaults to `0`.
    * `second` *(Number, optional)*: The second (0-59). Defaults to `0`.

```kyro
var datetime = use("lib:datetime");

// Using default parameters for month, day, hour, etc.
var start_of_year = datetime.DateTime(2024);
print(start_of_year); // "2024-01-01 00:00:00"
```

#### to_timestamp

```kyro
to_timestamp() -> Number
```

Converts the `DateTime` instance into its numeric Unix timestamp representation.

* **Parameters:** None
* **Returns:** *(Number)* The equivalent Unix epoch timestamp.

```kyro
var datetime = use("lib:datetime");

var dt = datetime.DateTime(2023, month = 10, day = 27);
var ts = dt.to_timestamp();
```

#### add_days

```kyro
add_days(days: Number) -> DateTime
```

Calculates a new `DateTime` adjusted by the specified number of days (can be positive or negative).

* **Parameters:**
    * `days` *(Number)*: The number of days to add.
* **Returns:** *(DateTime)* A new `DateTime` instance representing the recalculated date.

```kyro
var datetime = use("lib:datetime");

var today = datetime.now();
var next_week = today.add_days(7);
print(next_week);
```

#### format

```kyro
format(format_str: String = "%Y-%m-%d %H:%M:%S") -> String
```

Formats the `DateTime` values as a string based on the provided format specifiers.

Supported format specifiers:
* `%Y` - Four-digit year
* `%m` - Two-digit month
* `%d` - Two-digit day
* `%H` - Two-digit hour
* `%M` - Two-digit minute
* `%S` - Two-digit second

* **Parameters:**
    * `format_str` *(String, optional)*: The formatting template. Defaults to `"%Y-%m-%d %H:%M:%S"`.
* **Returns:** *(String)* The formatted date-time string.

```kyro
var datetime = use("lib:datetime");

var today = datetime.now();
var date_only = today.format("%d/%m/%Y");
print(date_only); // e.g. "27/10/2023"
```

#### __str__

```kyro
__str__() -> String
```

Returns the standard string representation of the `DateTime` instance formatted as `"%Y-%m-%d %H:%M:%S"`. This method is called automatically by global output functions like `print` and `println`.

* **Parameters:** None
* **Returns:** *(String)* The formatted string.

```kyro
var datetime = use("lib:datetime");

var dt = datetime.DateTime(2024, month = 5, day = 15, hour = 9);
print(dt); // "2024-05-15 09:00:00"
```