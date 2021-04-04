use std::marker::PhantomData;

use optargs_macro::optfn;

#[optfn]
fn example(a: Option<u32>, b: Option<String>) -> bool {
    false
}

fn main() {
    let g = example!(a: 10);
    let g = example!(a: 10, b: "asd".into());
}
