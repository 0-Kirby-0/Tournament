use field::{
    helpers::{Coordinate, Offset},
    Field,
};
use rayon::iter::*;

#[derive(Clone)]
pub(super) struct State<'a> {
    field: field::Field<super::individual::Individual>,
    parameters: &'a Parameters,
}

impl<'a> State<'a> {
    pub fn new(parameters: &'a Parameters) -> Self {
        State {
            field: field::Field::new_default(parameters.field_width, parameters.field_height),
            parameters,
        }
    }
}

impl<'a> Iterator for State<'a> {
    type Item = State<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        Some(Self {
            field: Field::new_from_grid(
                self.field
                    .get_grid()
                    .par_iter()
                    .enumerate()
                    .map(|(row_idx, line)| {
                        line.iter()
                            .enumerate()
                            .map(|(column_idx, protagonist)| {
                                let offsets = Offset::square_kernel(1, false);
                                let antagonists = self.field.kernel_iter(
                                    Coordinate {
                                        row: row_idx,
                                        column: column_idx,
                                    },
                                    &offsets,
                                );
                                protagonist.bout_series(antagonists, self.parameters)
                            })
                            .collect()
                    })
                    .collect(),
            ),
            parameters: self.parameters,
        })
    }
}

pub(super) struct Parameters {
    field_width: usize,
    field_height: usize,

    win_reward: u8,
    loss_reward: u8,
    draw_reward: u8,
    cooperation_reward: u8,
}

impl Default for Parameters {
    fn default() -> Self {
        Parameters {
            field_width: 1000,
            field_height: 1000,
            win_reward: 3,
            loss_reward: 0,
            draw_reward: 0,
            cooperation_reward: 3,
        }
    }
}
