pub mod swing;

use core::num::NonZeroU8;
use core::time::Duration;

const MAX_SEQ_LEN: usize = 32;

pub enum BoundError {
    OutOfBound,
    BoundTooSmall,
}

pub struct BoundedWrappingU8 {
    index: u8,
    exclusive_upper_bound: NonZeroU8,
}

impl BoundedWrappingU8 {
    pub fn new(exclusive_upper_bound: NonZeroU8) -> BoundedWrappingU8 {
        BoundedWrappingU8 {
            index: 0,
            exclusive_upper_bound,
        }
    }

    pub fn set_current_position(&mut self, index: u8) -> Result<(), BoundError> {
        if index >= u8::from(self.exclusive_upper_bound) {
            return Err(BoundError::OutOfBound);
        }

        self.index = index;

        Ok(())
    }

    pub fn set_exclusive_upper_bound(&mut self, bound: NonZeroU8) -> Result<(), BoundError> {
        if u8::from(bound) <= self.index {
            return Err(BoundError::BoundTooSmall);
        }

        self.exclusive_upper_bound = bound;

        Ok(())
    }

    pub fn next_pos(&mut self) {
        if self.index.saturating_add(1) >= u8::from(self.exclusive_upper_bound) {
            self.index = 0;
        }

        self.index += 1;
    }

    pub fn last_pos(&mut self) {
        if self.index.checked_sub(1) == None {
            // reached lower bound -> wrap to upper bound
            // self.index = self.exclusive_upper_bound - 1;
        }

        self.index -= 1;
    }
}

enum NoteLength {
    Whole,
    Half,
    Quarter,
    Eigth,
    Sixtheenth,
}

struct Sequence {
    // scope
    total_length: NonZeroU8,
    root_length: NoteLength,

    //
    start_offset: BoundedWrappingU8,
    relative_position: u8,
}

pub fn expand_micro_microrhythm(
    relative_note_lenghts: &[NonZeroU8],
    new_duration: &mut [Duration],
    grid: Duration,
) {
    for (note_length, duration) in core::iter::zip(relative_note_lenghts, new_duration) {
        *duration = grid * u8::from(*note_length) as u32;
    }
}

// pub fn mirco_swing(relative_note_lenghts: &[NonZeroU8], existing_note_sequence:) {

// }
