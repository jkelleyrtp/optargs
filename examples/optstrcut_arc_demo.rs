use std::{any::Any, sync::Arc};

#[derive(optargs::OptStructArc)]
struct Example {
    a: i32,
    b: Option<String>,
}

fn main() {
    let ex1: Arc<Example> = Example! {
        a: 10,
        b: "asd".into()
    };
    let ex2: Arc<dyn Any> = Example! {
        a: 10,
        b: "asd".into()
    };
}
