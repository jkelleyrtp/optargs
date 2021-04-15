#[optargs::optfn]
pub fn plot(
    x: Vec<i32>,
    y: Option<Vec<i32>>,
    title: Option<&'static str>,
    xlabel: Option<&'static str>,
    ylabel: Option<&'static str>,
    // kwargs: ...
) -> bool {
    // call_blah!(..kwargs);

    false
}

mod mod1 {

    fn test() {
        blah!();
    }

    use crate::__blah_optional as blah;
    #[macro_export]
    macro_rules! __blah_optional {
        () => {};
    }
}

fn main() {
    let x = vec![1, 2, 3];
    let y = vec![1, 2, 3];

    // let p = plot!(
    //     x,
    //     y,
    //     title: "asdsad",
    //     xlabel: "adasdsas",
    //     ylabel: "adasdsas"
    // );
}
