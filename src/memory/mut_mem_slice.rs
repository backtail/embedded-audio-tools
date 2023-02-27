use crate::memory::{
    MemSliceError::{self, *},
    MutMemoryPtr,
};

/// Raw slice pointer that implements the `Send` trait since it's only acting on static memory
#[derive(Clone, Copy)]
pub struct MutMemSlice {
    pub ptr: MutMemoryPtr,
    pub length: usize,
}

unsafe impl Sync for MutMemSlice {}

impl MutMemSlice {
    #[inline(always)]
    pub fn null() -> MutMemSlice {
        MutMemSlice {
            ptr: MutMemoryPtr(core::ptr::null_mut()),
            length: 0,
        }
    }

    pub fn get_sub_slice(
        &self,
        offset: usize,
        sub_length: usize,
    ) -> Result<MutMemSlice, MemSliceError> {
        if offset >= self.length {
            return Err(IndexOutOfBound);
        }

        if offset + sub_length >= self.length {
            return Err(LengthOutOfBound);
        }

        Ok(MutMemSlice {
            ptr: unsafe { MutMemoryPtr(self.ptr.0.add(offset)) },
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

    #[inline(always)]
    pub unsafe fn assign_unchecked(&mut self, index: usize, value: f32) {
        self.ptr.0.add(index).write(value);
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

    /// Only use this on static memory!
    #[inline(always)]
    pub unsafe fn set_slice(&mut self, ptr: *mut f32, length: usize) {
        self.ptr.0 = ptr;
        self.length = length;
    }

    #[inline(always)]
    pub fn as_slice(&mut self) -> *mut [f32] {
        core::ptr::slice_from_raw_parts_mut(self.ptr.0, self.length)
    }
}

#[inline(always)]
pub fn from_slice(slice: &mut [f32]) -> MutMemSlice {
    MutMemSlice {
        ptr: MutMemoryPtr(slice.as_mut_ptr()),
        length: slice.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assign_value() {
        let mut buffer = [0.0_f32; 24];
        let mut ptr_buffer = from_slice(&mut buffer[..]);

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
