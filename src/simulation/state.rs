use super::parameters::{ParameterKind, Parameters};
use crate::colors::Rgb;
use field::{
    helpers::{Coordinate, Offset},
    Field,
};
use rayon::iter::*;

#[derive(Clone)]
pub struct State<'a> {
    field: field::Field<super::individual::Individual>,
    parameters: &'a Parameters,
}

impl<'a> State<'a> {
    pub fn new(parameters: &'a Parameters) -> Self {
        State {
            field: field::Field::new_default(
                parameters.get(ParameterKind::FieldWidth).unwrap(),
                parameters.get(ParameterKind::FieldHeight).unwrap(),
            ),
            parameters,
        }
    }

    pub fn as_byte_stream(&'a self) -> impl Iterator<Item = u8> + use<'a> {
        self.field.value_iterator().flat_map(|individual| {
            std::convert::Into::<Rgb>::into(individual.get_stats()).to_bytes()
        })
    }
}

impl<'a> Iterator for State<'a> {
    type Item = State<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        let offsets = Offset::square_kernel(1, false);
        let field = Field::new_from_grid(
            self.field
                .get_grid()
                .par_iter()
                .enumerate()
                .map(|(row_idx, line)| {
                    line.iter()
                        .enumerate()
                        .map(|(column_idx, protagonist)| {
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
        );
        Some(Self {
            field,
            parameters: self.parameters,
        })
    }
}
