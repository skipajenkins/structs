
# 🦀 Rust — Structs Project

---

A project demonstrating structs, tuple structs, struct update syntax, destructuring, and basic geometry functions in Rust.

This follows along with The Rust Programming Language (“The Rust Book”) and showcases how structs are defined, instantiated, updated, and used in functions.

---

## 📌 Prerequisites

Before running this project, ensure the following tools are installed:

### ✔️ Install Rust
Download Rust & Cargo (comes bundled together):
```bash
https://www.rust-lang.org/tools/install
```

Verify installation:
```
rustc --version
cargo --version
```

### 📁 Project Setup

Create a new Rust project (if reproducing manually):
```bash
cargo new structs
cd structs
```

Then replace the main.rs file with the code provided in this repository.

To run the project:
```bash
cargo run
```

---

## 📘 What This Project Demonstrates
✅ 1. Defining and Instantiating Structs

This project introduces a standard User struct:
```bash
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
```

An instance is created and modified:
```bash
let mut user1 = User {
    active: true,
    username: String::from("Ferris"),
    email: String::from("Ferris@gmail.com"),
    sign_in_count: 4,
};

user1.email = String::from("Ferris1@gmail.com");
```

✅ 2. Struct Update Syntax (..user1)

Create a new struct by copying fields from another:
```bash
let user2 = User {
    username: String::from("Ferris2.0"),
    ..user1
};
```

This moves remaining fields from user1 into user2.

✅ 3. Tuple Structs

Rust also allows structs without named fields:
```bash
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;
```

Example usage:
```bash
let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

Tuple structs allow storing related values when naming fields isn't necessary.

✅ 4. Destructuring Structs
```bash
let Point(x, y, z) = origin;
```

This extracts values from a tuple struct into variables.

✅ 5. Struct Debug Printing
```bash
Using #[derive(Debug)]:

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

println!("{rect2:#?}");
```

This prints the struct in a human-readable format.

✅ 6. Functions That Accept Structs

A rectangle area function:
```bash
fn _area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

As well as geometric functions using primitive parameters and tuples:
```basj
fn area_of_rectangle(x: i32, y: i32) -> i32 { x * y }
fn area_of_square(x: i32) -> i32 { x * x }
fn area(dimensions: (u32, u32)) -> u32 { dimensions.0 * dimensions.1 }
```
▶️ Running the Program

Run:
```
cargo run
```

The output demonstrates:

Struct creation & mutation

Tuple struct values

Debug printing

Geometry calculations

Struct update syntax in action

Destructuring tuple structs

---

## 📜 Example Output (Simplified)
```
Area of rectangle : 50
Area of square : 25.
rect2 is Rectangle {
    width: 30,
    height: 70,
}
[src/main.rs:xx] &rect1 = (
    30,
    50,
)
```

---

## 🎯 Learning Goals

By completing this project, you learn:

* How Rust stores and manages structured data

* How to define and instantiate structs

* How to mutate struct fields

* How to use tuple structs

* How ownership interacts with struct fields

* How to write functions that take structs as parameters

* How to print structs using Debug

* How to destructure values

---

## 🦀 Built With

Rust

Cargo

---

## 📄 License

This project is open-source and available under the MIT License
