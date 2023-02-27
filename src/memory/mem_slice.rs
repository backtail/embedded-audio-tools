use crate::memory::{
    MemSliceError::{self, *},
    MemoryPtr,
};

/// Raw slice pointer that implements the `Send` trait since it's only acting on static memory
#[derive(Clone, Copy)]
pub struct MemSlice {
    pub ptr: MemoryPtr,
    pub length: usize,
}

unsafe impl Sync for MemSlice {}

impl MemSlice {
    #[inline(always)]
    pub fn null() -> MemSlice {
        MemSlice {
            ptr: MemoryPtr(core::ptr::null()),
            length: 0,
        }
    }

    pub fn get_sub_slice(
        &self,
        offset: usize,
        sub_length: usize,
    ) -> Result<MemSlice, MemSliceError> {
        if offset >= self.length {
            return Err(IndexOutOfBound);
        }

        if offset + sub_length >= self.length {
            return Err(LengthOutOfBound);
        }

        Ok(MemSlice {
            ptr: unsafe { MemoryPtr(self.ptr.0.add(offset)) },
            length: sub_length,
        })
    }

    #[inline(always)]
    pub unsafe fn get_unchecked(&self, index: usize) -> f32 {
        self.ptr.0.add(index).read()
    }

    pub fn get(&self, index: usize) -> Result<f32, MemSliceError> {
        if index >= self.length {
            return Err(IndexOutOfBound);
        }

        unsafe { Ok(self.get_unchecked(index)) }
    }

    /// Only use this on static memory!
    #[inline(always)]
    pub unsafe fn set_slice(&mut self, ptr: *const f32, length: usize) {
        self.ptr.0 = ptr;
        self.length = length;
    }

    #[inline(always)]
    pub fn as_slice(&mut self) -> *const [f32] {
        core::ptr::slice_from_raw_parts(self.ptr.0, self.length)
    }
}

#[inline(always)]
pub fn from_slice(slice: &[f32]) -> MemSlice {
    MemSlice {
        ptr: MemoryPtr(slice.as_ptr()),
        length: slice.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_value() {
        let mut buffer = [0.0_f32; 24];
        for (i, val) in buffer.iter_mut().enumerate() {
            *val = i as f32;
        }

        let ptr_buffer = from_slice(&buffer[..]);

        let index = 10;

        let value = ptr_buffer.get(index).unwrap();
        assert_eq!(value, buffer[index]);

        let value = ptr_buffer.get(index + 1).unwrap();
        assert_eq!(value, buffer[index + 1]);

        let value = ptr_buffer.get(index + 2).unwrap();
        assert_eq!(value, buffer[index + 2]);

        assert_eq!(ptr_buffer.get(ptr_buffer.length + 1), Err(IndexOutOfBound));
    }
}
