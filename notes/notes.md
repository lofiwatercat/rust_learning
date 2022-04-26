# Notes of the Rust Book

## 3 - Common Programming Concepts
---

### 3.1 - Variables and Mutability
Create a variable like so: `let x = 5`
Variables are immutable by default. Allow them to mutate with `let mut x = 5`

Constants are done like so: `const HOURS_IN_A_DAY: u32 = 24`

Shadowing - when we create an already existing variable again with the `let`
keyword, it will create the variable with the new value. Scopes matter

```
fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```
Output would be x = 12 for the inner scope, and then x = 6

And we can change data types as well
```
fn main() {
    let spaces = "   ";
    let spaces = spaces.len();
}
```

### 3.2 - Data Types
You can sign variables like so
`let x: f32 = 5.0`

Scalars - integers, floating-point numbers, booleans, and characters.

Integers - can be signed or unsigned, for example: i8 or u8, representing signed
and unsigned 8 bit integers respectively. Default is i32.

Floating-points - numbers with decimal points. Either f32 or f64(default)

Booleans - true or false, can be explicit like so: `let f: bool = false`

Characters - use single quotes

**Compound Types** - can group multiple values into one type. Comes in tuples and
arrays

Tuple - a way to group a number of values with a variety of types together.
Fixed length. `let tup: (i32, f64, u8) = (500, 309.46, 1)`

Can get individual values out of tuples like so. `let (x, y, z) = tup` It's
called destructuring. Can also access individual elements directly like so.
`let five_hundred = tup.0` There's also empty tuple `()`

Array - Every element is the same type. Fixed length. `let a = [1, 2, 3]` The
data is allocated on the stack. Can also specify their type.
`let b: [i32; 5] = [1, 2, 3, 4, 5]`

Can also initialize an array to contain the same value for each element on
initialization.

`let a = [3; 5];` is the same as `let a = [3, 3, 3, 3, 3,]`

Can access elements of an array through its index.

```
let a = [1, 2, 3, 4, 5];

let first = a[0]; // 1
```

Unit type - It's empty. When expressions return nothing, this is it.

### 3.3 - Functions
Snake case. Define functions by using `fn` and a set of parenthesis, followed by
curly brackets.

```
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Parameters - Add them in the parenthesis of the function. **Requires** the type
annotation.

```
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
```

Statements - instructions that peform an action but do not return a value. `let`
is an example.

Expressions - evaluate to a value. Can be part of statements. When we add a
semicolon to an expression, it turns into a statement.

Functions with return values - Functions can return values. Need to declare
their type after an arrow. Can return early from a function by using `return`
keyword, but most functions return last expression implicitly.

```
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

Note that the `5` in the function `five()` does not have a semicolon after it to
return that value.

### 3.4 - Comments
Use `//` to start comments. Same thing for multi-line comments

### 3.5 - Control Flow
If expressions - Self explanatory. The condition has to be a bool, unlike other
languages.

```
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

Can also use `if` in a `let` statement. The `if` and `else` arms must be
compatible types.

```
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of the number is: {}", number);
}
```

Repetition with Loops - `loop`, `while`, and `for`

loop - Repeats the code forever until you explicitly tell it to stop (ctrl-c).
Can break out of the loop with `break` or skip to the next loop with
`continue`. If working with loops inside of loops, `break` and `continue` will
apply to the innermost loop at that point. Can optionally specify a *loop label*
on a loop to target another loop besides the inner most with `break` and
`continue`.

```
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
```

To return a value from a loop, just add the value after the break.

```
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

while - You know this.

for - To loop through items in a collection, like an array.

```
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {}", element);
    }
}
```

## 4 - Ownership
### 4.1 - What is ownership?
Stack and Heap - Both stack and heap are parts of memory available to the code
during runtime. The stack stores values through *last in, first out*, like a
stack of plates where the last plate in (at the top) is the first one out.
Adding data is called *pushing to the stack* and removing is *popping off the
stack*.

