use optargs_macro;

///
/// ```
/// #[optargs::optfn]
/// pub fn plot(
///     x: Vec<i32>,
///     y: Option<Vec<i32>>,
///     title: Option<&str>,
///     xlabel: Option<&str>,
///     ylabel: Option<&str>,
///     legend: Option<bool>
/// ) {}
///
/// // can now call it with optional arguments
/// plot!(
///     x: vec![1,2,3],
///     y: vec![1,2,3],
///     title: "Awesome plot",
///     xlabel: "x axis",
///     ylabel: "y axis"
/// );
/// ```
pub use optargs_macro::optfn;
pub use optargs_macro::OptStruct;
