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
- Traditional macro_rules scoping applies IE you can't use the macro before function declaration. However, they are exported with macro_export, so you can use them anywhere with `crate::$MACRO`. Currently, there's no way to disable this, so you can't have two functions with the same name. If this becomes a problem, we'll gladly accept a PR.

## How it works:
OptArgs uses const generics to ensure compile-time correctness. I've taken the liberty of expanding and humanizing the macros in the reference examples. 

In essence, we encode the state of required parameters into a ZST with const parameters. When each required parameter is added, we flip the const parameter from false to true. Only when all the required parameters are entered, then can we proceed with calling the original function.

```rust
struct Validator<const A: bool> {}
impl Validator<true> { fn validate(self) {} }
impl<const A: bool> Validator<A> { fn b(self) -> Validator<A> { self }}
// Not to worry about the unsafe, Validator is a ZST and has no place in memory.
// All this validation code will be removed anyways.
impl Validator<false> { fn a(self) -> Validator<true> { unsafe {std::mem::transmute(self)} } }
```


## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
