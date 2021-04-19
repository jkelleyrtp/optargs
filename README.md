# OptArgs: Optional arguments for Rust functions

Enable optional arguments for any function:

```rust
#[optargs::optfn]
pub fn plot(
    x: Vec<i32>,
    y: Option<Vec<i32>>,
    title: Option<&str>,
    xlabel: Option<&str>,
    ylabel: Option<&str>,
    legend: Option<bool>
) {}

// can now call it with optional arguments
plot!(
    x: vec![1,2,3], 
    y: vec![1,2,3], 
    title: "Awesome plot", 
    xlabel: "x axis", 
    ylabel: "y axis"
);
```

...or struct:

```rust
#[derive(optargs::OptStruct)]
pub struct Scatter {
    x: Vec<i32>,
    y: Option<Vec<i32>>,
    title: Option<&str>,
    xlabel: Option<&str>,
    ylabel: Option<&str>,
    legend: Option<bool>
}
impl Scatter {
    fn plot(self) {/* custom impl that references self */}
}

// Call it
Scatter!{ x: vec![1,2,3], y: vec![1,2,3] }.plot()
```

This crate is especially useful for cleaning up builder-heavy codebases and making library APIs more ergonomic. It also integrates well with Rust-Analyzer and doesn't generate heavy compile times.

---

This crate adds two macros to make it easy to add optional arguments to functions.
- `#[optargs]` - derive a `macro_rules` to call a function with optional arguments.
- `#[derive(OptStruct)]` - derive a typed-builder builder for a struct with optional fields.

This crate takes advantage of the recent const_generics in Rust stable (1.51), so our MSRV is 1.51.

Of note:
- All optional arguments will default to none. We currently don't provide a custom default (accepting PRs if anyone wants!).
- All optional arguments must come *after* required arguments.
- Unnamed positional arguments *must* be in the correct position.
- All arguments *can* be required, but now you get to name them.

## How it works:
OptArgs uses const generics to ensure compile-time correctness. I've taken the liberty of expanding and humanizing the macros in the reference examples. 
