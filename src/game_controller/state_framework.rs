pub trait GameManager<S> {
    fn process_state(&mut self, state: S) -> S;
}