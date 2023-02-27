pub mod mem_slice;
pub mod mut_mem_slice;

/// Describes all possible errors that can occur when handling static buffer manipulation
#[derive(Debug, PartialEq)]
pub enum MemSliceError {
    IndexOutOfBound,
    LengthOutOfBound,
}

/// Raw pointer that implements the `Send` trait since it's only acting on static memory
///
/// Should always point at the beginning of your audio buffer in use
#[derive(Clone, Copy)]
pub struct MemoryPtr(pub *const f32);
unsafe impl Send for MemoryPtr {}

/// Raw mutable pointer that implements the `Send` trait since it's only acting on static memory
///
/// Should always point at the beginning of your audio buffer in use
#[derive(Clone, Copy)]
pub struct MutMemoryPtr(pub *mut f32);
unsafe impl Send for MutMemoryPtr {}
