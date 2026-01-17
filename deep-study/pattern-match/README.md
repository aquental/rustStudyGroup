# Rust's Pattern-Matching Capabilities

We're diving into one of Rust's most powerful and elegant features: **pattern matching**. Pattern matching allows you to destructure and inspect data structures in a concise, safe, and expressive way. It's central to Rust's philosophy of handling errors, variants, and complex data without runtime surprises.

We'll start with the basics and build up to advanced uses. I'll provide code examples throughout—feel free to copy and run them in your Rust playground or IDE. By the end, you'll see how pattern matching makes your code more readable and robust.

## 1. Introduction to Pattern Matching

Pattern matching in Rust is primarily done via the `match` expression, which is like a supercharged switch statement. It lets you compare a value against a series of patterns and execute code based on which pattern matches.

Key principles:

- **Exhaustive matching**: All possible cases must be covered, or the compiler complains. This prevents bugs.
- **Patterns can bind variables**: You can extract parts of the data into new variables.
- **Supports many types**: Enums, structs, tuples, arrays, slices, and more.

Patterns are used not just in `match`, but also in `let`, `if let`, loops, and function arguments.

## 2. The `match` Expression

The `match` keyword takes an expression and branches based on patterns.

### Basic Example: Matching Primitives

Let's match on an integer:

```rust
fn main() {
    let number = 42;

    match number {
        0 => println!("Zero!"),
        1 => println!("One!"),
        42 => println!("The answer to life, the universe, and everything!"),
        _ => println!("Some other number."),
    }
}
```

Here:

- Each arm is `pattern => expression,`.
- `_` is a wildcard that matches anything (and ignores it).
- The match must be exhaustive; without `_`, it won't compile if not all integers are covered (impossible, hence `_` is key).

### Matching Enums

Enums are where pattern matching shines. Consider a simple enum:

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),  // State quarter with origin
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {}!", state);
            25
        }
    }
}

fn main() {
    let coin = Coin::Quarter(String::from("Alaska"));
    println!("Value: {} cents", value_in_cents(coin));
}
```

- Patterns like `Coin::Quarter(state)` destructure the enum variant and bind the inner value to `state`.
- This is exhaustive because all variants are covered.

## 3. Destructuring Complex Types

Patterns can peel apart structs, tuples, and more.

### Struct Destructuring

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x: 0, y } => println!("On the y-axis at {}", y),  // Bind y, match x exactly
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point { x, y } => println!("At ({}, {})", x, y),
    }
}
```

- You can match specific fields and bind others.
- Shorthand: `Point { x, y }` binds both.

### Tuple and Array Destructuring

```rust
fn main() {
    let tuple = (1, "hello", true);

    match tuple {
        (1, msg, _) => println!("First is 1, message: {}", msg),  // Ignore third
        (n, _, true) => println!("Third is true, first: {}", n),
        _ => println!("No match"),
    }

    let array = [1, 2, 3];
    match array {
        [first, ..] => println!("First element: {}", first),  // .. ignores the rest
        _ => (),  // Empty match
    }
}
```

- `..` ignores remaining elements in slices/arrays.
- For fixed-size arrays: `[a, b, c]` binds each.

## 4. Advanced Patterns

### Guards

Add conditions with `if`:

```rust
fn main() {
    let num = Some(42);

    match num {
        Some(x) if x % 2 == 0 => println!("Even number: {}", x),
        Some(x) => println!("Odd number: {}", x),
        None => println!("No number"),
    }
}
```

- The guard `if x % 2 == 0` refines the pattern.

### Binding with `@`

Bind a value while matching a subpattern:

```rust
fn main() {
    let msg = "Hello, world!";

    match msg.len() {
        n @ 10..=15 => println!("Medium length: {}", n),  // Bind to n and match range
        _ => println!("Other length"),
    }
}
```

- `n @ pattern` binds the matched value to `n`.

### Or Patterns and Ranges

```rust
fn main() {
    let x = 5;

    match x {
        1 | 2 => println!("One or two"),
        3..=5 => println!("Three to five"),
        _ => println!("Other"),
    }
}
```

- `|` for alternatives.
- `..=` for inclusive ranges.

### Ignoring Parts

Use `_` or nested ignores:

```rust
struct Nested {
    a: i32,
    b: (i32, i32),
}

fn main() {
    let nested = Nested { a: 1, b: (2, 3) };

    match nested {
        Nested { a, b: (_, y) } => println!("a: {}, second in b: {}", a, y),
    }
}
```

## 5. Pattern Matching Beyond `match`

### In `let` Bindings

Destructure directly:

```rust
let (x, y) = (1, 2);  // x=1, y=2

let Point { x, y: renamed_y } = Point { x: 3, y: 4 };  // Bind x=3, renamed_y=4
```

- Fails to compile if patterns don't match.

### `if let` and `while let`

For optional matching:

```rust
fn main() {
    let option = Some(42);

    if let Some(x) = option {
        println!("Got: {}", x);
    } else {
        println!("Nothing");
    }

    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
}
```

- `if let` is like a single-arm match for `Option` or `Result`.
- `while let` loops while the pattern matches.

### In Loops and Functions

```rust
fn main() {
    let pairs = vec![(1, 'a'), (2, 'b')];

    for (num, ch) in pairs {
        println!("{}: {}", num, ch);
    }
}

fn print_point(Point { x, y }: Point) {
    println!("({}, {})", x, y);
}
```

- `for` destructures iterators.
- Function params can use patterns.

## 6. Handling `Option` and `Result`

Pattern matching is idiomatic for these:

```rust
fn divide(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 { Err("Division by zero") } else { Ok(a / b) }
}

fn main() {
    match divide(10.0, 0.0) {
        Ok(result) => println!("Result: {}", result),
        Err(msg) => println!("Error: {}", msg),
    }
}
```

- Avoids unwrap panics; forces error handling.

## 7. Best Practices and Pitfalls

- **Exhaustiveness**: Always cover all cases (it's a feature, not a bug)!
- **Refutability**: Some patterns (e.g., with bindings) are "irrefutable" for `let`, but "refutable" for `match` arms.
- **Performance**: Matching is compile-time optimized; no overhead.
- **Common error**: Forgetting to handle `None` or `Err`.
- **Pro tip**: Use patterns to avoid nested `if`s and make code flatter.

Practice by rewriting some conditional code with patterns. Questions? Let's discuss in the comments or next class!
