use crate::colors::Hsl;
#[derive(Default)]
pub struct Individual {
    stats: Hsl,
    score: u8,
}

impl Individual {
    pub fn new(hue: u8, saturation: u8, lightness: u8) -> Self {
        Individual {
            stats: Hsl::new(hue, saturation, lightness),
            score: u8::default(),
        }
    }

    fn compare(&self, other: &Self) -> u8 {
        todo!()
    }
}

impl std::fmt::Display for Individual {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} | {:2}]", self.stats, self.score)
    }
}