Heap is less organized. The memory allocator finds a big enough empty spot,
marks it as in use, and returns a pointer, which is an address to the spot.
Called *allocating on the heap* or just *allocating*.

Stack is faster

Ownership rules
* Every value has a variable that is its *owner*.
* There can only be one owner at a time.
* When the owner goes out of scope, the value is dropped.

Variable Scope - A variable is valid from the point it is declared until the end
of the current scope.

```
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```

Strings - We did string literals earlier, like `let greet = "Hello"`. They are
hardcoded and stored on the stack beacuse their size is known at compile time.
There's also the `String` type, which is allocated on the heap and can store an
unknown amount of text at compile time.

` let s = String::from("Hello");`

It can be mutated.

```
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s); // Will print "hello, world!"
```

Can't do the same with a string literal.

Memory and Allocation - We allocate memory and use it. When the variable goes
out of scope, the memory is deallocated.

```
fn main() {
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
                                       // longer valid
}
```

Rust calls the `drop` function automatically to deallocate memory.

Move - When we create a string, it has 3 parts.
* A pointer to the memory that holds the contents
* The length
* The capacity

The three parts are held on the stack, and the pointer points to memory in the
heap that holds the contents. When we copy the variable, like:

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
}
```
We copy whats in the stack, namely the ptr, length, and capacity. The data on
the heap is not copied, and both pointers point to it. When a variable goes out
of scope, Rust drops it. But if it drops it twice, for example s1 and s2, it
could lead to corruption. So when we copy s1 to s2, s1 is no longer valid. In
this example, we can't use s1 anymore. This is called a **move**.

If you want to do a deep copy (copy both stack and heap data), can use `clone`.

```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

