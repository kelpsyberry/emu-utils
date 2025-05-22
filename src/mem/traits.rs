use core::{
    mem::{self, MaybeUninit},
    ptr,
};
use std::alloc::{alloc_zeroed, handle_alloc_error, Layout};

/// # Safety
/// Any given byte pattern must be valid when interpreted as `Self`.
pub unsafe trait Fill8 {}

#[inline]
pub fn fill_8<T: Fill8 + ?Sized>(v: &mut T, fill_value: u8) {
    unsafe { ptr::write_bytes(v as *mut _ as *mut u8, fill_value, mem::size_of_val(v)) }
}

unsafe impl<T, const LEN: usize> Fill8 for [T; LEN] where T: Fill8 {}
unsafe impl<T> Fill8 for [T] where T: Fill8 {}
unsafe impl<T> Fill8 for MaybeUninit<T> where T: Fill8 {}

/// # Safety
/// A 0 byte pattern must be valid when interpreted as `Self`.
pub unsafe trait Zero {}

unsafe impl<T> Zero for *const T {}
unsafe impl<T> Zero for *mut T {}
unsafe impl<T, const LEN: usize> Zero for [T; LEN] where T: Zero {}
unsafe impl<T> Zero for [T] where T: Zero {}
unsafe impl<T> Zero for MaybeUninit<T> where T: Zero {}

#[inline]
pub fn zeroed_box<T: Zero>() -> Box<T> {
    unsafe {
        let layout = Layout::new::<T>();
        let ptr = alloc_zeroed(layout);
        if ptr.is_null() {
            handle_alloc_error(layout);
        }
        Box::from_raw(ptr.cast())
    }
}

#[inline]
pub fn zero<T: Zero>() -> T {
    unsafe { mem::MaybeUninit::zeroed().assume_init() }
}

#[inline]
pub fn make_zero<T: Zero + ?Sized>(v: &mut T) {
    unsafe { ptr::write_bytes(v as *mut _ as *mut u8, 0, mem::size_of_val(v)) }
}

mod sealed {
    pub trait MemValue {}

    macro_rules! impl_mem_value {
        ($($ty: ty),*) => {
            $(
                impl MemValue for $ty {}
            )*
        };
    }

    impl_mem_value!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);
}

pub trait MemValue: Sized + Copy + Zero + Fill8 + sealed::MemValue {
    fn from_le_bytes<const SIZE: usize>(bytes: [u8; SIZE]) -> Self;
    fn from_be_bytes<const SIZE: usize>(bytes: [u8; SIZE]) -> Self;
    fn from_ne_bytes<const SIZE: usize>(bytes: [u8; SIZE]) -> Self;

    fn to_le_bytes<const SIZE: usize>(self) -> [u8; SIZE];
    fn to_be_bytes<const SIZE: usize>(self) -> [u8; SIZE];
    fn to_ne_bytes<const SIZE: usize>(self) -> [u8; SIZE];

    fn le_byte(self, i: usize) -> u8;
    fn be_byte(self, i: usize) -> u8;
    fn ne_byte(self, i: usize) -> u8;

    /// # Safety
    /// The given pointer must be [valid] for `Self` reads and point to a properly initialized value
    /// of `Self`.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_le(ptr: *const Self) -> Self;
    /// # Safety
    /// The given pointer must be aligned to a `Self` boundary, be [valid] for `Self` reads and
    /// point to a properly initialized value of `Self`.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_le_aligned(ptr: *const Self) -> Self;
    /// # Safety
    /// The given pointer must be [valid] for `Self` reads and point to a properly initialized value
    /// of `Self`.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_be(ptr: *const Self) -> Self;
    /// # Safety
    /// The given pointer must be aligned to a `Self` boundary, be [valid] for `Self` reads and
    /// point to a properly initialized value of `Self`.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_be_aligned(ptr: *const Self) -> Self;
    /// # Safety
    /// The given pointer must be [valid] for `Self` reads and point to a properly initialized value
    /// of `Self`.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_ne(ptr: *const Self) -> Self;
    /// # Safety
    /// The given pointer must be aligned to a `Self` boundary, be [valid] for `Self` reads and
    /// point to a properly initialized value of `Self`.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_ne_aligned(ptr: *const Self) -> Self;

