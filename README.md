# Rust_Book
 This repository mainly created to practice the Rust language. Based on the online [documentation](https://doc.rust-lang.org/book/) that provide by the Rust Official site.

## Pre-requisites
1. Rust Compiler Version: 1.74.1
2. Cargo(Rust package manager) Version: 1.74.1

## How to Start

Creating a new Rust project is easy with Cargo, the Rust package manager. Here's how to do it:

### 1. Open a terminal:

Navigate to the directory where you want your project to be created.

### 2. Run the cargo new command:

```
cargo new my_project
```
Replace my_project with your desired project name.

### 3. Optional arguments:

* ```--bin```: Create a binary executable (default).
* ```--lib```: Create a library instead.
* ```--vcs none```: Skip initializing a git repository.
### 4. Project structure:

* ```Cargo.toml```: Defines project metadata, dependencies, and build configuration.
* ```src/main.rs```: Entry point for your program (if creating a binary).
Other directories and files can be added as needed.
### 5. Run your project:

To run a binary project, execute:

```
cargo run
```
This compiles and runs your code.

#### Here are some additional resources:

* [The Cargo Book](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)
* [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
* [The Rust Book](https://doc.rust-lang.org/book/)