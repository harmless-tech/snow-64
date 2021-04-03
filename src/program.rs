pub struct Program<'a> {
    start_fn: &'a dyn Fn() -> ()
}