    /// # Safety
    /// The given pointer must be [valid] for `Self` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_le(self, ptr: *mut Self);
    /// # Safety
    /// The given pointer must be aligned to a `Self` boundary and be [valid] for `Self` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_le_aligned(self, ptr: *mut Self);
    /// # Safety
    /// The given pointer must be [valid] for `Self` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_be(self, ptr: *mut Self);
    /// # Safety
    /// The given pointer must be aligned to a `Self` boundary and be [valid] for `Self` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_be_aligned(self, ptr: *mut Self);
    /// # Safety
    /// The given pointer must be [valid] for `Self` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_ne(self, ptr: *mut Self);
    /// # Safety
    /// The given pointer must be aligned to a `Self` boundary and be [valid] for `Self` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_ne_aligned(self, ptr: *mut Self);
}

mod impl_primitive {
    use super::{Fill8, MemValue, Zero};
    use core::mem;

    macro_rules! impl_unsafe_trait {
        ($tr: ty; $($ty: ty),*) => {
            $(
                unsafe impl $tr for $ty {}
            )*
        };
    }

    impl_unsafe_trait!(Fill8; u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, f32, f64);
    impl_unsafe_trait!(Zero; u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize, bool, char, f32, f64);

    macro_rules! impl_mem_value {
        ($($ty: ty),*) => {
            $(
                impl MemValue for $ty {
                    #[inline]
                    fn from_le_bytes<const SIZE: usize>(bytes: [u8; SIZE]) -> Self {
                        assert!(SIZE == mem::size_of::<Self>());
                        <$ty>::from_le_bytes(unsafe { core::intrinsics::transmute_unchecked(bytes) })
                    }

                    #[inline]
                    fn from_be_bytes<const SIZE: usize>(bytes: [u8; SIZE]) -> Self {
                        assert!(SIZE == mem::size_of::<Self>());
                        <$ty>::from_be_bytes(unsafe { core::intrinsics::transmute_unchecked(bytes) })
                    }

                    #[inline]
                    fn from_ne_bytes<const SIZE: usize>(bytes: [u8; SIZE]) -> Self {
                        assert!(SIZE == mem::size_of::<Self>());
                        <$ty>::from_ne_bytes(unsafe { core::intrinsics::transmute_unchecked(bytes) })
                    }

                    #[inline]
                    fn to_le_bytes<const SIZE: usize>(self) -> [u8; SIZE] {
                        assert!(SIZE == mem::size_of::<Self>());
                        unsafe { core::intrinsics::transmute_unchecked(<$ty>::to_le_bytes(self)) }
                    }

                    #[inline]
                    fn to_be_bytes<const SIZE: usize>(self) -> [u8; SIZE] {
                        assert!(SIZE == mem::size_of::<Self>());
                        unsafe { core::intrinsics::transmute_unchecked(<$ty>::to_be_bytes(self)) }
                    }

                    #[inline]
                    fn to_ne_bytes<const SIZE: usize>(self) -> [u8; SIZE] {
                        assert!(SIZE == mem::size_of::<Self>());
                        unsafe { core::intrinsics::transmute_unchecked(<$ty>::to_ne_bytes(self)) }
                    }

                    #[inline]
                    fn le_byte(self, i: usize) -> u8 {
                        self.to_le_bytes()[i]
                    }

                    #[inline]
                    fn be_byte(self, i: usize) -> u8 {
                        self.to_be_bytes()[i]
                    }

                    #[inline]
                    fn ne_byte(self, i: usize) -> u8 {
                        self.to_ne_bytes()[i]
                    }

                    #[inline]
                    #[allow(unused_mut)]
                    unsafe fn read_le(ptr: *const Self) -> Self {
                        let mut res = ptr.read_unaligned();
                        #[cfg(not(target_endian = "little"))]
                        {
                            res = res.swap_bytes();
                        }
                        res
                    }

                    #[inline]
                    #[allow(unused_mut)]
                    unsafe fn read_le_aligned(ptr: *const Self) -> Self {
                        let mut res = ptr.read();
                        #[cfg(not(target_endian = "little"))]
                        {
                            res = res.swap_bytes();
                        }
                        res
                    }

                    #[inline]
                    unsafe fn read_be(ptr: *const Self) -> Self {
                        let mut res = ptr.read_unaligned();
                        #[cfg(not(target_endian = "big"))]
                        {
                            res = res.swap_bytes();
                        }
                        res
                    }

                    #[inline]
                    unsafe fn read_be_aligned(ptr: *const Self) -> Self {
                        let mut res = ptr.read();
                        #[cfg(not(target_endian = "big"))]
                        {
                            res = res.swap_bytes();
                        }
                        res
                    }

                    #[inline]
                    unsafe fn read_ne(ptr: *const Self) -> Self {
                        ptr.read_unaligned()
                    }

                    #[inline]
                    unsafe fn read_ne_aligned(ptr: *const Self) -> Self {
                        ptr.read()
                    }

                    #[inline]
                    #[allow(unused_mut)]
                    unsafe fn write_le(mut self, ptr: *mut Self) {
                        #[cfg(not(target_endian = "little"))]
                        {
                            self = self.swap_bytes();
                        }
                        ptr.write_unaligned(self);
                    }

                    #[inline]
                    #[allow(unused_mut)]
                    unsafe fn write_le_aligned(mut self, ptr: *mut Self) {
                        #[cfg(not(target_endian = "little"))]
                        {
                            self = self.swap_bytes();
                        }
                        ptr.write(self);
                    }

                    #[inline]
                    unsafe fn write_be(mut self, ptr: *mut Self) {
                        #[cfg(not(target_endian = "big"))]
                        {
                            self = self.swap_bytes();
                        }
                        ptr.write_unaligned(self);
                    }

                    #[inline]
                    unsafe fn write_be_aligned(mut self, ptr: *mut Self) {
                        #[cfg(not(target_endian = "big"))]
                        {
                            self = self.swap_bytes();
                        }
                        ptr.write(self);
                    }

                    #[inline]
                    unsafe fn write_ne(self, ptr: *mut Self) {
                        ptr.write_unaligned(self);
                    }

                    #[inline]
                    unsafe fn write_ne_aligned(self, ptr: *mut Self) {
                        ptr.write(self);
                    }
                }
            )*
        };
    }

