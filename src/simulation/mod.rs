mod individual;
mod parameters;
mod state;

#[ouroboros::self_referencing]
pub struct Simulation {
    parameters: parameters::Parameters,

    #[borrows(parameters)]
    #[not_covariant]
    steps: Vec<state::State<'this>>,
}

impl Simulation {
    pub fn new_pub() -> Self {
        Self::new(parameters::Parameters::default(), |params| {
            vec![state::State::new(params)]
        })
    }
    pub fn current(&self) -> &state::State {
        self.with_steps(|vec| vec.last().unwrap())
    }
}