Don't need to deep copy for data types with the `Copy` trait, such as:
* All integer types
* Booleans
* Floating point
* char
* Tuples (if all their elements implement `Copy`, so (i32, i32) will work, but
  not (i32, String)

Ownership and Functions - Passing a variable to a function will move or copy,
like assignment does. This example is pretty good.

```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

Can't use `s` after takes_ownership since the value was *moved* into the
function. We can use `x` after the function because x is an integer and it has
the `copy` trait.

Return values and scope - Returning values can also transfer ownership.

```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

Can use tuples to return multiple values

```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
```

But its a pain in the ass. So thats what references and borrowing is for.

### 4.2 - References and Borrowing

Unlike previously, where we had to use a tuple to still be able to use the value
that was passed in, we can use references. A *reference* is like a pointer in
that it's an address we can follow to access data that is owned by a variable,
but it is guarenteed to point to a valid value of a particular type, unlike
pointers which can become dangling. Here's an example:

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

Take note of the `&` before the argument in the functions, showing that we are
passing a reference. We can use the value of `s1` without passing ownership, so we
can continue using `s1` after calling the `calculate_length` function. In
`calculate_length`, we are creating a reference `s`, which refers to the value
of `s1` and is valid until the end of the scope. When we exit the function, we
don't need to return ownership because we never had it.

This action of creating references is called borrowing. Also can't modify the
value being borrowed, unless we use a *mutable reference*, like this:

```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

Three things. We have to make the variable `s`, the reference `&mut s`, and the
argument type `&mut String` all mutable.

Also, we can only have one mutable reference to one particular piece of data at
a time.

```
fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
}
```

This would not compile because s would have two mutable references pointing to
it. This helps prevent data races, where several pointers would access the data
at the same time and one of them is being used to write the data while there is
no synchronization.

We can have multiple mutable references, just not simultaneously.

```
fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;
}
```

And we can't use both mutable and immutable references simultaneously. This code
won't compile.

```
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM

    println!("{}, {}, and {}", r1, r2, r3);
}
```

But having multiple immutable references is no problem.

A reference's scope begins from its initialization but ends at the last time it
is used, so this will compile:

```
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}
```

Since `r1` and `r2` aren't used after the `println!`, their scope ends there
and we can create `r3`, a mutable reference.

Rust also guarantees that all references will never be dangling. For example,
look at this code which won't compile.

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

We created `s` insde of `dangle` and return a reference to it, but when we reach
the end of the scope, `s` gets deallocated and the reference `&s` will be
pointing to an invalid string. The solution is to return the `String` directly.

```
fn main() {
    let string = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

### 4.3 - The Slice Type
Slices - let us reference a contiguous sequence of elements in a collection
rather than the whole thing.

Excercise, get the first word in a string.

```
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {}
```

We need to go through the string element by element to check if a value is a
space, so we convert our `String` to an array with the `as_bytes` method.

Then we need to create an iterator over the array of bytes using the `iter`
method. It returns an element, and we use `enumerate` to wrap the results in a
tuple `(i, &item)`, giving us the index `i` and the reference to the element
`&item`.

In the for loop, we look for the byte literal that represents the space and
return that index, otherwise we return the length of the String.

The problem is we are returning a `usize` that is only useful in context of the
`&String`. If we clear s, like in this example:

```
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!
}
```

Then word will be meaningless. We got a solution, and its name is String Slices.

String Slices - A reference to a part of the string. Looks like this:

```
fn main() {
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
}
```

We create slices by specifying the starting index and the ending index, and the
ending index is not included. In this case, `let world = %s[6..11];`, world
would be a slice that contains a pointer to the byte at index 6 of `s` with a
length value of 5.

For the `..` range syntax, we can start at index zero by dropping the first
value. Can also drop the second value for the entire string

```
fn main() {
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
let slice = &s[..]; // Same as let slice = &s[0..len];
}
```

Here's the previous `first_word` program rewritten with slices.

```
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Remember the earlier problem with the changing value and the reference becoming
useless? With the new method, it won't compile, letting us know there's a
problem

```
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```

What we are returning is connected to the state of `s`, so when we `clear` it
but still use the reference later when we `println!`, it's a problem. Remember,
if we have an immutable reference to something, `word` in this case, we can't
also have a mutable reference.

Quick reminder, references is for borrowing. Can still mutate data with them.

String literals are slices, like `let s  = "Hello, world!";` is a `&str`,
pointing to a specific point in binary. Also why string literals `&str` are
immutable.

String Slices as parameters - We can also write the previous function like so:
`fn first_word(s: &str) -> &str {` This way, we could pass in a slice of a
`String` or a reference to it.

Other Slices - We could also slice arrays too.

`let a = [1, 2, 3, 4, 5];`

We can cut a slice like this:

```
let slice = &a[1..3];

assert_eq!(slice, &[2, 3]);
```

And it will have a type of &[i32]

## 5 - Using Structs to Structure Related Data

### 5.1 - Defining and Instantiating Structs

Structs are similar to tuples. Can hold multiple related values of different
types, but we have to add a name for each piece of data. That lets us add the
data in any order.

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {}
```

And we can create an *instance* of the struct like this:

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
}
```

We use dot notation to get the value. We can change the value if the instance is
mutable. The entire instance must be mutable. Here's an example of a function
creating an instance and implicitly returning it.

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}
```

Notice how the functin parameters `email` and `username` are the same as the
fields. We don't have to repeat them by using the `field init shorthand` syntax.

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    let user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
}
```

If we want to create a new instance of a struct that has most of the values as
another instance but changes some, we can use *struct update syntax*.

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };
}
```

This is the long way to create user2, but using `struct update syntax`, we can
shorten it to this:

```
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    // --snip--

    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
}
```

The `..` specifies that the remaining fields should be the same as user1. Just
remember that the `..` has to come last, after you specify any values you wanted
to change.

Also in this example, take note that we use `=` when creating the instance, so
we are moving data. Since we did `..user1` when creating user2 and the
`username` is a `String`, user1 becomes invalid. If we gave user2 new `String`
values for both `email` and `username`, then we could still use user1 after
because the rest of the fields, `active` and `sign_in_count`, implement the
`Copy` trait.

Using Tuple Structs without Named Fields to Create Different Types

Tuple structs - for when structs are too much but you want a tuple with a name
and different types than other tuples

