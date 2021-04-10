use optargs::optfn2;

#[optfn2]
fn example(a: i32, b: Option<&str>) {}

fn main() {
    example!(a: 10, b: "asd");
}
