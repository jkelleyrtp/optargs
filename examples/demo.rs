#[optargs::optfn]
fn go_gme(
    to_the_moon: Option<bool>,
    rocket_ships: Option<usize>,
    doges: Option<usize>,
    tendies: Option<bool>,
) {
    println!(
        "
GME:              
----              
Destination:  - {}
Velocity:     - {}
Passengers:   - {}
Menu:         - {}
",
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
    // accepts this
    go_gme!(
        to_the_moon: true,
        rocket_ships: 10,
        doges: 7,
        tendies: true
    );

    // but not this :(
    let doges = 8;
    go_gme!(doges, tendies: true);
}
