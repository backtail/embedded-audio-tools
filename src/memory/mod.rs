pub mod memory_slice;

use crate::memory::memory_slice::{MutLocation, NonMutLocation};

/// Describes all possible errors that can occur when handling buffer manipulation
#[derive(Debug, PartialEq)]
pub enum MemSliceError {
    IndexOutOfBound,
    LengthOutOfBound,
}

/// Raw pointer that implements the `Send` trait since it's only acting on stack memory
///
/// Should always point at the beginning of your audio buffer in use
#[derive(Clone, Copy)]
pub struct NonMutable(*const f32);
unsafe impl Send for NonMutable {}

/// Raw mutable pointer that implements the `Send` trait since it's only acting on stack memory
///
/// Should always point at the beginning of your audio buffer in use
#[derive(Clone, Copy)]
pub struct Mutable(*mut f32);
unsafe impl Send for Mutable {}

///////////////////////////////////////////////////////////////////////////////
/// Memory Pointer Trait Implemenations
///////////////////////////////////////////////////////////////////////////////

impl NonMutLocation for NonMutable {
    type Output = NonMutable;
    fn get(&self) -> *const f32 {
        self.0
    }
    fn new(ptr: *const f32) -> Self::Output {
        NonMutable(ptr)
    }
}

impl NonMutLocation for Mutable {
    type Output = Mutable;
    fn get(&self) -> *const f32 {
        *&self.0
    }

    fn new(ptr: *const f32) -> Self::Output {
        Mutable(ptr.cast_mut())
    }
}

impl MutLocation for Mutable {
    type Output = Mutable;

    fn get_mut(&mut self) -> *mut f32 {
        self.0
    }

    fn new_mut(ptr: *mut f32) -> Self::Output {
        Mutable(ptr)
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Common Trait Implementations
///////////////////////////////////////////////////////////////////////////////

impl Default for NonMutable {
    fn default() -> Self {
        NonMutable(core::ptr::null())
    }
}

impl Default for Mutable {
    fn default() -> Self {
        Mutable(core::ptr::null_mut())
    }
}
