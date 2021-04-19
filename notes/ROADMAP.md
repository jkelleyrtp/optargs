# Future
- positional parameters (potential conflict with naming)
- argument forwarding (*args, **kwargs)
- allow arbitrary ordering of parameters (potential conflict with named + positional params )

# v1 
- [x] optional functions with named parameters with `fn!()` syntax
- [x] optional structs with `Struct!{}` syntax
- [x] use of const generics over the typed-builder crate
- [ ] builder structs with `Struct::builder().build()` syntax
  
n.b.: Place some restrictions on ordering and naming requirements to potentially lift them in the future in a non-breaking fashion.

For structs:
```rust
// enables the builder syntadx
#[derive(optbuilder)]
struct Example {}

// enables both the builder syntax and macro caller syntax
#[derive(optbuilder, optmacro)]
struct Example {}
```

For functions:
```rust
#[optfn]
fn example() { }
```
