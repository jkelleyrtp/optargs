// #[optargs::optfn]
pub fn plot(
    x: Vec<i32>,
    y: Option<Vec<i32>>,
    title: Option<&str>,
    xlabel: Option<&str>,
    ylabel: Option<&str>,
) -> bool {
    false
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
