use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount, EnumIter};

pub struct Parameters {
    params: [ParameterValue; ParameterKind::COUNT],
}

impl Parameters {
    pub fn get(&self, kind: ParameterKind) -> ParameterValue {
        *self
            .params
            .get(kind as usize)
            .expect("Parameters struct guarantees that all Kinds have associated values.")
    }
    pub fn set(&mut self, kind: ParameterKind, value: ParameterValue) {
        *self
            .params
            .get_mut(kind as usize)
            .expect("Parameters struct guarantees that all Kinds have associated values.") = value
    }
}

impl Default for Parameters {
    fn default() -> Self {
        let mut params = [ParameterValue::default(); ParameterKind::COUNT];
        for (param, kind) in params.iter_mut().zip(ParameterKind::iter()) {
            *param = kind.default();
        }
        Parameters { params }
    }
}

#[derive(EnumIter, EnumCount)]
pub enum ParameterKind {
    FieldWidth,
    FieldHeight,
    WinReward,
    LossReward,
    DrawReward,
    CooperationReward,
    HardeningRate,
    SofteningRate,
}

impl ParameterKind {
    fn default(&self) -> ParameterValue {
        use ParameterValue as P;
        match self {
            Self::FieldHeight => P::Word(1000),
            Self::FieldWidth => P::Word(1000),
            Self::WinReward => P::Byte(3),
            Self::LossReward => P::Byte(0),
            Self::DrawReward => P::Byte(0),
            Self::CooperationReward => P::Byte(3),
            Self::HardeningRate => P::Byte(1),
            Self::SofteningRate => P::Byte(1),
        }
    }
    fn name(&self) -> &'static str {
        match self {
            Self::FieldHeight => "Field Height",
            Self::FieldWidth => "Field Width",
            Self::WinReward => "Win Reward",
            Self::LossReward => "Loss Reward",
            Self::DrawReward => "Draw Reward",
            Self::CooperationReward => "Cooperation Reward",
            Self::HardeningRate => "Hardening Rate",
            Self::SofteningRate => "Softening Rate",
        }
    }
    fn description(&self) -> &'static str {
        match self {
            Self::FieldHeight => "Total height of the field/image, in pixels.",
            Self::FieldWidth => "Total width of the field/image, in pixels.",
            Self::WinReward => "Maximum score reward for winning, ie being 120° ahead of the opponent.",
            Self::LossReward => "Maximum score reward for losing, ie being 120° behind the opponent.",
            Self::DrawReward => "Maximum score reward for drawing, ie being 180° away from the opponent.",
            Self::CooperationReward => "Maximum score reward for cooperating, ie being 0° away from the opponent.",
            Self::HardeningRate => "The rate at which an individual will 'harden' when experiencing a lot of competition.",
            Self::SofteningRate => "The rate at which an individual will 'harden' when experiencing little competition.",
        }
    }
}

#[derive(Clone, Copy)]
pub(super) enum ParameterValue {
    Byte(u8),
    Word(usize),
    Float(f32),
}

impl Default for ParameterValue {
    fn default() -> Self {
        ParameterValue::Byte(0)
    }
}

impl ParameterValue {
    pub fn unwrap<T>(&self) -> T
    where
        T: Unwrappable,
    {
        let out: T = T::unwrap_enum(*self);
        out
    }
}

pub(super) trait Unwrappable {
    fn unwrap_enum(pv: ParameterValue) -> Self;
}
impl Unwrappable for u8 {
    fn unwrap_enum(pv: ParameterValue) -> Self {
        match pv {
            ParameterValue::Byte(val) => val,
            _ => panic!("Tried to unwrap parameter value in wrong type."),
        }
    }
}
impl Unwrappable for usize {
    fn unwrap_enum(pv: ParameterValue) -> Self {
        match pv {
            ParameterValue::Word(val) => val,
            _ => panic!("Tried to unwrap parameter value in wrong type."),
        }
    }
}
impl Unwrappable for f32 {
    fn unwrap_enum(pv: ParameterValue) -> Self {
        match pv {
            ParameterValue::Float(val) => val,
            _ => panic!("Tried to unwrap parameter value in wrong type."),
        }
    }
}
