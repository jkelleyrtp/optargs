#[optargs::optfn2]
fn example(a: i32, b: Option<&str>) -> bool {
    false
}

fn main() {
    let r = example!(a: 10);
}
