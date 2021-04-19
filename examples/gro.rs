use std::marker::PhantomData;

use optargs_macro::optfn;

// here's an idea:
// order by alphabetical
// statically generate the shifted version
// then unshift it
// essentially, save all the keys and then generate them into the macro_rules
// bypass the restriction of using a struct in the first place
// macro that generates a macro
#[optfn]
fn example(a: u32, b: i32, c: Option<String>) -> bool {
    false
}

fn main() {
    let g = example!(a: 10, b: 10);
    let g = example!(a: 10, b: 10, c: "alaskjdl;askd;laskd;laskdl;askd;laskd;lakd;lakd;laskd;laksd;lkas;dlkas;ldka;lsdkaksd;laksd;laksd;lkasl;sd".into());

    let g = example! {
        a: 10,
        b: 10,
        c: "alaskjdl;askd;laskd;laskdl;askd;laskd;lakd;lakd;laskd;laksd;lkas;dlkas;ldka;lsdkaksd;laksd;laksd;lkasl;sd".into()
    };
}

mod v2 {

    use optargs_macro::optfn;

    // #[optfn2]
    // pub fn plot(
    //     x: Vec<i32>,
    //     y: Option<Vec<i32>>,
    //     title: Option<&str>,
    //     xlabel: Option<&str>,
    //     ylabel: Option<&str>,
    // ) -> bool {
    //     false
    // }

    // fn test() {
    //     let x = vec![1, 2, 3];
    //     let y = vec![1, 2, 3];
    //     plot! {x: x, y: y, title: "asdsad", xlabel: "adasdsas", ylabel: "adasdsasdadlkjasdlkjalskddlkajsdklajsdklalksjdklasjdalksdjaslkdjasldkjas"};

    //     plot!(x: x, y: y, title: "asdsad");
    // }
}
