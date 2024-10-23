use field::{
    helpers::{Coordinate, Offset},
    Field,
};

use rayon::iter::*;

use crate::individual::Individual;
#[derive(Clone)]
struct State {
    field: field::Field<Individual>,
}

impl State {}

impl Iterator for State {
    type Item = State;
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
                                protagonist.bout_series(antagonists)
                            })
                            .collect()
                    })
                    .collect(),
            ),
        })
    }
}
