use core::mem::transmute;

pub(crate) trait Convert<To> {
    fn convert(self) -> To;
    fn convert_ref(&self) -> &To;
    fn convert_mut_ref(&mut self) -> &mut To;
}
macro_rules! convert {
    ($from:ty, $to:ty) => {
        impl Convert<$to> for $from {
            #[inline(always)]
            fn convert(self) -> $to {
                unsafe { transmute(self) }
            }
            #[inline(always)]
            fn convert_ref(&self) -> &$to {
                unsafe { transmute(self) }
            }
            #[inline(always)]
            fn convert_mut_ref(&mut self) -> &mut $to {
                unsafe { transmute(self) }
            }
        }
        impl Convert<$from> for $to {
            #[inline(always)]
            fn convert(self) -> $from {
                unsafe { transmute(self) }
            }
            #[inline(always)]
            fn convert_ref(&self) -> &$from {
                unsafe { transmute(self) }
            }
            #[inline(always)]
            fn convert_mut_ref(&mut self) -> &mut $from {
                unsafe { transmute(self) }
            }
        }
    };
}
convert!(u128, [u64; 2]);
convert!(u128, [u32; 4]);
convert!(u128, [u16; 8]);
convert!(u128, [u8; 16]);
convert!([u64; 2], [u32; 4]);
convert!([u64; 2], [u16; 8]);
convert!([u64; 2], [u8; 16]);
convert!([u32; 4], [u16; 8]);
convert!([u32; 4], [u8; 16]);
convert!([u16; 8], [u8; 16]);
convert!(u64, [u32; 2]);
convert!(u64, [u16; 4]);
convert!(u64, [u8; 8]);
convert!([u32; 2], [u16; 4]);
convert!([u32; 2], [u8; 8]);
convert!(u32, [u16; 2]);
convert!(u32, [u8; 4]);
convert!([u16; 2], [u8; 4]);
convert!(u16, [u8; 2]);

convert!([f64; 2], [u8; 16]);
convert!([f32; 4], [u8; 16]);
convert!(f64, [u8; 8]);
convert!([f32; 2], [u8; 8]);
convert!(f32, [u8; 4]);



macro_rules! as_array {
    ($input:expr, $len:expr) => {{
        {
            #[inline(always)]
            fn as_array<T>(slice: &[T]) -> &[T; $len] {
                assert_eq!(slice.len(), $len);
                unsafe {
                    &*(slice.as_ptr() as *const [_; $len])
                }
            }
            as_array($input)
        }
    }}
}

pub(crate) trait ReadFromSlice {
    fn read_u16(&self) -> (u16, &[u8]);
    fn read_u32(&self) -> (u32, &[u8]);
    fn read_u64(&self) -> (u64, &[u8]);
    fn read_u128(&self) -> (u128, &[u8]);
    fn read_last_u16(&self) -> u16;
    fn read_last_u32(&self) -> u32;
    fn read_last_u64(&self) -> u64;
    fn read_last_u128(&self) -> u128;
}

impl ReadFromSlice for [u8] {
    #[inline(always)]
    fn read_u16(&self) -> (u16, &[u8]) {
        let (value, rest) = self.split_at(2);
        (as_array!(value, 2).convert(), rest)
    }

    #[inline(always)]
    fn read_u32(&self) -> (u32, &[u8]) {
        let (value, rest) = self.split_at(4);
        (as_array!(value, 4).convert(), rest)
    }

    #[inline(always)]
    fn read_u64(&self) -> (u64, &[u8]) {
        let (value, rest) = self.split_at(8);
        (as_array!(value, 8).convert(), rest)
    }

    #[inline(always)]
    fn read_u128(&self) -> (u128, &[u8]) {
        let (value, rest) = self.split_at(16);
        (as_array!(value, 16).convert(), rest)
    }


    #[inline(always)]
    fn read_last_u16(&self) -> u16 {
        let (_, value) = self.split_at(self.len() - 2);
        as_array!(value, 2).convert()
    }

    #[inline(always)]
    fn read_last_u32(&self) -> u32 {
        let (_, value) = self.split_at(self.len() - 4);
        as_array!(value, 4).convert()
    }

    #[inline(always)]
    fn read_last_u64(&self) -> u64 {
        let (_, value) = self.split_at(self.len() - 8);
        as_array!(value, 8).convert()
    }

    #[inline(always)]
    fn read_last_u128(&self) -> u128 {
        let (_, value) = self.split_at(self.len() - 16);
        as_array!(value, 16).convert()
    }
}
