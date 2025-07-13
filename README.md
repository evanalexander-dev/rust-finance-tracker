# Overview

I wanted to deepen my understanding of system programming languages, particularly those that emphasize memory safety without sacrificing performance. This project explores Rust's unique ownership model and how it prevents common programming errors while maintaining zero-cost abstractions.

I built a personal finance tracker that demonstrates Rust's core language features, including ownership, borrowing, pattern matching, and error handling. The application manages financial transactions using Rust's powerful type system and standard library collections.

My purpose was to gain hands-on experience with Rust's ownership system, understand how borrowing works in practice, and explore Rust's approach to memory management. I also wanted to see how Rust's functional programming features integrate with traditional imperative code.

# Development Environment

I developed this software using RustRover IDE from JetBrains on Arch Linux.

The programming language used is Rust (1.88.0). I used Rust's standard library exclusively, specifically the 
`std::collections::HashMap` for category management, `std::fs` for file operations, and `std::io` for user input/output.

# Useful Websites

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)—Official comprehensive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)—Practical examples of Rust concepts
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)—Complete API reference

# Future Work

- Add data validation for transaction categories
- Implement transaction editing and deletion features
- Add support for multiple currencies and exchange rates
