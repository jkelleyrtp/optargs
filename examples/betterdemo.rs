#[optargs::optfn]
fn example(a: i32, b: Option<&str>) {}

macro_rules! example_example {
    ($($key:ident $(: $value:expr)? ), *) => {
        {
            #[allow(unused_mut)]
            let mut inners = (None, None);
            { $( example_example!(@setter_helper inners $key $key $($value)? ); )* }

            // Validate
            // struct Validator<const A: bool> {}
            // impl Validator<true> { fn validate(self) {} }
            // impl Validator<false> { fn a(self) -> Validator<true> { unsafe {std::mem::transmute(self)} } }
            // impl<const A: bool> Validator<A> { fn b(self) -> Validator<A> { self  }}

            // #[allow(unused_mut)]
            // let mut validator = Validator::<false> {};
            // validator $(.$key())* .validate();
            example(inners.0.unwrap(), inners.1)
        }
    };

    (@setter_helper $src:ident a $key:ident) => {
        $src.0 = Some($key);
    };

    (@setter_helper $src:ident a $key:ident $value:expr) => {
        $src.0 = Some($value);
    };

    (@setter_helper $src:ident b $key:ident) => {
        $src.1 = Some($key);
    };

    (@setter_helper $src:ident b $key:ident $value:expr) => {
        $src.1 = Some($value);
    };
}

fn main() {
    // let p = |_| {};
    example!(a: 10, b: "asd");

    // example_example!(a: 10);
    example_example!(b: "asd");

    // let a = 10;
    // example!(a);
}
