pub mod adsr;
pub mod ar;
pub mod fp_ar;
pub mod multi_stage;

pub use adsr::*;
pub use ar::AttackRelease;
pub use fp_ar::FixedPointAttackRelease;
pub use multi_stage::MultiStageEnvelope;

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(C)]
pub enum ARPhase {
    HOLD = -1,
    ATTACK = 0,
    RELEASE = 1,
}
