use optargs_macro;
pub use optargs_macro::masker;
pub use optargs_macro::optfn;
pub use optargs_macro::optfn2;

// a little helper macro for the
#[macro_export]
macro_rules! builder_field {
    ($key:expr, $value:expr) => {
        $value
    };
    ($key:expr) => {
        $key
    };
}

// a proc macro that generates a macro-rules with no limitations on arguments
// and a derive macro that generates a builder with const generics
// and a proc macro that generates a macro-rules for a builder
