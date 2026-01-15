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

## 4. Control Flow with `if` Statements ✅

### Concepts Learned:
- **Conditional expressions with `if`, `else if`, `else`**
- **Type consistency in if branches** (all branches must return same type)
- **Multiple condition handling**
- **`if` as expression** (returns values)

### Completed Exercises:

#### if1.rs - Comparing Numbers:
```rust path=null start=null
fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
```

#### if2.rs - Type Consistency:
```rust path=null start=null
fn picky_eater(food: &str) -> &str {
    if food == "strawberry" {
        "Yummy!"
    } else if food == "lemon" {
        "When life gives you lemons..."
    } else {
        "I'll have a bite of that!"
    }
}
```

#### if3.rs - Type Consistency in Expressions:
```rust path=null start=null
let animal = "crab";
let identifier = if animal == "crab" {
    1
} else if animal == "gopher" {
    2
} else if animal == "snake" {
    3
} else {
    0  // All branches must return same type (i32)
};
```

**Key Learning**: All branches of an `if` expression must return the same type. Type checking is enforced at compile time, preventing type mismatches.

---

## 5. Primitive Types ✅

### Concepts Learned:
- **Boolean types (`bool`)** - `true` or `false`
- **Character types (`char`)** - Single Unicode character in single quotes
- **Numeric types** - Integers (`i32`, `i64`) and floats (`f32`, `f64`)
- **Type consistency** - Each variable has a fixed type
- **Array types** - Fixed-size collections of same type

### Completed Exercises:

#### Booleans (primitive_types1.rs):
```rust path=null start=null
fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning!");
    }
    
    let is_evening = false;
    if is_evening {
        println!("Good evening!");
    }
}
```

#### Characters (primitive_types2.rs):
```rust path=null start=null
fn main() {
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    }
    
    let your_character = 'R';
    println!("Your character is: {}", your_character);
}
```

#### Numeric Types (primitive_types4-6.rs):
```rust path=null start=null
let number: u8 = 3;      // Unsigned 8-bit integer
let float: f32 = 3.14;   // 32-bit floating point
let sum: i32 = number as i32 + 5;  // Type casting
```

**Key Learning**: Rust has distinct `char` type using single quotes (different from strings). Type inference is powerful but explicit type annotations increase clarity.

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
4. **Control Flow** - if/else expressions, type consistency, comparisons
5. **Primitive Types** - booleans, characters, integers, floats, arrays
6. **Slices** - Array slicing and borrowing
7. **Tuples & Destructuring** - Tuple unpacking for cleaner code
8. **Vectors & Iterators** - Iterator patterns and practical usage

### ⚠️ In Progress/Need Work:
1. **Move Semantics** - Advanced ownership concepts started

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

---

## 7. Slices (slices1) ✅

### Concepts Learned:
- **Array slicing syntax**: `&array[start..end]` where end is exclusive
- **Slices are zero-copy references** to contiguous data in arrays
- **Borrowing without ownership transfer**
- **Slice type is `&[T]`**

### Code Example:

```rust path=null start=null
fn main() {
    let a = [1, 2, 3, 4, 5];
    let nice_slice = &a[1..4];  // Slice from index 1 to 3
    
    assert_eq!([2, 3, 4], nice_slice);
}
```

**Key Learning**: Slices provide safe, zero-copy access to array elements without full ownership.

---

## 8. Tuples & Destructuring (tuples1) ✅

### Concepts Learned:
- **Tuples are fixed-size collections** of different types: `(Type1, Type2, ...)`
- **Tuple destructuring**: unpacking values into named variables
- **Destructuring syntax**: `let (var1, var2) = tuple;`
- **More idiomatic than index access** (tuple.0, tuple.1)
- **Increases code readability** with named variables

### Code Example:

```rust path=null start=null
fn main() {
    let cat = ("Furry McFurson", 3.5);
    
    // Destructure tuple into named variables
    let (name, age) = cat;
    
    println!("{name} is {age} years old");
}
```

**Key Learning**: Destructuring is idiomatic Rust and preferred over index-based access for code clarity.

---

## 9. Vectors & Iterators (vecs2) - Iterator vs Loop Analysis ✅

### Question: Which approach is more commonly used by Rust developers?
### Answer: **`iter().map(...).collect()` is MOST COMMON**

### Why Rustaceans Prefer Iterators:
- **Declarative** - expresses intent clearly
- **Composable** - chains easily with filter, enumerate, etc.
- **Safer** - harder to introduce logic errors
- **Idiomatic** - reads like functional/mathematical code

### Most Common Pattern:
```rust path=null start=null
input.iter().map(|x| x * 2).collect()
```

### When to Use Traditional For Loops:
- **Early breaks or continues** needed
- **Complex conditional logic** with multiple branches
- **Multiple side effects** in loop body
- **Performance tuning** with `with_capacity()`

### Traditional Loop Pattern (When Appropriate):
```rust path=null start=null
let mut out = Vec::new();
for x in input {
    out.push(x * 2);
}
```

### What Senior Rust Developers Actually Write:

**Optimized Loop Version (when performance matters):**
```rust path=null start=null
let mut out = Vec::with_capacity(input.len());
for &x in input {
    out.push(x * 2);
}
```

**Clean Iterator Version (preferred default):**
```rust path=null start=null
input.iter().map(|&x| x * 2).collect()
```

*Note: The `|&x|` pattern avoids implicit deref and is preferred in code reviews.*

### Idiomatic Rust Guideline 🧠
**Start with iterators → drop to loops only when logic demands it**

*This is opposite to many other languages where loops are the default choice.*

### Comparison Table:

| Aspect | Iterators | For Loops |
|--------|-----------|----------|
| Readability | Excellent (declarative) | Good (explicit) |
| Composability | Great (chainable) | Limited |
| Safety | Very safe (functional) | Requires care |
| Performance | Excellent (zero-cost abstractions) | Excellent (with capacity) |
| Default Choice | ✅ YES | Only when needed |

**Key Learning**: Rust's iterator pattern is preferred by default, but pragmatism wins when loop logic is complex. This mindset is characteristic of idiomatic Rust development.

---

## Updated Progress Summary

### ✅ Recently Completed Sections:
1. **Slices (slices1)** - Array slicing and borrowing
2. **Tuples & Destructuring (tuples1)** - Tuple unpacking for cleaner code
3. **Vectors & Iterators (vecs2)** - Iterator patterns and practical usage

### 🎯 Key Paradigm Shift:
Rust developers think **functionally by default** with iterators, unlike imperative languages where loops are the norm. This is a fundamental difference in Rust's philosophy.

---

## Final Progress Summary

### Major Topics Covered:
- ✅ Fundamentals (intro, variables, functions)
- ✅ Control structures (if expressions, type consistency)
- ✅ Primitive types (bool, char, numbers, arrays)
- ✅ Collections (tuples, slices, vectors)
- ✅ Functional patterns (iterators, map, collect)
- ⚠️ Ownership & borrowing (in progress)

### Idiomatic Rust Principles Mastered:
1. **Immutability by default** - Must explicitly declare `mut`
2. **Expression-based thinking** - Last expression returns value
3. **Type safety** - Compiler enforces type consistency
4. **Functional preference** - Iterators over loops by default
5. **Memory safety** - Ownership prevents common bugs

---

## Last updated: January 15, 2026
