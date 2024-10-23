use crate::colors::Hsl;
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

    pub fn bout(&mut self, antagonist: &Individual) {
        todo!()
    }

    pub fn bout_series<'a, I>(&self, antagonists: I) -> Self
    where
        I: Iterator<Item = &'a Individual>,
    {
        let mut out = *self;

        antagonists.for_each(|antagonist| {
            out.bout(antagonist);
        });

        out
    }
}

impl std::fmt::Display for Individual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} | {:2}]", self.stats, self.score)
    }
}
