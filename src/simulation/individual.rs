use crate::colors::Hsl;

use super::state::Parameters;
#[derive(Default, Clone, Copy)]
pub struct Individual {
    stats: Hsl,
    score: u8,
}

impl Individual {
    pub fn new(hue: u8, saturation: u8, lightness: u8) -> Self {
        Self {
            stats: Hsl::new(hue, saturation, lightness),
            score: u8::default(),
        }
    }

    pub fn bout(&mut self, antagonist: &Individual, parameters: &Parameters) {
        todo!()
    }

    pub fn bout_series<'a, I>(&self, antagonists: I, parameters: &Parameters) -> Self
    where
        I: Iterator<Item = &'a Individual>,
    {
        let mut out = *self;

        antagonists.for_each(|antagonist| {
            out.bout(antagonist, parameters);
        });

        out
    }
}

impl std::fmt::Display for Individual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} | {:2}]", self.stats, self.score)
    }
}
