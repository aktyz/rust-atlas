## Chapter 1 - Getting Started

### 1.1. Installation

Usefull bash commands for handling Rust tooling:
```bash
rustc --version
rustup update
rustup doc # opening the Book in your browser
```

### 1.2. Hello, World!

Compiling Rust code:
```bash
cd hello_world
rustc main.rs
```
Running/executing compiled program:
```bash
./main
```

Running Rust formatter on your Rust code
```bash
rustfmt main.rs
```

### 1.3. Hello, Cargo!

Usefull Cargo commands:
```bash
cargo --version       # check if you have Cargo installed
cargo new hello_cargo # create a new Cargo project
cargo fmt             # running Rust fomratter on Cargo project
```

Cargo initializes a new project, depending if your directory has alredy Git initialized:
- if you already have git - `cargo new <project_name>` WILL **NOT** initialize Git for you, **unless** you use:
```bash
cargo new --vcs=git <project_name>
```
- if you don't have git - `cargo new <project_name>` WILL initialize Git and `.gitignore` for you

> In Rust, packages of code are referred as **CRATES**

Building and Running a Cargo Project:
```bash
cargo build                # creates an executable file: `target/debug/hello_cargo`
                           # default build is a debug build
./target/debug/hello_cargo # run the executable
```

Compile the code and then run the resultant executable all in one command:
```bash
cargo run
```

Check your code to make sure it compiles but doesn’t produce an executable:
```bash
cargo check
```

Running "production build" that is optymized for running speed:
```bash
cargo build --release # creates an executable file: `target/release/hello_cargo`
```

> The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. If you’re benchmarking your code’s running time, be sure to run `cargo build --release` and benchmark with the executable in target/release.

## Chapter 2 - Guessing Game

> The **PRELUDE** is the list of things that Rust automatically imports into every Rust program. It’s kept as small as possible, and is focused on things, particularly traits, which are used in almost every single Rust program.

In Rust, variables and references are immutable by default, meaning once we give the variable a value, the value won’t change:

```rs
let apples = 5; // treated as a constant by the program

let mut bananas = 42; // adding `mut` allows to modify the variable
```

> `String::new`, a function that returns a new instance of a `String`. `String` is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text.

[Crates.io](https://crates.io/) is where people in the Rust ecosystem post their open source Rust projects for others to use.

```bash
cargo doc --open # generates documentation for your Cargo project
```

> A `match` expression is made up of `arms`. An `arm` consists of a **PATTERN** to match against, and the code that should be executed if the value given to `match` fits that `arm`'s pattern. Rust takes the value given to `match` and looks through each `arm`'s pattern in turn.

> **SHADOWING** lets us reuse the `guess` variable name rather than forcing us to create two unique variables such as `guess_str` and `guess`.
