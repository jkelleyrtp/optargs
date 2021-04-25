#[optargs::optfn]
fn blah2(a: Option<&dyn Iterator<Item = i32>>) {}

fn main() {
    blah2! {
        a: &(0..10)
    }
}
