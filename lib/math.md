---
title: math
---

This module contains mathematical constants and trigonometric functions written in kyro.

**Include using**

```kyro
var math = use("lib:math");
```

### PI

```kyro
PI: Number
```

The mathematical constant $\pi$ (pi), representing the ratio of the circumference of a circle to its diameter.

* **Value:** `3.141592653589793`

### TWO_PI

```kyro
TWO_PI: Number
```

The mathematical constant $2\pi$ (tau), representing a full turn in radians.

* **Value:** `6.283185307179586`

### sum

```kyro
sum(args: List = []) -> Number
```

Calculates the sum of all numerical elements in a list.

* **Parameters:**
    * `args` *(List, optional)*: A list of numbers to be summed. Defaults to an empty list `[]`.
* **Returns:** *(Number)* The total sum of the elements.

```kyro
var math = use("lib:math");

var values = [1, 2, 3, 4, 5];
print(math.sum(values)); // 15

// Calling sum with no arguments defaults to an empty list
print(math.sum());       // 0
```

### sin

```kyro
sin(x: Number = 0) -> Number
```

Calculates the sine of an angle $x$ (expressed in radians) using a Taylor series approximation.

* **Parameters:**
    * `x` *(Number, optional)*: The angle in radians. Defaults to `0`.
* **Returns:** *(Number)* The sine of $x$.

```kyro
var math = use("lib:math");

print(math.sin(math.PI / 2)); // 1
print(math.sin());            // 0
```

### cos

```kyro
cos(x: Number = 0) -> Number
```

Calculates the cosine of an angle $x$ (expressed in radians) using a Taylor series approximation.

* **Parameters:**
    * `x` *(Number, optional)*: The angle in radians. Defaults to `0`.
* **Returns:** *(Number)* The cosine of $x$.

```kyro
var math = use("lib:math");

print(math.cos(math.PI)); // -1
print(math.cos());        // 1
```

### tan

```kyro
tan(x: Number = 0) -> Number
```

Calculates the tangent of an angle $x$ (expressed in radians).

* **Parameters:**
    * `x` *(Number, optional)*: The angle in radians. Defaults to `0`.
* **Returns:** *(Number)* The tangent of $x$.

```kyro
var math = use("lib:math");

print(math.tan(0)); // 0
print(math.tan());  // 0
```

### min

```kyro
min(a: Number, b: Number) -> Number
```

Returns the smaller of two numbers.

* **Parameters:**
    * `a` *(Number)*: The first number.
    * `b` *(Number)*: The second number.
* **Returns:** *(Number)* The smaller value.

```kyro
var math = use("lib:math");

print(math.min(10, 20)); // 10
```

### max

```kyro
max(a: Number, b: Number) -> Number
```

Returns the larger of two numbers.

* **Parameters:**
    * `a` *(Number)*: The first number.
    * `b` *(Number)*: The second number.
* **Returns:** *(Number)* The larger value.

```kyro
var math = use("lib:math");

print(math.max(10, 20)); // 20
```

### deg_to_rad

```kyro
deg_to_rad(deg: Number) -> Number
```

Converts an angle from degrees to radians.

* **Parameters:**
    * `deg` *(Number)*: The angle in degrees.
* **Returns:** *(Number)* The angle in radians.

```kyro
var math = use("lib:math");

print(math.deg_to_rad(180)); // 3.141592653589793 (math.PI)
```

### rad_to_deg

```kyro
rad_to_deg(rad: Number) -> Number
```

Converts an angle from radians to degrees.

* **Parameters:**
    * `rad` *(Number)*: The angle in radians.
* **Returns:** *(Number)* The angle in degrees.

```kyro
var math = use("lib:math");

print(math.rad_to_deg(math.PI)); // 180
```

### sqrt

```kyro
sqrt(x: Number) -> Number
```

Calculates the square root of a non-negative number.

* **Parameters:**
    * `x` *(Number)*: The input number. Must be greater than or equal to `0`.
* **Returns:** *(Number)* The calculated square root.

```kyro
var math = use("lib:math");

print(math.sqrt(16)); // 4
```

### pow

```kyro
pow(base: Number, exp: Number) -> Number
```

Calculates a base raised to an integer exponent power.

* **Parameters:**
    * `base` *(Number)*: The base value.
    * `exp` *(Number)*: The integer exponent.
* **Returns:** *(Number)* The result of $base^{exp}$.

```kyro
var math = use("lib:math");

print(math.pow(2, 3));  // 8
print(math.pow(5, -1)); // 0.2
```