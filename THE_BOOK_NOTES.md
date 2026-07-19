## Chapter 1 - Getting Started

### 1.1. Installation

Usefull bash commands for handling Rust tooling:
```bash
rustc --version
rustup update
rustup doc #opening the Book in your browser
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

