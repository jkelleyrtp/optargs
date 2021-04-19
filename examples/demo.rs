#[optargs::optfn]
fn go_gme(
    price: f32,
    to_the_moon: Option<bool>,
    rocket_ships: Option<usize>,
    doges: Option<usize>,
    tendies: Option<bool>,
) {
    println!(
        "
GME:  {}            
----              
Destination:  - {}
Velocity:     - {}
Passengers:   - {}
Menu:         - {}
",
        price,
        to_the_moon.map(|_| "🌓").unwrap_or(""),
        rocket_ships
            .map(|f| (0..f).map(|_| "🚀").collect())
            .unwrap_or("".to_string()),
        doges
            .map(|f| (0..f).map(|_| "🐶").collect())
            .unwrap_or("".to_string()),
        tendies.map(|_| "🍗").unwrap_or("")
    );
}

fn main() {
    go_gme!(
        price: 10.0,
        to_the_moon: true,
        rocket_ships: 10,
        doges: 7,
        tendies: true
    );

    // pass it from the environment
    let doges = 8;
    go_gme!(price: 10.0, doges, tendies: true);

    // order doesn't matter since they're named!
    go_gme!(price: 10.0, tendies: true, doges);

    // this works
    let price = 10.0;
    go_gme!(price);

    // but this doesn't since we need name for the builder
    // This might eventually work, but it's currently a limited until positional args are implemented completely
    // go_gme!(10.0);
}
