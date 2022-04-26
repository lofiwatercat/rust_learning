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

Lets break it down.
