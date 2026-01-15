# Rust Programming Concepts - Rustlings Learning Notes

## Overview
These notes document the Rust programming concepts learned through completing rustlings exercises. Each section includes the key concepts, code examples from your solutions, and explanations.

---

## 1. Introduction & Basic Setup ✅

### Concepts Learned:
- **Basic Rust syntax and structure**
- **`println!` macro usage**
- **Raw string literals with `r#"..."#`**
- **Program entry point with `main()` function**

### Code Examples:

```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\00_intro\intro1.rs start=8
fn main() {
    println!(r#"       Welcome to...                      "#);
    println!(r#"                 _   _ _                  "#);
    println!(r#"  _ __ _   _ ___| |_| (_)_ __   __ _ ___  "#);
    println!(r#" | '__| | | / __| __| | | '_ \ / _` / __| "#);
    println!(r#" | |  | |_| \__ \ |_| | | | | | (_| \__ \ "#);
    println!(r#" |_|   \__,_|___\__|_|_|_| |_|\__, |___/ "#);
    println!(r#"                               |___/      "#);
    println!();
    println!("This exercise compiles successfully.");
}
```

```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\00_intro\intro2.rs start=1
fn main() {
    // Fixed: Added the correct string
    println!("Hello world!");
}
```

**Key Learning**: Understanding basic program structure and output formatting.

---

## 2. Variables and Mutability ✅

### Concepts Learned:
- **Variable declaration with `let`**
- **Type annotations (`: i32`)**
- **Mutability with `mut` keyword**
- **Variable shadowing**
- **Constants with `const`**
- **Immutability by default**

### Code Examples:

#### Variable Declaration:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\01_variables\variables1.rs start=2
let x = 5; // Immutable variable
println!("x has the value {x}");
```

#### Type Annotations:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\01_variables\variables2.rs start=3
let x: i32 = 10; // Explicit type annotation
```

#### Mutability:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\01_variables\variables4.rs start=3
let mut x = 3; // Mutable variable
println!("Number {x}");
x = 5; // Can reassign because of `mut`
println!("Number {x}");
```

#### Variable Shadowing:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\01_variables\variables5.rs start=2
let number = "T-H-R-E-E"; // String type
println!("Spell a number: {number}");

let number = 3; // Shadow with different type (integer)
println!("Number plus two is: {}", number + 2);
```

#### Constants:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\01_variables\variables6.rs start=2
const NUMBER: i32 = 3; // Constants must have type annotation
```

**Key Learning**: Rust variables are immutable by default, requiring explicit `mut` for mutability.

---

## 3. Functions ✅

### Concepts Learned:
- **Function definition syntax**
- **Parameter types must be explicitly declared**
- **Return types with `-> Type`**
- **Expression vs Statement**
- **Implicit returns (no semicolon)**

### Code Examples:

#### Basic Function:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\02_functions\functions1.rs start=7
fn call_me() {
    // Function with no parameters or return value
}
```

#### Function with Parameters:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\02_functions\functions2.rs start=2
fn call_me(num: i32) { // Parameter type must be specified
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
```

#### Function with Return Type:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\02_functions\functions4.rs start=6
fn is_even(num: i64) -> bool {
    num % 2 == 0 // Expression (no semicolon) = return value
}

fn sale_price(price: i64) -> i64 {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}
```

#### Expression Return:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\02_functions\functions5.rs start=2
fn square(num: i32) -> i32 {
    num * num // No semicolon = expression that returns
}
```

**Key Learning**: Function parameters need explicit types, and final expressions return values automatically.

---

## 4. Control Flow with `if` Statements ⚠️

### Concepts Identified (Need Completion):
- **Conditional expressions**
- **Type consistency in if branches**
- **Multiple condition handling**

### Incomplete Exercises Found:

#### if1.rs - Missing Implementation:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\03_if\if1.rs start=1
fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // SOLUTION NEEDED: if a > b { a } else { b }
}
```

#### if2.rs - Type Mismatch Error:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\03_if\if2.rs start=2
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else {
        1 // ERROR: returning number instead of string
    }
}
// SOLUTION NEEDED: Different string returns for different cases
```

#### if3.rs - Mixed Types Error:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\03_if\if3.rs start=3
let identifier = if animal == "crab" {
    1
} else if animal == "gopher" {
    2.0 // ERROR: Mixed integer and float types
} else if animal == "snake" {
    3
} else {
    "Unknown" // ERROR: Mixed number and string types
};
```

**Key Learning**: All branches of an `if` expression must return the same type.

---

## 5. Primitive Types ⚠️

### Concepts Identified (Partial Completion):
- **Boolean types (`bool`)**
- **Character types (`char`)**
- **Arrays**
- **Type consistency**

### Examples Found:

#### Booleans:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\04_primitive_types\primitive_types1.rs start=4
let is_morning = true;
if is_morning {
    println!("Good morning!");
}
// TODO: Need to complete with is_evening boolean
```

#### Characters:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\04_primitive_types\primitive_types2.rs start=6
let my_first_initial = 'C'; // Single quotes for char
if my_first_initial.is_alphabetic() {
    println!("Alphabetical!");
}
// TODO: Need to declare your_character variable
```

**Key Learning**: Rust has distinct `char` type using single quotes, different from strings.

---

## 6. Move Semantics & Ownership ⚠️

### Concepts Identified (Advanced Topic):
- **Ownership transfer**
- **Mutable references**
- **Move vs Clone**

### Examples Found:

#### Basic Move Semantics:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\06_move_semantics\move_semantics1.rs start=2
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let vec = vec; // Takes ownership
    vec.push(88); // ERROR: vec is immutable
    vec
}
```

#### Solution with Mutability:
```rust path=C:\Users\pardh\Desktop\Rust\rustlings\exercises\06_move_semantics\move_semantics2.rs start=1
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec; // Make mutable to modify
    vec.push(88);
    vec
}
// Issue: Original vector is moved and no longer accessible
```

**Key Learning**: Ownership is transferred when passing values to functions.

---

## Summary of Current Progress

### ✅ Completed Concepts:
1. **Introduction & Setup** - Basic Rust program structure
2. **Variables** - Declaration, mutability, shadowing, constants
3. **Functions** - Parameters, return types, expressions vs statements

### ⚠️ Partially Completed/Need Work:
1. **Control Flow** - if statements need completion
2. **Primitive Types** - boolean and char exercises incomplete
3. **Move Semantics** - Advanced ownership concepts started

### 📝 Key Rust Principles Learned:
1. **Immutability by Default** - Variables are immutable unless marked `mut`
2. **Explicit Typing** - Function parameters and some variables need type annotations
3. **Expression-based** - Functions return the last expression automatically
4. **Memory Safety** - Ownership system prevents common programming errors
5. **Zero-cost Abstractions** - High-level features compile to efficient code

---

## Next Steps for Learning:

1. **Complete Control Flow exercises** (if1.rs, if2.rs, if3.rs)
2. **Finish Primitive Types** (arrays, tuples, slices)
3. **Master Move Semantics** (ownership, borrowing, references)
4. **Progress to Collections** (vectors, hashmaps)
5. **Learn Error Handling** (Result, Option types)
6. **Study Advanced Features** (traits, generics, lifetimes)

---

## Practice Recommendations:

1. Focus on **ownership concepts** - most important Rust feature
2. Practice **type consistency** in control flow
3. Understand **mutability rules** thoroughly
4. Write small programs applying each concept
5. Read compiler error messages carefully - they're very helpful in Rust!

*Generated from rustlings exercises analysis*  
*Last updated: September 21, 2025*