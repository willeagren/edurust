# What is Ownership?
The ownership rules state that:
- Each value in Rust has an *owner*.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Variable Scope
A scope is the range within a program for which an item is valid. Take the
following variable:
```rust
let s = "hello";
```
The variable `s` refers to a string literal, where the value of the string is
hardcoded into the text of our program. The variable is valid from the point at
which it's declared until the end of the current *scope*. 
```rust
{                       // s is not valid here, it's not yet declared
    let s = "hello";    // s is valid from this point forward

    // do stuff with s
}                       // this scope is now over, and s is no longer valid

```

## The `String` type
We've already seen string literals, where a string value is hardcoded into our
program. String literals are convenient, but they aren't suitable for every
situation in which we may want to use text. One reason is that they're
immutable. Another is that not every string value can be known when we write
our code: for example, if we want to take user input and store it? For these
situations, Rust has a second string type, `String`. This type manages data
allocated on the heap and as such is able to store an amount of text that is
unknown to us at compile time. You can create a `String` from a string literal
using the `from` function, like so:
```rust
let s = String::from("hello");
```
This kind of string *can* be mutated:
```rust
let mut s = String::from("hello");
s.push_str(", world!");  // push_str() appends a literal to a String
println!("{}", s);  // This will print 'hello, world!'
```
So, what's the difference here? Why can `String` be mutated but literals
cannot? The difference is how these two types deal with memory.

