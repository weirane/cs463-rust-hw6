# CS 463 — Homework 6: Rust

**Due:** Monday, November 2nd, 5pm

*You may collaborate with others and online resources. Work must be uploaded to GradeScope.*

In this homework, implement the same nine functions from Homework 1 (Python) and Homework 4 (Haskell), now in Rust. The behavior is the same unless a Rust type or input range is specified below.

## Installation

### Getting Rust and Cargo

Install Rust locally using `rustup`, as shown in the lecture slides. This installs the Rust compiler (`rustc`) and Cargo, which builds your project and runs its tests. See the [official Rust installation instructions](https://rust-lang.org/tools/install/) for more details.

On **macOS, Linux, or Windows Subsystem for Linux (WSL)**, open a terminal and run:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions and choose the default installation.

On **Windows**, download and run [`rustup-init.exe` for x86_64](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe), then follow the on-screen instructions. If prompted, install the Visual Studio C++ Build Tools required by Rust. For Windows on ARM, use the ARM64 installer linked on the official installation page. If you use WSL, run the command above inside your WSL terminal instead.

After installation, close and reopen your terminal, then check that both tools are available:

```sh
rustc --version
cargo --version
```

Both commands should print a version number. If either command is not found, follow the `PATH` configuration guidance on the official installation page.

### Libraries and testing

This homework uses only the Rust standard library and Rust's built-in testing framework. You do not need to install any additional libraries or add dependencies. Cargo is included with Rust; use `cargo test` from the supplied project directory (the directory containing `Cargo.toml`) as described below.

## Setup and submission

Download the starter zip for homework 6 from our class directory:
[h6.zip](https://people.cs.gmu.edu/~marks/463/h/h6.zip)



The supplied Cargo project has `Cargo.toml`, `src/lib.rs`, `src/tests.rs`, and `src/main.rs`. Replace the `todo!()` bodies in `src/lib.rs` with your implementations. Keep the function names and signatures unchanged. The line `#[cfg(test)] mod tests;` at the bottom of `src/lib.rs` loads the separate tester module.

Run all supplied tests from the project directory with:

```sh
cargo test
```

To run tests for one function, use a filter such as `cargo test fib` (all tests with `fib` in the name will be run). The starter compiles, but its tests will fail until you replace the `todo!()` bodies. Passing the supplied tests does not guarantee full credit; your functions must work throughout the stated input ranges. Submit **only `src/lib.rs`** to the Homework 6 assignment on Gradescope. Put your name, NetID, and any collaborators or resources used in comments at the top of that file.

Use the Rust standard library only; do not add dependencies to `Cargo.toml`. You may write helper functions. Do not substitute a library operation that performs an assigned task for your implementation: for example, do not use `reverse` or `rev` to implement `reversed`, or `Iterator::zip` to implement `zip_with`. Sorting is allowed for `median` and `mode`. Do not hardcode the examples or tests.

### How do I do manual testing?

You can use the provided `src/main.rs` and edit the `main()` function to call your functions with test inputs. You can then use `cargo run` to compile and run it.

```rust
use h6::*; // import your functions

fn main() {
    println!("fib(10) = {}", fib(10));
    println!("{:?}", reversed(&[1, 2, 3])); // use {:?} to print a vector
}
```

## Required functions

### 1. Fibonacci

```rust
pub fn fib(n: u128) -> u128
```

Given a non-negative integer n, calculate and return the nth fibonnaci number. You will need to handle larger inputs. You still might want to warm up with a slow_fib definition, but eventually you need to be faster about it to get full credit (aim for O(n) ). The beginning of the list of fibonacci numbers is [0, 1, 1, 2, 3, 5, 8, ...].

Examples: `fib(0) == 0`, `fib(1) == 1`, `fib(10) == 55`.

### 2. Reversed

```rust
pub fn reversed<T: Clone>(xs: &[T]) -> Vec<T>
```

Given a list of values, create a new list with the same values in reversed order. 

Example: `reversed(&[1, 2, 3, 2]) == vec![2, 3, 2, 1]`.

### 3. Prime check

```rust
pub fn is_prime(n: i64) -> bool
```

Check if the argument is prime (is only divisible by 1 and itself). The smallest prime is 2. 

Examples: `is_prime(-3) == false`, `is_prime(2) == true`, `is_prime(1117) == true`.

### 4. Nub

```rust
pub fn nub<T: PartialEq + Clone>(xs: &[T]) -> Vec<T>
```

Return a list of all values that show up in the argument at least once. The answer must not include any duplicates, and must report the occurring values in the same order as their first occurrence. Think of this as returning a copy of the list with all duplicates removed.

Example: `nub(&[3, 2, 1, 2, 3, 4]) == vec![3, 2, 1, 4]`.

### 5. Zip with

```rust
pub fn zip_with<A, B, C, F>(f: F, xs: &[A], ys: &[B]) -> Vec<C>
where
    F: FnMut(&A, &B) -> C,
{
    // Your implementation.
}
```

Given a two-argument function and two lists of arguments to supply, create a list of the results of applying the function to each same-indexed pair of arguments from the two lists. (Zip up the two lists with the function).

- if either list runs out of items, ignore the rest of the longer list.

Example: `zip_with(|x, y| x + y, &[1, 2, 3], &[10, 20]) == vec![11, 22]`.

### 6. Collatz

```rust
pub fn collatz(n: u64) -> Vec<u64>
```

Given a positive integer n, we generate successive integer values in a list, which must end with a 1. The rules for successive values are:

- if n is 1, the sequence ends.
- if n is even, the next number is n/2.
- if n is odd, the next number is n*3+1.

Examples: `collatz(1) == vec![1]`, `collatz(3) == vec![3, 10, 5, 16, 8, 4, 2, 1]`.

### 7. Median

```rust
pub fn median(xs: &[i64]) -> f64
```

- Purpose: given a list that contains one or more integers, calculate and return the median value (the middle value when the values are in sorted order).
- when there's an even number of items, there's no true middle item - and we return the average of the two closest to the middle (hence the Double return type).

Examples: `median(&[3, 4, 5, 1, 2]) == 3.0`, `median(&[1, 3, 6, 8]) == 4.5`.

### 8. Mode

```rust
pub fn mode(xs: &[i64]) -> Vec<i64>
```

- Purpose: given a list that contains one or more integers, calculate and return the mode(s).
- the mode must be a list of all values that share the most occurrences, ordered increasing (go ahead and sort it whenever it's useful to you). This list might only have one value in it but will always be a list.

Examples: `mode(&[1, 2, 3]) == vec![1, 2, 3]`, `mode(&[3, 1, 2, 3, 1]) == vec![1, 3]`.

### 9. Check Sudoku

```rust
pub fn check_sudoku(grid: &[[u8; 9]; 9]) -> bool
```

Given a 9x9 2d list, check if it represents a valid, solved sudoku.

- We aren't solving a sudoku, we're only checking someone's completed attempt. That means that we are checking that:
  - each of the nine rows must contain all digits 1 through 9
  - each of the nine columns must contain all digits 1 through 9
  - when we split the grid into three 3x3 groups that don't overlap, each of these groups must contain all digits 1 through 9.
