#![no_std]

use core::{mem, slice};

pub struct RawBytes;

impl RawBytes {
    #[inline(always)]
    pub fn bytes_view<'t, T: Sync + Unpin + ?Sized + 't>(v: &T) -> &'t [u8] {
        unsafe { slice::from_raw_parts(v as *const _ as *const u8, mem::size_of_val(v)) }
    }

    #[inline(always)]
    pub fn bytes_view_mut<'t, T: Sync + Unpin + ?Sized + 't>(v: &mut T) -> &'t mut [u8] {
        unsafe { slice::from_raw_parts_mut(v as *mut _ as *mut u8, mem::size_of_val(v)) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[repr(C, packed(4))]
    struct Foo {
        x: [u32; 32],
    }

    #[test]
    fn test() {
        let mut foo = Foo { x: [0; 32] };

        let foo_bytes = RawBytes::bytes_view(&foo);
        assert_eq!(foo_bytes.len(), 128);

        let foo_bytes = RawBytes::bytes_view_mut(&mut foo);
        foo_bytes[0] = 1;
        assert_eq!(foo.x[0], 1);
    }
}