    macro_rules! impl_mem_value_float {
        ($($ty: ty),*) => {
            $(
                impl MemValue for $ty {
                    #[inline]
                    fn from_le_bytes<const SIZE: usize>(bytes: [u8; SIZE]) -> Self {
                        assert!(SIZE == mem::size_of::<Self>());
                        <$ty>::from_le_bytes(unsafe { core::intrinsics::transmute_unchecked(bytes) })
                    }

                    #[inline]
                    fn from_be_bytes<const SIZE: usize>(bytes: [u8; SIZE]) -> Self {
                        assert!(SIZE == mem::size_of::<Self>());
                        <$ty>::from_be_bytes(unsafe { core::intrinsics::transmute_unchecked(bytes) })
                    }

                    #[inline]
                    fn from_ne_bytes<const SIZE: usize>(bytes: [u8; SIZE]) -> Self {
                        assert!(SIZE == mem::size_of::<Self>());
                        <$ty>::from_ne_bytes(unsafe { core::intrinsics::transmute_unchecked(bytes) })
                    }

                    #[inline]
                    fn to_le_bytes<const SIZE: usize>(self) -> [u8; SIZE] {
                        assert!(SIZE == mem::size_of::<Self>());
                        unsafe { core::intrinsics::transmute_unchecked(<$ty>::to_le_bytes(self)) }
                    }

                    #[inline]
                    fn to_be_bytes<const SIZE: usize>(self) -> [u8; SIZE] {
                        assert!(SIZE == mem::size_of::<Self>());
                        unsafe { core::intrinsics::transmute_unchecked(<$ty>::to_be_bytes(self)) }
                    }

                    #[inline]
                    fn to_ne_bytes<const SIZE: usize>(self) -> [u8; SIZE] {
                        assert!(SIZE == mem::size_of::<Self>());
                        unsafe { core::intrinsics::transmute_unchecked(<$ty>::to_ne_bytes(self)) }
                    }

                    #[inline]
                    fn le_byte(self, i: usize) -> u8 {
                        self.to_le_bytes()[i]
                    }

                    #[inline]
                    fn be_byte(self, i: usize) -> u8 {
                        self.to_be_bytes()[i]
                    }

                    #[inline]
                    fn ne_byte(self, i: usize) -> u8 {
                        self.to_ne_bytes()[i]
                    }

                    #[inline]
                    #[allow(unused_mut)]
                    unsafe fn read_le(ptr: *const Self) -> Self {
                        let mut res = ptr.read_unaligned();
                        #[cfg(not(target_endian = "little"))]
                        {
                            res = Self::from_bits(res.to_bits().swap_bytes());
                        }
                        res
                    }

                    #[inline]
                    #[allow(unused_mut)]
                    unsafe fn read_le_aligned(ptr: *const Self) -> Self {
                        let mut res = ptr.read();
                        #[cfg(not(target_endian = "little"))]
                        {
                            res = Self::from_bits(res.to_bits().swap_bytes());
                        }
                        res
                    }

                    #[inline]
                    unsafe fn read_be(ptr: *const Self) -> Self {
                        let mut res = ptr.read_unaligned();
                        #[cfg(not(target_endian = "big"))]
                        {
                            res = Self::from_bits(res.to_bits().swap_bytes());
                        }
                        res
                    }

                    #[inline]
                    unsafe fn read_be_aligned(ptr: *const Self) -> Self {
                        let mut res = ptr.read();
                        #[cfg(not(target_endian = "big"))]
                        {
                            res = Self::from_bits(res.to_bits().swap_bytes());
                        }
                        res
                    }

                    #[inline]
                    unsafe fn read_ne(ptr: *const Self) -> Self {
                        ptr.read_unaligned()
                    }

                    #[inline]
                    unsafe fn read_ne_aligned(ptr: *const Self) -> Self {
                        ptr.read()
                    }

                    #[inline]
                    #[allow(unused_mut)]
                    unsafe fn write_le(mut self, ptr: *mut Self) {
                        #[cfg(not(target_endian = "little"))]
                        {
                            self = Self::from_bits(self.to_bits().swap_bytes());
                        }
                        ptr.write_unaligned(self);
                    }

                    #[inline]
                    #[allow(unused_mut)]
                    unsafe fn write_le_aligned(mut self, ptr: *mut Self) {
                        #[cfg(not(target_endian = "little"))]
                        {
                            self = Self::from_bits(self.to_bits().swap_bytes());
                        }
                        ptr.write(self);
                    }

                    #[inline]
                    unsafe fn write_be(mut self, ptr: *mut Self) {
                        #[cfg(not(target_endian = "big"))]
                        {
                            self = Self::from_bits(self.to_bits().swap_bytes());
                        }
                        ptr.write_unaligned(self);
                    }

                    #[inline]
                    unsafe fn write_be_aligned(mut self, ptr: *mut Self) {
                        #[cfg(not(target_endian = "big"))]
                        {
                            self = Self::from_bits(self.to_bits().swap_bytes());
                        }
                        ptr.write(self);
                    }

                    #[inline]
                    unsafe fn write_ne(self, ptr: *mut Self) {
                        ptr.write_unaligned(self);
                    }

                    #[inline]
                    unsafe fn write_ne_aligned(self, ptr: *mut Self) {
                        ptr.write(self);
                    }
                }
            )*
        };
    }

    impl_mem_value!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
    impl_mem_value_float!(f32, f64);
}
