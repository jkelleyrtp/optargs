use optargs::optfn2;

#[optfn2]
fn example(a: i32, b: Option<&str>) {}

macro_rules! bexample {
    ($($key:ident : $value:expr ), *) => {
        {
            #[allow(unused_mut)]
            let mut inners = (None, None);
            {
                $(
                    bexample!(@setter_helper inners, $key, $value)
                )*
            }
            // {$( my_helper!(inners, $key, $value) )*}

            struct Validator<const A: bool> {}
            impl Validator<true> { fn validate(self) {} }
            impl Validator<false> { fn a(self) -> Validator<true> { unsafe {std::mem::transmute(self)} } }

            #[allow(unused_mut)]
            let mut validator = Validator::<false> {};
            validator $(.$key())* .validate();
            example(inners.0.unwrap(), inners.1)
        }
    };

    (@key_helper $key:ident, $value:expr) => {
        $value
    };
    (@key_helper $key:ident) => {
        $key
    };

    (@setter_helper $src:ident, a, $value:expr) => {
        $src.0 = Some($value);
    };

    (@setter_helper $src:ident, b, $value:expr) => {
        $src.1 = Some($value);
    };
}

fn main() {
    // example!(a: 10, b: "asd");

    bexample!(a: 10);
    bexample!(a: 10);
    // bexample!();
    // bexample!(a: 10, b: "asd");
}
