//! poc for struct builder

struct Example {
    a: i32,
    b: Option<&'static str>,
}
impl Example {
    fn build(mut self) -> bool {
        false
    }
}

macro_rules! Example {
    ($($key:ident $(: $value:expr)? ), *) => {{
        let mut inners = (None, None);
        { $( Example!(@setter_helper inners $key $key $($value)? ) )* }
        Example {
            a: inners.0.unwrap(),
            b: inners.1
        }
    }};
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
    let p = Example! { a: 10 };
}
