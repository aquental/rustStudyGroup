# Rust's Type System Capabilities

We're exploring Rust's **type system**, a cornerstone of its safety, performance, and expressiveness. Rust's type system is statically typed, strongly typed, and inference-heavy, meaning the compiler checks types at compile time, enforces strict rules, but often infers types so you don't have to write them everywhere.

Rust prevents common bugs like null pointers, data races, and invalid memory access through its type system features like ownership, borrowing, enums, traits, and generics. We'll break it down step by step, with code examples to illustrate. Run these in your Rust environment to experiment!

## 1. Fundamentals of Rust's Type System

- **Static and Strong Typing**: Types are known at compile time (static), and conversions must be explicit (strong). No implicit coercions like in JavaScript.
- **Type Inference**: The compiler guesses types based on usage, reducing boilerplate.
- **Primitives**: Built-in types like `i32`, `f64`, `bool`, `char`, `&str`, etc.
- **Compound Types**: Tuples, arrays, structs, enums.

### Example: Basic Types and Inference

```rust
fn main() {
    let x = 42;  // Inferred as i32
    let y: f64 = 3.14;  // Explicit type
    let greeting = "Hello, Rust!";  // Inferred as &str

    println!("x: {}, y: {}, greeting: {}", x, y, greeting);

    // Type mismatch error: won't compile
    // let sum = x + y;  // i32 + f64 not allowed implicitly
    let sum = x as f64 + y;  // Explicit cast
    println!("Sum: {}", sum);
}
```

- Here, `x` is inferred, but mixing types requires casts.
- This enforces safety: no accidental overflows or precision loss.

## 2. Custom Types: Structs and Enums

Rust lets you define your own types for better modeling.

### Structs

Structs bundle data. They can be tuple-like, named, or unit.

```rust
// Named struct
struct User {
    username: String,
    email: String,
    active: bool,
}

// Tuple struct
struct Color(u8, u8, u8);

// Unit struct (no fields)
struct AlwaysActive;

fn main() {
    let user = User {
        username: String::from("alice"),
        email: String::from("alice@example.com"),
        active: true,
    };

    let rgb = Color(255, 0, 0);

    println!("User: {} ({}), Active: {}", user.username, user.email, user.active);
    println!("Red: {}", rgb.0);
}
```

- Access fields with dot notation.
- Structs can implement methods (via `impl` blocks, covered later with traits).

### Enums

Enums represent variants, often with data. They're algebraic data types (ADTs), enabling sum types.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },  // Anonymous struct
    Write(String),
    ChangeColor(Color),  // Using another type
}

fn process_message(msg: Message) {
    match msg {  // Pattern matching from previous lecture!
        Message::Quit => println!("Quitting"),
        Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
        Message::Write(text) => println!("Writing: {}", text),
        Message::ChangeColor(Color(r, g, b)) => println!("Color: ({}, {}, {})", r, g, b),
    }
}

fn main() {
    let msg = Message::Write(String::from("Hello"));
    process_message(msg);
}
```

- Enums tag values, preventing invalid states (e.g., no nulls; use `Option<T>` instead).

## 3. Generics: Reusable Code

Generics parameterize types, like templates in C++. They enable type-safe, efficient code without duplication.

### Generic Functions and Structs

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let numbers = vec![34, 50, 25, 100];
    println!("Largest number: {}", largest(&numbers));

    let floats = vec![3.14, 2.71, 1.618];
    println!("Largest float: {}", largest(&floats));

    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: 1.0, y: 4.0 };
    // let mixed = Point { x: 5, y: 4.0 };  // Error: types must match
}
```

- `T: PartialOrd` is a trait bound: T must implement partial ordering (for comparison).
- Monomorphization: Compiler generates specific code for each type at compile time, zero runtime cost.

### Multiple Generics and Bounds

You can have multiple params and complex bounds.

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

fn display<T: std::fmt::Display, U: std::fmt::Display>(pair: &Pair<T, U>) {
    println!("First: {}, Second: {}", pair.first, pair.second);
}
```

## 4. Traits: Defining Behavior

Traits are like interfaces: they define methods that types can implement. Key to polymorphism.

### Defining and Implementing Traits

```rust
trait Summary {
    fn summarize(&self) -> String;  // Required method
    fn default_summary(&self) -> String {  // Default implementation
        String::from("Read more...")
    }
}

struct NewsArticle {
    headline: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}: {}", self.headline, &self.content[..50])
    }
}

fn notify(item: &impl Summary) {  // Trait bound in function
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 2.0 Released"),
        content: String::from("Exciting new features..."),
    };
    notify(&article);
    println!("Default: {}", article.default_summary());
}
```

- `impl Trait` for args allows any type implementing the trait.
- Traits can be bounds for generics: `fn generic_notify<T: Summary>(item: &T)`.

### Trait Objects for Dynamic Dispatch

For runtime polymorphism:

```rust
fn main() {
    let summaries: Vec<Box<dyn Summary>> = vec![
        Box::new(NewsArticle { /* fields */ }),
        // Another type implementing Summary
    ];

    for s in summaries {
        println!("{}", s.summarize());
    }
}
```

- `dyn Trait` uses vtables for dynamic dispatch (small perf cost).

## 5. Lifetimes: Managing References

Lifetimes ensure references are valid, preventing dangling pointers. Annotated with `'a` (tick-a).

### Basic Lifetime Annotation

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(&string1, &string2);
        println!("Longest: {}", result);
    }
    // println!("{}", result);  // Error: result's lifetime tied to string2, which is dropped
}
```

- `'a` says the return reference lives as long as the shortest input reference.
- Compiler infers most lifetimes; explicit when ambiguous.

### Structs with Lifetimes

```rust
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let e = Excerpt { part: first_sentence };
}
```

- Ensures `part` doesn't outlive `novel`.

## 6. Advanced Topics: Associated Types and More

- **Associated Types**: Traits can define types, e.g., `Iterator::Item`.
- **Newtype Pattern**: Wrap types for added safety, like `struct Millimeters(u32)`.
- **Type Aliases**: `type Kilometers = i32;` for readability.
- **Never Type (`!`)**: For functions that never return (e.g., `panic!`).

## 7. Best Practices and Pitfalls

- **Leverage Inference**: Don't over-annotate; let the compiler help.
- **Use Enums Over Booleans**: Model states explicitly to avoid invalid combos.
- **Trait Bounds for Flexibility**: Prefer over concrete types for reusable code.
- **Lifetime Elision**: Rust elides common patterns; learn them to write less.
- **Common Errors**: Mismatched types, borrowing rules violations—read compiler messages!
- **Performance**: Zero-cost abstractions mean generics/traits are as fast as hand-written code.

Rust's type system might feel strict at first, but it catches bugs early and leads to reliable software. Try implementing a small project using these features. Any questions? Drop them below!

Happy Rust-ing! 🦀
