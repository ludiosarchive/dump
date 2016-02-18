dump
===

dump provides a macro `dump!` that takes one or more variables and prints the name, type, and value of each variable. The output is also prefixed with `[source_file:line_no]`.

This is designed to be a little more ergonomic than `println!("a={:?} b={:?}", a, b)` and also has the benefit of printing the type.

[![](http://meritbadge.herokuapp.com/dump)](https://crates.io/crates/dump)

### Usage

Add this to your Cargo.toml:

```
[dependencies]
dump = "0.1"
```

and this to your crate root:

```
#[macro_use]
extern crate dump;
```

### Example

```rust
#[macro_use]
extern crate dump;

fn main() {
	let s = "hi";
	let n = 3;
	dump!(s, n);
}
```

Outputs:

```
[src/main.rs:7] s: &'static str = "hi"; n: i32 = 3;
```
