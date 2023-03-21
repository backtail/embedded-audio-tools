use core::ops::Neg;

use super::{Mutable, NonMutable};
use crate::memory::MemSliceError::{self, *};

use crate::float::interpolation::{lagrange, lagrange_only_4_elements, lerp_unchecked};

#[allow(unused_imports)]
use micromath::F32Ext;

///////////////////////////////////////////////////////////////////////////////
/// Struct
///////////////////////////////////////////////////////////////////////////////

/**
Raw slice pointer that implements the `Send` trait since it's **only**
works **safely** on **static memory**.

Use default implemention to get the null pointer with length of zero and the
respective mutabilty via the generic argument.

## Example

```
# use embedded_audio_tools::memory_access::*;
// Thread-safe non-mutable slice
let buffer = [0.0_f32; 24];
let non_mut_slice = from_slice(&buffer[..]);

// Thread-safe mutable slice
let mut buffer = [0.0_f32; 24];
let mut mut_slice = from_slice_mut(&mut buffer[..]);

// Null pointer and length of 0
let mut ptr_buffer = MemorySlice::<Mutable>::default(); // or null();

// Change associated buffer in runtime
unsafe {
    ptr_buffer.change_mut_slice_unchecked(buffer.as_mut_ptr(), buffer.len());
}

assert_eq!(ptr_buffer.as_slice(), mut_slice.as_slice());
```

*/
#[derive(Clone, Copy)]
pub struct MemorySlice<Mutability> {
    ptr: Mutability,
    length: usize,
}

///////////////////////////////////////////////////////////////////////////////
/// Memory Pointer Traits
///////////////////////////////////////////////////////////////////////////////

pub trait NonMutLocation {
    type Output;
    fn get(&self) -> *const f32;
    fn new(ptr: *const f32) -> Self::Output;
}

pub trait MutLocation {
    type Output;
    fn get_mut(&mut self) -> *mut f32;
    fn new_mut(ptr: *mut f32) -> Self::Output;
}

///////////////////////////////////////////////////////////////////////////////
/// Common Trait Implementation
///////////////////////////////////////////////////////////////////////////////

/// This data structure is thread-safe!
unsafe impl<Mutability> Sync for MemorySlice<Mutability> {}

impl<Mutability: Default> Default for MemorySlice<Mutability> {
    fn default() -> Self {
        MemorySlice {
            ptr: Default::default(),
            length: 0,
        }
    }
}

pub fn null() -> MemorySlice<NonMutable> {
    MemorySlice::<NonMutable>::default()
}

pub fn null_mut() -> MemorySlice<Mutable> {
    MemorySlice::<Mutable>::default()
}

///////////////////////////////////////////////////////////////////////////////
/// Functions independent of Mutability
///////////////////////////////////////////////////////////////////////////////

