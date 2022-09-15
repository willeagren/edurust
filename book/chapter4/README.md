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
The mechanics of passing a value to a function are similar to those when
passing a value to a variable. Passing a variable to a function will move or
copy, just as assignment does. Below is an example with some annotations
showing where variables go into and out of scope:
```rust
fn main() {
    let s = String::from("hello");      // s comes into scope

    takes_ownership(s);                 // s's value moves into the function...
                                        // ... and so is no longer valid here

    let x = 5;                          // x comes into scope

    makes_copy(x);                      // x would move into the function,
                                        // but i32 is Copy, so it's okay to
                                        // still use x afterward
    
}  // Here, x goes out of scope, then s. But because s's value was moved,
   // nothing special happens.

fn takes_ownership(some_string: String) {  // some_string comes into scope
    println!("{}", some_string);
}  // Here, some_string goes out of scope and 'drop' is called. The backing
   // memory is freed.

fn makes_copy(some_integer: i32) {  // some_integer comes into scope
    println!("{}", some_integer);
}  // Here, some_integer goes out of scope. Nothing special happens.
```
If we tried to use `s` after the call to `takes_ownership`, Rust would throw a
compile-time error. These static checks protect us from mistakes. 


## Return Values and Scope
Returning values can also tranfer ownership. Below is an example of a function
that returns some value:
```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
}  // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
   // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {        // gives_ownership will move its
                                        // return value into the function
                                        // that calls it

    let s = String::from("hello");      // s comes into scope
    
    s                                   // s is returned and moves out
                                        // to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {  // a_string comes into 
                                                       // scope

    a_string                            // a_string is returned and moves out
                                        // to the calling function
}

```
The ownership of variable follows the same pattern every time: assigning a
value to another variable moves it. When a variable that includes data on the
heap goes out of scope, the value will be cleaned up by `drop` unless ownership
of the data has been moved to another variable.


While this works, taking ownership and then returning ownership with every
function is a bit tedious. What if we want to let a function use a value but
not take ownership? It's quite annoying that anything we pass in also needs to
be passed back if we want to use it again, in addition to any data resulting
from the body of the function that we might want to return as well.


Rust does let us return multiple values using a tuple, as shown below.
```rust
fn main() {
    let s1 = String::from("hello");
    
    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();               // len() returns the length
                                        // of the string

    (s, length)
}
```
But this is too much ceremony and a lot of work for a concept that should be
common. Luckily for us, Rust has a feature for using a value without
transferring ownership, called *references*.


## References and Borrowing
The issue with the tuple example abow is that we have to return the `String` to
the calling function so we can still use the `String` after the call to
`calculate_length`, because the `String` was moved into `calculate_length`.
Instead, we can provide a reference to the `String` value. A *reference* is
like a pointer in that it's an address we can follow to access the data stored
at that address; that data is owned by some other variable. Unlike a pointer, a
reference is guaranteed to point to a valid value of a particular type for the
life of that reference.


Here is how you would define and use a `calculate_length` function that has a
reference to an object as a parameter instead of taking ownership of the value:
```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```
The `&s1` syntax lets us create a reference that *refers* to the value of `s1`
but does not own it. Because it does not worn it, the value it points to will
not be dropped when the reference stops being used.


The scope in which the variable `s` is valid is the same as any function
parameter's scope, but the value pointed to by the reference is not dropped
when `s` stops being used because `s` doesn't have ownership. When functions
have references as parameters instead of actual values, we won't need to return
the values in order to give back ownership, because we never had ownership.

> We call the action of creating a reference *borrowing*. As in real life, if a
person owns something, you can borrow it from them. When you're done, you have
to give it back. You don't own it.

## Mutable References
Just as variables are immutable by default, so are references. But we can
modify the above code to allow us to modify a borrowed value with just a few
small tweaks that use, instead, a *mutable reference*:
```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
```
Mutable references have one big restriction: if you have a mutable reference to
a value, you can have no other references to that value. We *also* cannot have
a mutable reference while we have an immutable one to the same value.


## The Slice Type
*Slices* let you reference a contiguous sequence of elements in a collection
rather than the whole collection.
