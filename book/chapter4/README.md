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


## Memory and Allocation
In the case of a string literal, we know the contents at compile time, so the
text is hardcoded directly into the final executable. This is why string
literals are fast and efficient. But there properties only come from the string
literal's immutability. Unfortunately, we can't put a blob of memory into the
binary for each piece of text whose size is unknown at compile time and whose
size might change while running the program. 


With the `String` type, in order to support a mutable, growable piece of text,
we need to allocate an amount of memory on the heap, unkonwn at compile time,
to hold the contents. This mean:
- The memory must be requested from the memory allocated at runtime.
- We need a way of returning this memory to the allocated when we're done with
  our `String`.


That first part is done by us: when we call `String::from`, its implementation
requests the memory it needs. This is pretty much universal in programming
languages. 


However, the second part is different. In languages with a *garbace collector
(GC)*, the GC keeps track of and cleans up memory that isn't being used
anymore, and we don't need to think about it. In most language without a GC,
it's our responsibility to identify when memory is no longer being used and
call code to explicitly free it, just as we did to request it. Doing this
correctly has historically been a difficult programming probem. If we forget,
we'll waste memory. If we do it too early, we'll have an invalid variable. If
we do it twice, that's a bug too. We need to pair exactly one `allocate` with
exactly on `free`. 


Rust takes a different path: the memory is autoamtically returned once the
variable that owns it goes out of scope. Here's a version of our scope example
using a `String` instead of a string literal:
```rust
{
    let s = String::from("hello");  // s is valid from this point forward

    // do stuff with s
}                                   // this scope is now over, and s is no
                                    // longer valid
```
There is a natural point at which we can return the memory out `String` needs
to the allocator: when `s` goes out of scope. When a variable goes out of
scope, Rust calls a special function for us. This function is called `drop`,
and it's where the author of `String` can put the code to return the memory.
Rust call `drop` automatically at the closing curly brackets.


### Ways Variables and Data Interact: Move
Multiple variables can interact with the same data in different ways in Rust.
Let's look at an example using an integer:
```rust
let x = 5;
let y = x;
```
Now we have two variables, `x` and `y`, and both equal `5`. This is indeed what
is happening, because integers are simple values with a known, fixed size, and
these two `5`values are pushed onto the stack.


Now let's look at the `String` version:
```rust
let s1 = String::from("hello");
let s2 = s1;
```
This looks very similar, so we might assume that the way it works would be the
same: that is, the second line would make a copy of the value in `s1` and bind
it to `s2`. But this isn't quite what happens.


A `String` is made up of three parts: a pointer to the memory that holds the
contents of the string, a length, and a capacity. This group of data is stored
on the stack. The pointer points to the memory on the heap.


When we assign `s1` to `s2`, the `String` data is copied, meaning we copy the
pointer, the length, and the capacity that are on the stack. We do not copy the
data on the heap that the pointer refers to. 


Earlier, we said taht when a variable goes out of scope, rust automatically
calls the `drop` function and cleans up the heap memory for that variable. But
as we think of it know both `s1` and `s2` memory pointers point to the same
location on the heap! This is a problem: when `s2` and `s1` go out of scope,
they will both try to free the same memory. This is known as a *double free*
error as is one of the memory safety bugs we mentioned previously. Freeing
memory twice can lead to memory corruption, which can potentially lead to
security vulnerabilities. 


To ensure memory safety, after the line `let s2 = s1`, Rust considers `s1` as
no longer valid. Therefore, Rust doesn't need to free anything when `s1` goes
out of scope. Check what happens when you try to use `s1` after `s2` is
created; it won't work...
```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}", s1);
```

If you've heard the terms *shallow copy* and *deep copy* while working with
other languages, the concept of copying the pointer, length, and capacity
without copying the data on the heap sounds like making a shallow copy. But
because Rust also invalidates the first variable, instead of calling it a
shallow copy, it's known as a *move*. This solves our problem, with only `s2`
valid, when it goes out of scope, it alone will free the memory, and we're
done!


In addition, there's a design choice that's implied by this: Rust will never
automatically create "deep" copies of your data. Therefore, any *automatic*
copying can be assumed to be inexpensive in terms of runtime performance.


### Stack-Only Data: Copy
There's another wringle we haven't talked about yet. This code using integers -
part of which was shown earlier - works and is valid:
```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```
But this code seems to contradict what we just learned: we don't have a call to
`clone`, but `x` is still valid and wasn't moved to `y`.


The reason is that types such as integers that have a known size at compile
time are stored entirely on the stack, so copies of the actual values are quick
to make. That means there's no reason we would want to prevent `x` from being
valid after we create the variable `y`. In other words, there's no difference
between deep and shallow copying here, so calling `clone` wouldn't do anything
different from the usual shallow copying and we can leave it out.


Rust has a special annotation called the `Copy` trait that we can place on
types that are stored on the stack, as integers are. If a type implements the
`Copy` trait, variables that use it do not move, but rather are trivially
copied, making them still valid after assignment to another variable. 


Rust won't let us annotate a type with `Copy` if the type, or any of its parts,
has implemented the `Drop` trait. If the type needs something special to happen
when the value goes out of scope and we add the `Copy` annotation to that type,
we'll get a compile-time error. 


So what types implement the `Copy` trait? You can check documentation for the
given type to be sure, but as a general rule, any group of simple scalar values
can implement `Copy`, and nothing that requires allocation or is some form of
resource can implement `Copy`. Here are some types that implement `Copy`:
- All the integer types, such as `u32`.
- The Boolean type, `bool`.
- All the floating point types, such as `f64`.
- The character type, `char`.
- Tuples, if they only contain types that also implement `Copy`. For example
  `(i32, i32)` implement `Copy`, but `(i32, String)` does not.


## Ownership and Functions
The mechanics...
