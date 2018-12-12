# Rust-Lang (Rust Programming Language)

- Alex Zheng
- ting.zheng@uoit.net

## About Rust

> **History**  
> Rust is a relatively new language compared to other programming languages,
> as well as its predecessors in which it is based off of. It first started out
> as a personal project in 2006 by Mozilla Research employeee Graydon Hoare.
> Rust is a systems programming language focused on memory safety while still
> maintaining high performance, as well as safe concurrency. Its creator stated
> that it was possibly named after the rust family of fungi. Rust uses LLVM 
> as its backend.
>
> **Interesting features**
> - Safe
> - Low-level
> - High-performance
> - Supports multiple paradigms of programming such as imperative, functional and object-oriented
>
> **Fun facts**
> - Its compiler is written in Rust itself and can compile its own source code
> - Won 1st place for "Most Loved Programming Language" in the Stack Overflow Developer Survey in 2016, 2017, and 2018
> - Below is the logo of Servo, a Rust project by Mozilla for an experimental web browser engine  
<img src="https://upload.wikimedia.org/wikipedia/en/d/d9/Servo_logo.png" alt="very safe, such parallel, wow, much browser" width="50px" height="50px" />

## Syntax

> Rust source code is stored in .rs files

### Basic 'Hello, World!' program:
```rust
fn main() {
    println!("Hello, World!");
}
```

## About the tools

> **Getting started with Rust (on macOS/Linux)**  
> Installation:  
> `curl https://sh.rustup.rs -sSf | sh`	- Downloads and installs *rustup*, which installs Rust (Requires a relog)
>
> Updating Rust:  
> `rustup update stable`
  
> **Cargo**  
> Rust's compiler/build tool, which also functions as a package manager for tools for Rust known as 'components'
>
> Some basic commands:
> - `cargo --version`	- Checks for and displays the current version of Rust installed
> - `cargo build`		- Build your project
> - `cargo run`			- Run your project
> - `cargo test`		- Test your project
> - `cargo update`		- Update the dependencies of your project
> - `cargo doc`			- Generate the documentation for your project
> - `cargo publish`		- Publish your project to [crates.io], the open community repository of Rust binaries and libraries

[crates.io]: https://crates.io/

## About the Rust Standard Library

> **std**  
> The Rust Standard Library contains a collection of minimal, battle-tested
> shared abstractions, offering modules of primitive and other core types,
> library-defined operations, standard macros, I/O, multithreading,
> and much more, of which may contain modules themselves.
>
> It is available to to all Rust crates by default; no need for an extra 
> `extern crate std;`. All that is needed are `use` statements for modules
> through the std path, `use std::module`, or use with expressions through the
> absolute path, `::std::module::method`.
>
> **A few primitive data type modules**
> - bool	- Boolean *(True/False)*
> - char	- Single character
> - f32		- 32-bit floating point number (also available in 64-bit with *f64*)
> - i16		- 16-bit signed integer (also available in 8/32/64/128-bit with *i8*/*i32*/*i64*/*i128*)
> - str		- String, in borrowed form, `&str`, or literal, `&'static str`, also known as a 'string slice'
> - u8		- 8-bit unsigned integer (also available in 16/32/64/128-bit with *u16*/*u32*/*u64*/*u128*)
> - unit	- Data of type `()`, sometimes called *unit* or *nil*
>
> **Some collections**
> - array		- Fixed-size array of elements of type T of non-negative constant size N, `[T; N]`
> - BinaryHeap	- Priority queue implemented with a binary maxheap
> - BTreeMap	- Map based on a binary tree
> - HashMap		- Hash map implemented with linear probing and Robin Hood bucket stealing
> - LinkedList	- Doubly-linked list of type T, `LinkedList<T>`
> - slice		- Dynamically-sized view into a contiguous sequence of type T, either shared, `&[T]`, or mutable, `&mut [T]`
> - tuple		- Finite sequence of one or more types, `(S, T, U, ..)`
> - Vec			- Contiguous growable array type of heap-allocated elements of type T, `Vec<T>`, or `vec![a, b, c, ..]`
>
> **Some other modules**
> - alloc	- For dealing with memory allocation, `std::alloc`
> - arch	- For dealing with CPU architechure specifics, `std::arch`
> - env		- For dealing with the process environment, `std::env`
> - mem		- For dealing with memory manipulation, `std::mem`
> - net		- For dealing with networking, `std::net`
> - os		- For dealing with functionality specific to certain Operating Systems, `std::os`
> - pointer	- Raw, unsafe pointers of type T, `*const T`, `*mut T`
> - process	- For working with the current and child processes, `std::process`
> - sync	- For use with program execution synchronization, `std::sync`
> - thread	- For dealing with program threading, `std::thread`
> - time	- For dealing with temporal functionality, `std::time`

## About the crate registry (Rust's repository of open source libraries)

> **Crates** are Rust binaries and libraries.
> Open-source crates are available on [crates.io](https://crates.io/), which is
> Rust's package repository. They are written and published by other developers.
> Crates can be incorporated as dependencies into your own Rust projects and
> have its functionality be called from within there. You can also publish your
> own projects to the website as crates, allowing other developers to make use
> of your creations in their own and potentially continuing the chain.
>
> **A few of the most-downloaded crates**
> - rand	- Library for random number generation
> - regex	- Library for parsing, compiling, and executing regular expressions similar to Perl-style regex
> - serde	- Framework for serializing and deserializing Rust data structures efficiently and generically
> - winapi	- Provides raw Foreign Function Interface to the Windows Operating System's APIs

# Analysis of the language

> _Organize your report according to the project description
document_.

# References
[Cargo: packages for Rust](https://crates.io/)  
[Rust](https://www.rust-lang.org/)  
[Rust Standard Library crate doc](https://doc.rust-lang.org/std/index.html)  
[Medium](https://medium.com/learning-rust/rust-basics-e73304ab35c7)  
[Wikipedia](https://en.wikipedia.org/wiki/Rust_(programming_language))
