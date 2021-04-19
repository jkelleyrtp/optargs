#[derive(optargs::OptStruct)]
struct Example {
    a: i32,
    b: Option<String>,
}

fn main() {
    let ex = Example! {
        a: 10,
        b: "asd".into()
    };
}
