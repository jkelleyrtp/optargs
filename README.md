# OptArgs: Optional arguments for Rust functions

This crate adds two macros to make it easy to add optional arguments to functions.
- `#[optargs]` - derive a `macro_rules` to call a function with optional arguments.
- `#[derive(optbuilder)]` - derive a typed-builder builder for a struct with optional fields.

This crate takes advantage of the recent const_generics in Rust stable (1.51), so our MSRV is 1.51.

Here's how they work:

### Optargs
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
```rust
#[derive(optbuilder)]
struct Params {
    a: u32,
    b: Option<String>
}

// Typed-buidler style
let params = Params::builder().a(10).b("wasd").build();
```

## How it works:
The `#[optargs]` macro generates an intermediate builder struct with `#[derive(optbuilder)]`. It also generates a macro_rules that calls this builder from a given input.

The `optbuilder` derive macro uses const generics to create a compile-time safe builder for a given struct.


Take this input function:
```rust
fn do_something(a: u32, b: Option<String>) -> bool {
    todo!()
}
```

We would then generate a macro_rules implementation:
```rust
// Generated code
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