impl<Mutability> MemorySlice<Mutability> {
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.length
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Non-Mutable Memory Slice Functions
///////////////////////////////////////////////////////////////////////////////

impl<Mutability> MemorySlice<Mutability>
where
    Mutability: NonMutLocation<Output = Mutability> + Default,
{
    ///////////////////////////////////////////////////////////////////////////////
    /// Indirect Data Access
    ///////////////////////////////////////////////////////////////////////////////

    pub fn get_sub_slice(
        &self,
        offset: usize,
        sub_length: usize,
    ) -> Result<MemorySlice<Mutability>, MemSliceError> {
        if offset >= self.length {
            return Err(IndexOutOfBound);
        }

        if offset + sub_length >= self.length {
            return Err(LengthOutOfBound);
        }

        Ok(MemorySlice {
            ptr: Mutability::new(unsafe { self.ptr.get().add(offset) }),
            length: sub_length,
        })
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Direct Data Access
    ///////////////////////////////////////////////////////////////////////////////

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, index: usize) -> f32 {
        self.ptr.get().add(index).read_volatile()
    }

    pub fn get(&self, index: usize) -> Result<f32, MemSliceError> {
        if index >= self.length {
            return Err(IndexOutOfBound);
        }

        unsafe { Ok(self.get_unchecked(index)) }
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Wrapping Data Access
    ///////////////////////////////////////////////////////////////////////////////

    #[inline(always)]
    pub unsafe fn get_wrapped_unchecked(&self, index: isize) -> f32 {
        self.get_unchecked(index.rem_euclid(self.length as isize) as usize)
    }

    #[inline(always)]
    pub fn get_wrapped(&self, index: isize) -> f32 {
        if index >= self.length as isize || index < 0 {
            return unsafe { self.get_wrapped_unchecked(index) };
        }

        unsafe { self.get_unchecked(index as usize) }
    }

    pub fn get_slice_of_four_wrapped(&self, index: isize) -> [f32; 4] {
        if index + 3 >= self.length as isize || index < 0 {
            unsafe {
                [
                    self.get_wrapped_unchecked(index + 0),
                    self.get_wrapped_unchecked(index + 1),
                    self.get_wrapped_unchecked(index + 2),
                    self.get_wrapped_unchecked(index + 3),
                ]
            }
        } else {
            unsafe {
                [
                    self.get_unchecked((index + 0) as usize),
                    self.get_unchecked((index + 1) as usize),
                    self.get_unchecked((index + 2) as usize),
                    self.get_unchecked((index + 3) as usize),
                ]
            }
        }
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Linear Interpolation Data Access
    ///////////////////////////////////////////////////////////////////////////////

    #[inline(always)]
    pub unsafe fn lerp_unchecked(&self, index: f32) -> f32 {
        let a = self.get_unchecked(index as usize);
        let b = self.get_unchecked(index as usize + 1);

        lerp_unchecked(a, b, index - (index as usize) as f32)
    }

    pub fn lerp(&self, index: f32) -> Result<f32, MemSliceError> {
        if index < 0.0 {
            return Err(IndexOutOfBound);
        }

        if index == (self.length - 1) as f32
            && index < self.length as f32 - 1.0 + 10.0 * f32::EPSILON
        {
            return Ok(unsafe { self.get_unchecked(index as usize) });
        }

        let a = self.get(index as usize)?;
        let b = self.get(index as usize + 1)?;

        Ok(lerp_unchecked(a, b, index - (index as usize) as f32))
    }

    #[inline(always)]
    pub fn lerp_wrapped(&self, index: f32) -> f32 {
        let int_index = index.floor() as isize;
        let a = self.get_wrapped(int_index);
        let b = self.get_wrapped(int_index + 1);

        lerp_unchecked(a, b, index - (int_index as f32))
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Lagrange Interpolation Data Access
    ///////////////////////////////////////////////////////////////////////////////

    pub fn lagrange_wrapped(&self, index: f32, mut window_size: usize) -> f32 {
        let int_index = index.floor() as isize;
        let mut slice: [f32; 100] = [0.0_f32; 100];

        if window_size > 100 {
            window_size = 100;
        }

        let lower_bound = ((window_size / 2) as isize).neg() + 1;
        let upper_bound = (window_size / 2) as isize;

        for i in lower_bound..=upper_bound {
            unsafe {
                let wrapped_index = self.get_wrapped_unchecked(int_index + i);
                slice[(i + lower_bound.neg()) as usize] = wrapped_index;
            }
        }

        lagrange(&slice[..window_size], index - int_index as f32)
    }

    pub fn lagrange_four_points_wrapped(&self, index: f32) -> f32 {
        unsafe {
            lagrange_only_4_elements(
                &self.get_slice_of_four_wrapped(index.floor() as isize)[..],
                index,
            )
        }
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Data Overwriting
    ///////////////////////////////////////////////////////////////////////////////

    /// Overwrites the current `MemorySlice`. Useful, if you want to assign a buffer in runtime.
    ///
    /// ## Safety
    /// If you want to use this to be thread-safe, only use this on static memory.
    #[inline(always)]
    pub unsafe fn change_slice_unchecked(&mut self, ptr: *const f32, length: usize) {
        self.ptr = Mutability::new(ptr);
        self.length = length;
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Type Conversion
    ///////////////////////////////////////////////////////////////////////////////

    #[inline(always)]
    pub fn as_slice(&self) -> *const [f32] {
        core::ptr::slice_from_raw_parts(self.ptr.get(), self.length)
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Mutable Memory Slice Functions
///////////////////////////////////////////////////////////////////////////////

impl<Mutability> MemorySlice<Mutability>
where
    Mutability: MutLocation<Output = Mutability> + Default,
{
    ///////////////////////////////////////////////////////////////////////////////
    /// Direct Data Assignment
    ///////////////////////////////////////////////////////////////////////////////

    #[inline(always)]
    pub unsafe fn assign_unchecked(&mut self, index: usize, value: f32) {
        self.ptr.get_mut().add(index).write_volatile(value);
    }

    pub fn assign(&mut self, index: usize, value: f32) -> Result<(), MemSliceError> {
        if index >= self.length {
            return Err(IndexOutOfBound);
        }

        unsafe {
            self.assign_unchecked(index, value);
        }

        Ok(())
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Data Overwriting
    ///////////////////////////////////////////////////////////////////////////////

    /// Overwrites the current `MemorySlice`. Useful, if you want to assign a buffer in runtime.
    ///
    /// ## Safety
    /// If you want to use this to be thread-safe, only use this on static memory.
    #[inline(always)]
    pub unsafe fn change_mut_slice_unchecked(&mut self, ptr: *mut f32, length: usize) {
        self.ptr = Mutability::new_mut(ptr);
        self.length = length;
    }

    ///////////////////////////////////////////////////////////////////////////////
    /// Type Conversion
    ///////////////////////////////////////////////////////////////////////////////

    #[inline(always)]
    pub fn as_slice_mut(&mut self) -> *mut [f32] {
        core::ptr::slice_from_raw_parts_mut(self.ptr.get_mut(), self.length)
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Common Type Conversions
///////////////////////////////////////////////////////////////////////////////
#[inline(always)]
pub fn from_slice_mut(slice: &mut [f32]) -> MemorySlice<Mutable> {
    MemorySlice {
        ptr: Mutable::new_mut(slice.as_mut_ptr()),
        length: slice.len(),
    }
}

#[inline(always)]
pub fn from_slice(slice: &[f32]) -> MemorySlice<NonMutable> {
    MemorySlice {
        ptr: NonMutable::new(slice.as_ptr()),
        length: slice.len(),
    }
}

///////////////////////////////////////////////////////////////////////////////
/// Unit Tests
///////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_value() {
        let mut buffer = [0.0_f32; 24];
        let mut ptr_buffer = from_slice_mut(&mut buffer[..]);

        let value = 42.0;
        let index = 10;

        ptr_buffer.assign(index, value).unwrap();
        ptr_buffer.assign(index + 1, value).unwrap();
        ptr_buffer.assign(index + 2, value).unwrap();

        assert_eq!(value, buffer[index]);
        assert_eq!(value, buffer[index + 1]);
        assert_eq!(value, buffer[index + 2]);

        assert_eq!(
            ptr_buffer.assign(ptr_buffer.length + 1, value),
            Err(IndexOutOfBound)
        );
    }

    #[test]
    fn get_value() {
        let mut buffer = [0.0_f32; 24];
        for (i, val) in buffer.iter_mut().enumerate() {
            *val = i as f32;
        }

        let ptr_buffer = from_slice(&mut buffer[..]);

        assert_eq!(ptr_buffer.get(0), Ok(buffer[0]));
        assert_eq!(ptr_buffer.get(5), Ok(buffer[5]));
        assert_eq!(ptr_buffer.get(ptr_buffer.length + 1), Err(IndexOutOfBound));
    }

    #[test]
    fn get_value_wrapped() {
        const SIZE: usize = 24;
        let mut buffer = [0.0_f32; SIZE];
        for (i, val) in buffer.iter_mut().enumerate() {
            *val = i as f32;
        }

        let ptr_buffer = from_slice(&mut buffer[..]);

        for i in 0..6 * SIZE {
            let index = i as isize - (3 * SIZE) as isize;
            let _ = ptr_buffer.get_wrapped(index);

            assert_eq!(
                ptr_buffer.get_wrapped(index),
                (i % SIZE) as f32,
                "at index: {}",
                index
            );
        }
    }

    #[test]
    fn unchecked_lerp() {
        let mut buffer = [0.0_f32; 24];
        for (i, val) in buffer.iter_mut().enumerate() {
            *val = i as f32;
        }

        let ptr_buffer = from_slice(&mut buffer[..]);

        assert_eq!(unsafe { ptr_buffer.lerp_unchecked(5.5) }, 5.5);
    }

    #[test]
    fn checked_lerp() {
        const SIZE: usize = 24;
        let mut buffer = [0.0_f32; SIZE];
        for (i, val) in buffer.iter_mut().enumerate() {
            *val = i as f32;
        }

        let ptr_buffer = from_slice(&mut buffer[..]);

        assert_eq!(ptr_buffer.lerp(-f32::EPSILON), Err(IndexOutOfBound));
        assert_eq!(ptr_buffer.lerp(0.0), Ok(0.0));

        let close_under = (SIZE - 1) as f32 - 10.0 * f32::EPSILON;
        assert_ne!(close_under, (SIZE - 1) as f32);
        assert_eq!(ptr_buffer.lerp(close_under), Ok(close_under));
        assert_eq!(ptr_buffer.lerp((SIZE - 1) as f32), Ok((SIZE - 1) as f32));

        assert_eq!(
            ptr_buffer.lerp((SIZE - 1) as f32 + 9.0 * f32::EPSILON),
            Err(IndexOutOfBound)
        );
    }

    #[test]
    fn lerp_wrapped() {
        const SIZE: usize = 24;
        let mut buffer = [0.0_f32; SIZE];
        for (i, val) in buffer.iter_mut().enumerate() {
            *val = i as f32;
        }

        let ptr_buffer = from_slice(&mut buffer[..]);

        assert_eq!(ptr_buffer.lerp_wrapped(-1.0), (SIZE - 1) as f32);
        assert_eq!(ptr_buffer.lerp_wrapped(-0.5), ((SIZE - 1) as f32) / 2.0);
        assert_eq!(ptr_buffer.lerp_wrapped(0.0), 0.0);
        assert_eq!(ptr_buffer.lerp_wrapped(0.5), 0.5);
        assert_eq!(
            ptr_buffer.lerp_wrapped(SIZE as f32 - 0.5),
            ((SIZE - 1) as f32) / 2.0
        );
        assert_eq!(ptr_buffer.lerp_wrapped(SIZE as f32), 0.0);
        assert_eq!(ptr_buffer.lerp_wrapped(SIZE as f32 + 0.5), 0.5);
    }

    #[test]
    fn lagrange_wrapped() {
        let mut buffer = [0.0_f32, -1.0, 1.0, 0.4];
        let ptr_buffer = from_slice(&mut buffer[..]);
        for i in -10..10 {
            assert!(ptr_buffer.lagrange_wrapped(i as f32, 4).is_finite());
        }
    }
}
