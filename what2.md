```python
# Returns the sum of two numbers.
#
# Example:
# ```
# print(add(3, 4))
# # prints '7'
# ```
def add(a, b):
    total = a + b
    # total is the sum
    return total

print(add(42, 69))
```

* comment
    * `Returns the sum of two numbers.\n\nExample:\n"`
    * code
        * `print(add(3, 4))`
        * comment
            * `prints '7'`
* code (function def)
    * `def add(a, b):` (header)
    * `total = a + b` (value def)
    * comment
        * `total is the sum`
    * `return total` (statement)
* code
    * `print(add(42, 69))`

---

Function `add`

Returns the sum of two numbers.

Example:
```
print(add(3, 4))
# prints '7'
```

---

Returns the sum of two numbers.

Example:
```
print(add(3, 4))
# prints '7'
```

---

For whatever reasons we need a function to add two numbers. Let's define that, and later we'll see why it was needed in the first place:

```python
def add(a, b):
    return a + b
```

Now that we have that we can do whatever...

```python
print(add(40, 2))
```

* comment
    * `For whatever reasons we need a function to add two numbers. Let's define that, and later we'll see why it was needed in the first place:`
* code (function def)
    * `def add(a, b):` (header)
    * `return a + b` (statement)
* comment
    * `Now that we have that we can do whatever...`
* code
    * `print(add(40, 2))`

---

# Actual syntax

```
I am in literate mode!

> print("Hello from literate mode!");
```

```
# I am in code mode!

print("Hello from code mode!");
```

```
# foos the variables
#
# for example:
# > let x = foo(1, 2);
# > print(x)
# > # prints "3"
fn foo(a, b) {
    return a + b;
}
```