# FlatString

A FlatString is a string that is stored in a fixed-size / flat array of characters. This is useful for scenarios where you want to avoid dynamic memory allocation, or where you want to store a string in a fixed-size buffer. A FlatString is a simple struct that contains:
- a fixed-size array of type **u8**
- the length of the string
- the number of characters. 

The maximum store capacity of the FlatString is limited to **255** bytes (so a string has to be less than **255** characters long). As such, the length of the string (in bytes) and the number of characters are stored as a `u8` as well.

For example, a `FlatString<14>` will have a size of 16 bytes (14 bytes for the string, 1 byte for the length, and 1 byte for the number of characters).

## Usage

To use a FlatString, you need to add the following to your `Cargo.toml`:

```toml
[dependencies]
flat_string = "1.0.0"
```

Then, you can use the `FlatString` as follows:

```rust
use flat_string::FlatString;

fn main() {
    // the flat string will be created on the stack
    let s = FlatString::<14>::from_str("Hello World !");
    println!("{}", s);
}
```

## Methods

The following methods are available for the `FlatString`:

| Method         | Description                                                                                                                                                                           |
| -------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `from_str`     | Create a `FlatString` from a string slice                                                                                                                                             |
| `new`          | Create an empty `FlatString`                                                                                                                                                          |
| `len`          | Get the length of the string                                                                                                                                                          |
| `chars_count`  | Get the number of characters in the string                                                                                                                                            |
| `capacity`     | Get the capacity of the string                                                                                                                                                        |
| `as_str`       | Get the string as a `&str`                                                                                                                                                            |
| `clear`        | Clear the string                                                                                                                                                                      |
| `push_str`     | Ads a string slice to the existig string. If the resulted string size is bigger than the string capacity, the string will be truncated to fit in the allocated capacty.               |
| `push`         | Adds a character to the existing string. If the resulted string size is bigger than the string capacity, the string will be truncated to fit in the allocated capacty.                |
| `try_push_str` | Ads a string slice to the existig string only if the resulted string fits in the preallocated capacity. In this case this method will return `Some(&str)` otherwise it returns `None` |
| `try_push`     | Adds a character to the existing string only if the resulted string fits in the preallocated capacity. In this case this method will return `Some(&str)` otherwise it returns `None`  |
| `set`          | Set the string to a new value. If the new string size is bigger than the string capacity, the string will be truncated to fit in the allocated capacty.                               |

`FlatString` implements the following traits:
- `std::fmt::Display` and `std::fmt::Debug` (this allows you to print the string using `println!` and `dbg!`)
- `std::ops::Deref` (this allows you to use the `*` operator to get the string as a `&str`). This will also allow you to use the `FlatString` as a `&str` in function arguments.
- `Copy` and `Clone` (this allows you to copy the `FlatString` using the `Copy` trait)
- `PartialEq` and `Eq` (this allows you to compare two `FlatString` using the `==` operator)
- `PartialOrd` and `Ord` (this allows you to compare two `FlatString` using the `<`, `>`, `<=`, and `>=` operators)
- `Default` (this allows you to create an empty `FlatString`)

## Example

1. Create a `FlatString` from a string slice:
    ```rust
    let s = FlatString::<14>::from_str("Hello World !");
    // s will be "Hello World !"
    ``` 

2. Create a `FlatString` from a string slice that is larger than its capacity:
    ```rust
    let s = FlatString::<10>::from_str("Hello World !");
    // s will be "Hello Worl" (truncated to fit 10 bytes)
    ```   

3. Add a `&str` to an existing `FlatString` :
    ```rust
    let mut s = FlatString::<30>::from_str("Hello");
    // s is "Hello"
    s.push_str(" World !");
    // s is "Hello World !"
    ```   

4. Add a `&str` to an existing `FlatString` and exceed the capacity:
    ```rust
    let mut s = FlatString::<8>::from_str("Hello");
    // s is "Hello"
    s.push_str(" World !");
    // s is "Hello Wo" (truncated to fit 8 bytes)
    ```   

5. Add a `&str` to an existing `FlatString` using `try_push_str`:
    ```rust
    let mut s = FlatString::<8>::new();
    if let Some(res) = s.try_push_str("Hello") {
        // res is "Hello"
        println!("{}", res);
    } else {
        // the string was not added
    }
    let result = s.try_push_str(" World !") {
    // result is None as adding " World !" would exceed the capacity
    // s remains "Hello"
    ```
6. Validating the size of a `FlatString`
   ```rust
    let mut s = FlatString::<8>::new();
    assert_eq!(std::mem::size_of_val(&s), 10);
    // the size of the FlatString is 10 bytes:
    // - 8 bytes for the string
    // - 1 byte for the length
    // - 1 byte for the number of characters
    ```
