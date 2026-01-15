# WARP.md

This file provides guidance to WARP (warp.dev) when working with code in this repository.

## Repository Overview

This is a Rust learning repository containing multiple projects and exercises focused on mastering Rust programming fundamentals. The repository is organized into distinct projects, with rustlings exercises forming the core learning path.

## Project Structure

### Learning Projects
- **rustlings/** - Comprehensive Rust exercises organized by topic (00-23), including solutions
- **hellorust/** - Basic "Hello, World!" program
- **guessinggame/** - Interactive CLI game demonstrating user input, random generation, and error handling
- **euclidrust/** - GCD calculator demonstrating mathematical algorithms and testing
- **loops/** - Loop control flow practice

### Documentation
- **rustyynotes/** - Learning notes and session documentation
- **rustyypdf/** - PDF reference materials

## Common Commands

### Rustlings (Primary Learning Tool)
```powershell
# Navigate to rustlings
cd rustlings

# Run a specific exercise
cargo run --bin <exercise_name>
# Examples:
cargo run --bin variables1
cargo run --bin move_semantics2

# Run the solution for comparison
cargo run --bin <exercise_name>_sol
# Example:
cargo run --bin variables1_sol

# Check all exercises
cargo check
```

### Individual Projects
```powershell
# Build a project
cd <project_name>
cargo build

# Run a project
cargo run

# Run with release optimizations
cargo run --release

# Run tests
cargo test

# Example: Run euclidrust
cd euclidrust
cargo run
cargo test  # Runs the GCD tests
```

### Code Quality
```powershell
# Format code
cargo fmt

# Lint with Clippy
cargo clippy

# Check without building
cargo check
```

## Architecture and Key Concepts

### Rustlings Structure
Rustlings exercises are defined as individual binary targets in `Cargo.toml`, organized into 23 numbered directories (00-23):

1. **00_intro** - Print macros and basic syntax
2. **01_variables** - Variable declaration, mutability, shadowing, constants
3. **02_functions** - Parameters, return types, expressions vs statements
4. **03_if** - Conditional expressions and type consistency
5. **04_primitive_types** - Booleans, chars, integers, arrays, tuples
6. **05_vecs** - Vectors and dynamic collections
7. **06_move_semantics** - Ownership, borrowing, references (critical concept)
8. **07_structs** - Custom data structures
9. **08_enums** - Enum types and pattern matching
10. **09_strings** - String vs &str handling
11. **10_modules** - Code organization
12. **11_hashmaps** - Key-value collections
13. **12_options** - Option<T> for null safety
14. **13_error_handling** - Result<T, E> for error handling
15. **14_generics** - Generic types
16. **15_traits** - Trait definitions and implementations
17. **16_lifetimes** - Lifetime annotations
18. **17_tests** - Unit testing
19. **18_iterators** - Iterator patterns
20. **19_smart_pointers** - Box, Rc, Arc, Cow
21. **20_threads** - Concurrent programming
22. **21_macros** - Macro definitions
23. **22_clippy** - Linter exercises
24. **23_conversions** - Type conversions (as, From, Into, TryFrom)

Each exercise directory contains:
- `README.md` with concept explanations and references
- Exercise files (`.rs`)
- Solutions in `solutions/` directory

### Common Patterns in This Codebase

#### Ownership and Borrowing
The move_semantics exercises demonstrate Rust's core ownership model:
- Values have a single owner
- Ownership can be transferred (moved)
- References allow borrowing without transferring ownership
- Mutable references require explicit `mut` and enforce exclusive access

#### Testing Pattern
Projects like `euclidrust` include tests directly in `main.rs`:
```rust
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);
}
```

#### Error Handling Pattern
The guessing game demonstrates idiomatic error handling:
```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

## Development Guidelines

### When Working with Rustlings Exercises
1. Each exercise is a standalone binary - run it individually with `cargo run --bin <name>`
2. Solutions exist for reference but should be consulted after attempting the exercise
3. Compiler errors are intentionally helpful in Rust - read them carefully
4. Focus on understanding ownership and borrowing (topics 06, 16) as they are fundamental

### When Creating New Projects
1. Use `cargo new <project_name>` to create new projects in the repository root
2. Follow the existing pattern: simple `Cargo.toml` with edition = "2024"
3. Include tests using `#[test]` annotations
4. Add descriptive comments explaining Rust-specific concepts

### Rust-Specific Considerations
- **Immutability by default**: Variables require `mut` keyword to be mutable
- **Explicit types**: Function parameters always need type annotations
- **Expression-based**: Functions return the last expression (without semicolon)
- **No null**: Use `Option<T>` instead of null values
- **Error handling**: Use `Result<T, E>` instead of exceptions
- **Ownership**: Understand move semantics to avoid lifetime and borrowing issues

## Code Style

This repository follows standard Rust formatting:
- 4-space indentation
- snake_case for functions and variables
- CamelCase for types and traits
- Use `cargo fmt` before committing

## Windows-Specific Notes

This repository is developed on Windows with PowerShell:
- Use PowerShell commands (`Get-ChildItem`, `Test-Path`, etc.)
- File paths use backslashes but Rust code uses forward slashes
- Use `cargo` commands directly (no need for `.exe` suffix)

## Learning Progression

Current progress (based on repository state):
- ✅ Completed: Introduction, Variables, Functions
- 🔄 In Progress: Control Flow, Primitive Types, Move Semantics
- 📋 Upcoming: Structs, Enums, Collections, Error Handling, Traits, Lifetimes

When working on exercises, follow the numerical order as concepts build on each other.
