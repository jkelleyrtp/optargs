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
- `#[derive(optbuilder)]` - derive a typed-builder builder for a struct with optional fields.

This crate takes advantage of the recent const_generics in Rust stable (1.51), so our MSRV is 1.51.

Here's how they work:

### Optfn
---
```rust
#[optargs::optfn]
fn example(a: u32, b: Option<&str>) -> bool {
    todo!()
}

// now you can use the function as a macro with optional named arguments
let _ = example!(a: 10, b: "abc");
```

Of note:
- All optional arguments will default to none. We don't provide a custom default.
- All optional arguments must come *after* required arguments.
- Unnamed positional arguments *must* be in the correct position.
- All arguments *can* be required, but now you get to name them.

### Optbuilder
---
Generate a "builder" method:
```rust
#[derive(optbuilder)]
struct Params {
    a: u32,
    b: Option<String>
}

// Typed-buidler style
let params = Params::builder().a(10).b("wasd").build();
```

or generate a macro_rules to construct this struct in style:
```rust
#[derive(optbuilder(gen_macro))]
struct Params {
    a: u32,
    b: Option<String>
}

// Typed-buidler style
let params = Params!{ a: 10 };
```

## Combine both for very ergonomic interfaces
```rust
#[derive(optbuilder(gen_macro))]
struct Scatter {
    x: Vec<i32>,
    y: Vec<i32>
    z: Vec<i32>
}

#[derive(optbuilder(gen_macro))]
struct Labels<'a> {
    title: Option<&'a str>
    xlabel: Option<&'a str>
    ylabel: Option<&'a str>
    zlabel: Option<&'a str>
}

fn plot(plot: Scatter, labels: Labels) {}

// 
plot(
    Scatter!{ x, y },
    Labels!{ title: "my plot", xlabel: "time"}
)
```

## How it works:
The `#[optargs]` macro generates an intermediate builder struct with `#[derive(optbuilder)]`. It also generates a macro_rules that calls this builder from a given input.

The `optbuilder` derive macro uses const generics to create a compile-time safe builder for a given struct.

This guarantees that our builder definition is compile-time correct and completely optimizes away, leaving you with zero-cost builders as good as your own implementation.

---
**In depth:**

Take this input function:
```rust
fn do_something(a: u32, b: Option<String>) -> bool {
    todo!()
}
```

We would then generate a macro_rules implementation:
```rust
// Generated code
macro_rules! do_something {
    () => {
        // todo    
    };
}

// here's how it would work for structs
struct do_something {}
impl do_something {
    fn call(f: impl FnOnce(SomethingBuilder<false, false>) -> SomethingBuilder<true, true>) -> () {
        let args = f(SomethingBuilder::<false, false>::default());
        do_something(args.a.unwrap(), args.b)
    }
}
```

And a builder just for its arguments:
```rust
#[derive(Default)]
struct SomethingBuilder<const A: bool> {
    a: Option<u32>,
    b: Option<String>,
}

// allow this method on builders without the "a" field set
impl SomethingBuilder<false> {
    fn a(self, a: u32) -> SomethingBuilder<true> {
        SomethingBuilder {
            a: Some(a),
            b: self.b
        }
    }
}

// allow this method on all builders regardless of state (because it's optional)
impl<const A: bool> SomethingBuilder<A> {
    fn b(self, b: String) -> SomethingBuilder<A> {
        SomethingBuilder {
            a: self.a,
            b: Some(b),
        }
    }
}
```

