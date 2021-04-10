#![deny(deprecated)]
use optargs_macro::masker;

macro_rules! example {
    ( $($key:ident: $value:expr), *) => {{
        let mut a = None;
        let mut b = None;
        $(
            let $key = Some($value);
        )*
        let (a_, b_) = masker!{{}};
        example(a_, b_)
    }};
}

fn example(a: i32, title: Option<&str>) -> bool {
    false
}

fn main() {
    let g = example(10, None);
    // let p = {
    //     let mut a = None;
    //     let mut b = None;
    //     a = Some(10);
    //     example(a.unwrap(), b)
    // };
    let f = example!(a: 10, title: "asd");
    // example!(a: 10);
}

mod old {
    macro_rules! t {
        ($($key:ident $(: $value:expr)? ), *) => {
            ExampleBuilder::default()
            $(.$key( some_helper!($key $(, $value)?)  ))*
            .build()
        };
    }

    macro_rules! some_helper {
        ($key:ident, $value:expr) => {
            $value
        };
        ($key:ident) => {
            $key
        };
    }
    macro_rules! specialfield {
        ($key:expr, $value:expr) => {
            $value
        };
        ($key:expr) => {
            $key
        };
    }

    // struct Dead {}
    // impl Dead {
    //     #[deprecated]
    //     fn unwrap<T>(self) -> T {
    //         todo!()
    //     }
    // }
}
