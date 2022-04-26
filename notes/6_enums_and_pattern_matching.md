# 6 - Enums and Pattern Matching
---

## 6.1 - Defining an Enum
Enums let us define custom data types so we can have *varients*.

Using ip addresses as an example, we have IPv4 and IPv6. Any ip address can be
either version, but not both at the same time, so using an enum is suitable.
Since v4 and v6 versions are both ip addresses, they should be treated as the
same type when the code is handling situations that apply to any kind of IP
address. Lets define an enum `IpAddrKind` and list its possible IP address that
it can be, `V4` and `V6`.

```
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);
}

fn route(ip_kind: IpAddrKind) {}
```

Here, we created the enum `IpAddrKind` and created two instances of it, `four`
and `six`. Then we can define a function `route` that takes any `IpAddrKind`.

If we wanted to store data to the IP address, we could do structs.

```
fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
```

We defined a struct called `IpAddr` with two fields: `kind` for IpAddrKind and
`address` for `String`. We have two instances as well. However, we could use an
enum to be more concise.

```
fn main() {
    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"));
}
```

In the `enum` definition, `IpAddr` enum says that both `V4` and `V6` varients
have an associated `String` value. This way, `IpAddr::V4()` acts like a
function call that takes a `String` argument and returns an instance of `IpAddr`
type. This constructor is automatically defined as a result of enums.

Another advantage for enums, each varient can have different types and amounts
of asscoiated data.

```
fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
```

The type of data enums can hold is quite flexible. Here, the enum data types are
holding structs

```

#![allow(unused)]
fn main() {
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
}
```

Also like how we could define methods on structs, we can define methods on
enums.

```
fn main() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}
```

### The Option Enum and Its Advantage
The `option` enum - encodes a value that could be something or could be nothing.
Null has caused a lot of problems in other languages because of its
implementation, not it's idea. We have the enum `Option<T>` for that

```
enum Option<t> {
    None,
    Some(T),
}
```

The T means that the `Some` varient can hold one piece of data of any type. Here
are some examples.

```
fn main() {
    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;
}
```

The type of `some_number` is `Option<i32>`. `some_string` is `Option<&32>`. Rust
can infer these types because we specified a value inside of the `Some` variant.
For `absent_number`, we need to annotate the overall `Option` type because we
didn't specify a value. Here, we tell Rust that it's type is `Option<i32>`.

When we have a `Some` value, we know a value is present. When we have a `None`
value, it means null, a non-valid value. So why `Option<T>`? Because `Option<T>`
and T (where T can be anytype) are different values, and the compiler won't let
us use `Option<T>` value as if it were a valid value. This won't compile because
it's trying to ad an i8 to an `Option<i8>`

```
fn main() {
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
}
```

Only when we have a way to deal with `Option<i8>` will the compiler let us use
the value. Basically, we have to convert an `Option<T>` to a T before we can
perform T operations with it. Helps us catch when we assume things aren't null
when they actually are.

## 6.2 - The `match` Control Flow Construct
`match` - lets us compare a value against a series of patterns and then executes
code based on which pattern matches. Can think of it like a coin-sorting
machine, where each coin falls into the hole it fits into. Here's an example.

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

Lets break it down. First we use the `match` keyword followed by an expression,
which is the value `coin` in this case. Seems similar to `if`, but instead of a
bool, the expression can be any type. Here, `coin` is the `Coin` enum type.

Next are the `match` arms. Arms have two parts, pattern and code. The first arm
here has a pattern that is the value `Coin::Penny` and then the `=>` operator
seperates the pattern and code to run. The code is just `1`. Each arm is
seperated with a comma. The code is an expression, and the resulting value gets
returned for the entire `match` expression. We can use curly brackets for
multi-line code.

```
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {}
```

We can also bind arms to the parts of the values that match the pattern. This is
how we extract value from enum variants.

In this example, quarters will have a `UsState` enum value.

```
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {}
```

```
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alaska));
}
```

### Matching with `Option<T>`
Using `match`, we can get values out of `Option<T>`

```
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

`Some(5)` will match with `Some(i)` and the `5` will bind to `i`, meaning `i`
takes the value of `5`. Then we return `Some(6)`.

Another thing, matches are *exahustive*. We must exhaust every last possibility
in order for the code to be valid.

```
fn main() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

This won't compile because we are missing the `None` case for `Option<T>`. But
what do we do if there are a lot of options? We can use the Catch-All Pattern
and the \_Placeholder

### Catch-all Patterns and the \_Placeholder
If we want to take special actions for a few values, but all other actions
default to on, it would be tedious to write every case. For example, a dice roll
where on `3` and `7`, we do a certain action. Here's an example.

```
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}
```

The code that runs for the `other` arm uses the variable by passing it into the
`move_player` function. This catch-all has to come last and matches all values
not listed (ie `u8`)

Rust also has a special pattern when we don't want to use the value in the
catch-all pattern: `_`. It matches any value and doesn't bind to that value.

```
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
}
```

Here, if we don't roll a `3` or a `7`, we reroll. We can also change the code in
the `_` arm to the unit value `()` if we don't want anything to happen.

```
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
}
```

## 6.3 - Concise Control Flow with if let
The `if let` syntax lets us combine `if` and `let` into a less verbose way to
handle values that match one pattenr while ignoring the rest. This program
matches on an `Option<u8>` value in the `config_max` variable but only wants to
execute code if the value is the `Some` variant.

```
fn main() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
}
```

We have to deal with the `None` by adding the `_ => ()` after processing just
one varient. We can use `if let` instead, however we lose the exhaustive
checking that `match` enforces.

```
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
```


