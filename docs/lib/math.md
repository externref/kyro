---
title: math
---

This module contains mathematical constants and trigonometric functions written in kyro.

**Include using**

```rust
var math = use("lib:math");
```

### PI

```rust
PI: Number
```

The mathematical constant $\pi$ (pi), representing the ratio of the circumference of a circle to its diameter.

* **Value:** `3.141592653589793`


### TWO_PI

```rust
TWO_PI: Number
```

The mathematical constant $2\pi$ (tau), representing a full turn in radians.

* **Value:** `6.283185307179586`


### sum

```rust
sum(args: List) -> Number
```

Calculates the sum of all numerical elements in a list.

* **Parameters:**
    * `args` *(List)*: A list of numbers to be summed.
* **Returns:** *(Number)* The total sum of the elements.

```rust
var math = use("lib:math");

var values = [1, 2, 3, 4, 5];
print(math.sum(values)); // 15
```


### sin

```rust
sin(x: Number) -> Number
```

Calculates the sine of an angle $x$ (expressed in radians) using a Taylor series approximation.

* **Parameters:**
    * `x` *(Number)*: The angle in radians.
* **Returns:** *(Number)* The sine of $x$.

```rust
var math = use("lib:math");

print(math.sin(math.PI / 2)); // 1
```


### cos

```rust
cos(x: Number) -> Number
```

Calculates the cosine of an angle $x$ (expressed in radians) using a Taylor series approximation.

* **Parameters:**
    * `x` *(Number)*: The angle in radians.
* **Returns:** *(Number)* The cosine of $x$.

```rust
var math = use("lib:math");

print(math.cos(math.PI)); // -1
```


### tan

```rust
tan(x: Number) -> Number
```

Calculates the tangent of an angle $x$ (expressed in radians).

* **Parameters:**
    * `x` *(Number)*: The angle in radians.
* **Returns:** *(Number)* The tangent of $x$.

```rust
var math = use("lib:math");

print(math.tan(0)); // 0
```