```
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

`black` and `origin` are different types because they are instances of different
tuple structs, even though their values are the same types. Can use them like
regular tuples, like use `.` followed by index to access the individual value,
and so on.

Unit-Like Structs without any fields

Can define structs without fields, called *unit-like structs* because they
behave similarly to ().

```
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

Later this could be a behavior to implement where every instance of
`AlwaysEqual` is always equal to every instance of any other type, maybe for
testing purposes.

### 5.2 - An Example Program Using Structs

Here is a program that will calculate the area of a rectangle.

```
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

It works, but if you look at the signature of `area` with the two parameters,
it's not clear that they are related.

```
fn area(width: u32, height: u32) -> u32 {
```

Lets use some tuple structs insead.

```
fn main() {
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(rect1)
    );
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
```

We have more structure and are passing one argument. But its less clear in that
we have to use index instead of width and height names for the arguments. What
we can do is to add more meaning with structs.

```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

We created an instance of `Rectangle`, and passed a reference to it to `area`.
Then it uses the `width` and `height` fields.

But what if we want to print in the function to see what's going on?

```
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {}", rect1);
}
```

This code won't compile. `println!` is a macro and doesn't know what to do with
structs because there's no clear way to best display them due to the different
variations of structs you can make, so it doesn't even try. Instead, we can put
the `:?` inside of the curly brackets to tell `println!` that we want to use an
output format called the `Debug`. It prints the struct in a way that is useful
for developers to see its value. But it still doesn't work. We just gotta add
`#[derive(Debug)]` just before the struct definition.

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

When we run the program, it'll work now. There's a couple more formats we can
use to make bigger structs easier to read, like `:#?`. We could also use the
`dbg! macro`, which takes ownership of an expression, prints the file and line
number, prints the value, and returns ownership.

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```

Heres the output

```
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.61s
     Running `target/debug/rectangles`
[src/main.rs:10] 30 * scale = 60
[src/main.rs:14] &rect1 = Rectangle {
    width: 60,
    height: 50,
}
```

### 5.3 - Method Syntax

Methods - similar to functions. Defined with `fn` and a name, take parameters
and return a value, but they are defined within the context of a struct and
their first argument is always `self`, representing the instance of the struct
they are called on.

Let's change the previous `area` function we worked with last time.

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

To define a function within context of `Rectangle`, we start with `impl`
(implementation) block for `Rectangle`, where everything in the block will be
associated with `Rectangle`. Then we move the `area` function into the `impl`
block. Then in `main`, we use *method syntax* to call `area` on our `Rectangle`
instance by doing `rect1.area().`

In the signature for `area`, we use `&self` instead of `rectangle: &Rectangle`.
`&self` is short for `self: &Self`. In an `impl` block, `Self` is an alias for
the type that the `impl` block is for. We still need to use `&` infront of
`self` to indicate that htis method borrows the `Self` instance. Methods can
take ownership of `self`, borrow `self` immutably, or borrow mutably, like any
other parameter. We could use `&mut self` to change the instance.

The main reason to use methods is for organization. We group the functionality
of a struct into an `impl` block.

We can also give a method the same name as one of the struct's fields. On
`rectangle`'s `width` field:

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```

When we follow `rect1.width()` with parenthesis, Rust knows we mean the method
`width`. Without parenthesis, Rust knows it's the field `width`.

More practice. Lets make a second method for the `Rectangle`. We want an
instance of `Rectangle` to take another instance of `Rectangle` and return true
if it can fit completely within `self` (the first `Rectangle`), otherwise it
returns false. Let's call it `can_hold`.

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

Associated Functions - All functions defined in an `impl` block are called
*associated functions* because they are associated with the type named after the
`impl`. Some don't have `self` as their first parameter because they don't need
an instance of the type to work with. One example is the `String::from` function
defined on the `String` type. Often used for constructors that return a new
instance of the struct.

Here we are making a `square` based off the `Rectangle`

```
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(3);
}
```

To call this associated function, we use the `::` syntax like so: `let sq =
Rectangle::square(3);`.

Multiple impl blocks - Each struct can have multiple `impl` blocks.

```
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```
