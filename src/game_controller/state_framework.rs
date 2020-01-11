pub trait GameManager<S> {
    fn process_state(state: S) -> S;
}