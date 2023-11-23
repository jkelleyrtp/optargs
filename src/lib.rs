use optargs_macro;

/// Optional arguments for functions!
/// Add optfn on top of any function and then you can call the funtion with optional arguments.
///
/// Note that this still obeys traditional macro_rules, so you can only use the macro *after* declaration or import it from "crate".
///
///
/// ```ignore
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

/// Flexible struct builder with optional arguments
/// Derive OptStruct for your structs and then call the Struct's name as a macro to build it, eliding optionals.
///
/// Note that this still obeys traditional macro_rules, so you can only use the macro *after* declaration or import it from "crate".
///
/// ```rust
/// #[derive(optargs::OptStruct)]
/// pub struct Scatter {
///     x: Vec<i32>,
///     y: Option<Vec<i32>>,
///     title: Option<&str>,
///     xlabel: Option<&str>,
///     ylabel: Option<&str>,
///     legend: Option<bool>
/// }
///
/// let plot = Scatter!{
///     x: vec![1,2,3],
///     legend: true
/// };
/// ```
pub use optargs_macro::OptStruct;

/// Flexible struct builder with optional arguments. Wrapped in an `std::sync::Arc`
/// Derive OptStruct for your structs and then call the Struct's name as a macro to build it with an `std::sync::Arc` wrapper applied, eliding optionals.
///
/// Note that this still obeys traditional macro_rules, so you can only use the macro *after* declaration or import it from "crate".
///
/// ```rust
/// #[derive(optargs::OptStructArc)]
/// pub struct Scatter {
///     x: Vec<i32>,
///     y: Option<Vec<i32>>,
///     title: Option<&str>,
///     xlabel: Option<&str>,
///     ylabel: Option<&str>,
///     legend: Option<bool>
/// }
///
/// let plot = Scatter!{
///     x: vec![1,2,3],
///     legend: true
/// };
/// `
pub use optargs_macro::OptStructArc;
