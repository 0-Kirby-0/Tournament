mod individual;
mod state;
use state::{Parameters, State};
struct Simulation<'a> {
    steps: Vec<State<'a>>,
    parameters: Parameters,
}

impl<'a> Simulation<'a> {
    pub fn new() -> Self {
        Simulation {
            steps: vec![],
            parameters: Parameters::default(),
        }
    }
    pub fn populate_first(&'a mut self) {
        self.steps.push(State::new(&self.parameters));
    }
}