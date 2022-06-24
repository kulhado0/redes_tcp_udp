pub trait Route {
    fn routes() -> &[impl Fn];
}
