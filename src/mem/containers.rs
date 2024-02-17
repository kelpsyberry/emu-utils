use super::{Fill8, MemValue, Zero};
use core::{
    mem,
    ops::{Deref, DerefMut},
    ptr,
};
use std::alloc::{alloc, alloc_zeroed, dealloc, Layout};

pub trait ByteSlice {
    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `u8` reads.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_unchecked(&self, off: usize) -> u8;

    fn read(&self, off: usize) -> u8;

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `u8` reads.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_le_unchecked<T: MemValue>(&self, off: usize) -> T;

    fn read_le<T: MemValue>(&self, off: usize) -> T;

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` reads and aligned to a `T`
    /// boundary.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_le_aligned_unchecked<T: MemValue>(&self, off: usize) -> T;

    /// # Safety
    /// The resulting pointer from offsetting must be aligned to a `T` boundary.
    unsafe fn read_le_aligned<T: MemValue>(&self, off: usize) -> T;

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` reads.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_be_unchecked<T: MemValue>(&self, off: usize) -> T;

    fn read_be<T: MemValue>(&self, off: usize) -> T;

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` reads and aligned to a `T`
    /// boundary.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_be_aligned_unchecked<T: MemValue>(&self, off: usize) -> T;

    /// # Safety
    /// The resulting pointer from offsetting must be aligned to a `T` boundary.
    unsafe fn read_be_aligned<T: MemValue>(&self, off: usize) -> T;

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` reads.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_ne_unchecked<T: MemValue>(&self, off: usize) -> T;

    fn read_ne<T: MemValue>(&self, off: usize) -> T;

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` reads and aligned to a `T`
    /// boundary.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn read_ne_aligned_unchecked<T: MemValue>(&self, off: usize) -> T;

    /// # Safety
    /// The resulting pointer from offsetting must be aligned to a `T` boundary.
    unsafe fn read_ne_aligned<T: MemValue>(&self, off: usize) -> T;
}

macro_rules! impl_reads {
    () => {
        #[inline]
        unsafe fn read_unchecked(&self, off: usize) -> u8 {
            *self.as_ptr().add(off)
        }

        #[inline]
        fn read(&self, off: usize) -> u8 {
            assert!(self.len() > off);
            unsafe { *self.as_ptr().add(off) }
        }

        #[inline]
        unsafe fn read_le_unchecked<T: MemValue>(&self, off: usize) -> T {
            T::read_le(self.as_ptr().add(off) as *const T)
        }

        #[inline]
        fn read_le<T: MemValue>(&self, off: usize) -> T {
            assert!(self.len() >= off + mem::size_of::<T>());
            unsafe { T::read_le(self.as_ptr().add(off) as *const T) }
        }

        #[inline]
        unsafe fn read_le_aligned_unchecked<T: MemValue>(&self, off: usize) -> T {
            T::read_le_aligned(self.as_ptr().add(off) as *const T)
        }

        #[inline]
        unsafe fn read_le_aligned<T: MemValue>(&self, off: usize) -> T {
            assert!(self.len() >= off + mem::size_of::<T>());
            T::read_le_aligned(self.as_ptr().add(off) as *const T)
        }

        #[inline]
        unsafe fn read_be_unchecked<T: MemValue>(&self, off: usize) -> T {
            T::read_be(self.as_ptr().add(off) as *const T)
        }

        #[inline]
        fn read_be<T: MemValue>(&self, off: usize) -> T {
            assert!(self.len() >= off + mem::size_of::<T>());
            unsafe { T::read_be(self.as_ptr().add(off) as *const T) }
        }

        #[inline]
        unsafe fn read_be_aligned_unchecked<T: MemValue>(&self, off: usize) -> T {
            T::read_be_aligned(self.as_ptr().add(off) as *const T)
        }

        #[inline]
        unsafe fn read_be_aligned<T: MemValue>(&self, off: usize) -> T {
            assert!(self.len() >= off + mem::size_of::<T>());
            T::read_be_aligned(self.as_ptr().add(off) as *const T)
        }

        #[inline]
        unsafe fn read_ne_unchecked<T: MemValue>(&self, off: usize) -> T {
            T::read_ne(self.as_ptr().add(off) as *const T)
        }

        #[inline]
        fn read_ne<T: MemValue>(&self, off: usize) -> T {
            assert!(self.len() >= off + mem::size_of::<T>());
            unsafe { T::read_ne(self.as_ptr().add(off) as *const T) }
        }

        #[inline]
        unsafe fn read_ne_aligned_unchecked<T: MemValue>(&self, off: usize) -> T {
            T::read_ne_aligned(self.as_ptr().add(off) as *const T)
        }

        #[inline]
        unsafe fn read_ne_aligned<T: MemValue>(&self, off: usize) -> T {
            assert!(self.len() >= off + mem::size_of::<T>());
            T::read_ne_aligned(self.as_ptr().add(off) as *const T)
        }
    };
}

pub trait ByteMutSlice {
    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `u8` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_unchecked(&mut self, off: usize, value: u8);

    fn write(&mut self, off: usize, value: u8);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_le_unchecked<T: MemValue>(&mut self, off: usize, value: T);

    fn write_le<T: MemValue>(&mut self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes and aligned to a
    /// `T` boundary.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_le_aligned_unchecked<T: MemValue>(&mut self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be aligned to a `T` boundary.
    unsafe fn write_le_aligned<T: MemValue>(&mut self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_be_unchecked<T: MemValue>(&mut self, off: usize, value: T);

    fn write_be<T: MemValue>(&mut self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes and aligned to a
    /// `T` boundary.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_be_aligned_unchecked<T: MemValue>(&mut self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be aligned to a `T` boundary.
    unsafe fn write_be_aligned<T: MemValue>(&mut self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_ne_unchecked<T: MemValue>(&mut self, off: usize, value: T);

    fn write_ne<T: MemValue>(&mut self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes and aligned to a
    /// `T` boundary.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_ne_aligned_unchecked<T: MemValue>(&mut self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be aligned to a `T` boundary.
    unsafe fn write_ne_aligned<T: MemValue>(&mut self, off: usize, value: T);
}

pub trait ByteMutSliceOwnedPtr {
    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `u8` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_unchecked(&self, off: usize, value: u8);

    fn write(&self, off: usize, value: u8);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_le_unchecked<T: MemValue>(&self, off: usize, value: T);

    fn write_le<T: MemValue>(&self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes and aligned to a
    /// `T` boundary.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_le_aligned_unchecked<T: MemValue>(&self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be aligned to a `T` boundary.
    unsafe fn write_le_aligned<T: MemValue>(&self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_be_unchecked<T: MemValue>(&self, off: usize, value: T);

    fn write_be<T: MemValue>(&self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes and aligned to a
    /// `T` boundary.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_be_aligned_unchecked<T: MemValue>(&self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be aligned to a `T` boundary.
    unsafe fn write_be_aligned<T: MemValue>(&self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_ne_unchecked<T: MemValue>(&self, off: usize, value: T);

    fn write_ne<T: MemValue>(&self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be [valid] for `T` writes and aligned to a
    /// `T` boundary.
    ///
    /// [valid]: https://doc.rust-lang.org/stable/std/ptr/index.html#safety
    unsafe fn write_ne_aligned_unchecked<T: MemValue>(&self, off: usize, value: T);

    /// # Safety
    /// The resulting pointer from offsetting must be aligned to a `T` boundary.
    unsafe fn write_ne_aligned<T: MemValue>(&self, off: usize, value: T);
}

macro_rules! impl_writes {
    ($($mut: ident)?) => {
        #[inline]
        unsafe fn write_unchecked(&$($mut)* self, off: usize, value: u8) {
            *self.as_mut_ptr().add(off) = value;
        }

        #[inline]
        fn write(&$($mut)* self, off: usize, value: u8) {
            assert!(self.len() > off);
            unsafe {
                *self.as_mut_ptr().add(off) = value;
            }
        }

        #[inline]
        unsafe fn write_le_unchecked<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            value.write_le(self.as_mut_ptr().add(off) as *mut T)
        }

        #[inline]
        fn write_le<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            assert!(self.len() >= off + mem::size_of::<T>());
            unsafe { value.write_le(self.as_mut_ptr().add(off) as *mut T) }
        }

        #[inline]
        unsafe fn write_le_aligned_unchecked<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            value.write_le_aligned(self.as_mut_ptr().add(off) as *mut T)
        }

        #[inline]
        unsafe fn write_le_aligned<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            assert!(self.len() >= off + mem::size_of::<T>());
            value.write_le_aligned(self.as_mut_ptr().add(off) as *mut T)
        }

        #[inline]
        unsafe fn write_be_unchecked<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            value.write_be(self.as_mut_ptr().add(off) as *mut T)
        }

        #[inline]
        fn write_be<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            assert!(self.len() >= off + mem::size_of::<T>());
            unsafe { value.write_be(self.as_mut_ptr().add(off) as *mut T) }
        }

        #[inline]
        unsafe fn write_be_aligned_unchecked<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            value.write_be_aligned(self.as_mut_ptr().add(off) as *mut T)
        }

        #[inline]
        unsafe fn write_be_aligned<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            assert!(self.len() >= off + mem::size_of::<T>());
            value.write_be_aligned(self.as_mut_ptr().add(off) as *mut T)
        }

        #[inline]
        unsafe fn write_ne_unchecked<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            value.write_ne(self.as_mut_ptr().add(off) as *mut T)
        }

        #[inline]
        fn write_ne<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            assert!(self.len() >= off + mem::size_of::<T>());
            unsafe { value.write_ne(self.as_mut_ptr().add(off) as *mut T) }
        }

        #[inline]
        unsafe fn write_ne_aligned_unchecked<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            value.write_ne_aligned(self.as_mut_ptr().add(off) as *mut T)
        }

        #[inline]
        unsafe fn write_ne_aligned<T: MemValue>(&$($mut)* self, off: usize, value: T) {
            assert!(self.len() >= off + mem::size_of::<T>());
            value.write_ne_aligned(self.as_mut_ptr().add(off) as *mut T)
        }
    };
}

impl ByteSlice for [u8] {
    impl_reads!();
}

impl ByteMutSlice for [u8] {
    impl_writes!(mut);
}

impl<const LEN: usize> ByteSlice for [u8; LEN] {
    impl_reads!();
}

impl<const LEN: usize> ByteMutSlice for [u8; LEN] {
    impl_writes!(mut);
}

#[repr(C, align(8))]
#[derive(Clone)]
pub struct Bytes<const LEN: usize>([u8; LEN]);

unsafe impl<const LEN: usize> Zero for Bytes<LEN> {}
unsafe impl<const LEN: usize> Fill8 for Bytes<LEN> {}

impl<const LEN: usize> Bytes<LEN> {
    #[inline]
    pub const fn new(value: [u8; LEN]) -> Self {
        Bytes(value)
    }

    #[inline]
    pub const fn into_inner(self) -> [u8; LEN] {
        self.0
    }
}

impl<const LEN: usize> ByteSlice for Bytes<LEN> {
    impl_reads!();
}

impl<const LEN: usize> ByteMutSlice for Bytes<LEN> {
    impl_writes!(mut);
}

impl<const LEN: usize> Deref for Bytes<LEN> {
    type Target = [u8; LEN];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const LEN: usize> DerefMut for Bytes<LEN> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const LEN: usize> From<[u8; LEN]> for Bytes<LEN> {
    #[inline]
    fn from(other: [u8; LEN]) -> Self {
        Self::new(other)
    }
}

impl<const LEN: usize> From<Bytes<LEN>> for [u8; LEN] {
    #[inline]
    fn from(other: Bytes<LEN>) -> Self {
        other.0
    }
}

// This is necessary to sidestep the stacked borrows model (theoretically): a raw pointer is
// required so that a borrow might be derived from it, but `Box<T>` receives special treatment.
pub struct OwnedBytesCellPtr<const LEN: usize>(*mut Bytes<LEN>);

impl<const LEN: usize> OwnedBytesCellPtr<LEN> {
    /// # Safety
    /// The given pointer must point to a valid value of type `Bytes<LEN>` and be valid for reads
    /// and writes for the entire lifetime of this value.
    #[inline]
    pub unsafe fn new(ptr: *mut Bytes<LEN>) -> Self {
        OwnedBytesCellPtr(ptr)
    }

    #[inline]
    pub fn new_zeroed() -> Self {
        unsafe { OwnedBytesCellPtr(alloc_zeroed(Layout::new::<Bytes<LEN>>()) as *mut Bytes<LEN>) }
    }

    #[inline]
    pub const fn len(&self) -> usize {
        LEN
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn into_inner(self) -> *mut Bytes<LEN> {
        self.0
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.cast()
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.0.cast()
    }

    #[inline]
    pub fn as_arr_ptr(&self) -> *const [u8; LEN] {
        self.0.cast()
    }

    #[inline]
    pub fn as_arr_mut_ptr(&self) -> *mut [u8; LEN] {
        self.0.cast()
    }

    /// # Safety
    /// The lifetime of the returned value must not intersect with those of other unique references
    /// to the slice.
    #[inline]
    pub unsafe fn as_arr(&self) -> &[u8; LEN] {
        &*self.as_arr_ptr()
    }

    /// # Safety
    /// The lifetime of the returned value must not intersect with those of other references to the
    /// slice.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_mut_arr(&self) -> &mut [u8; LEN] {
        &mut *self.as_arr_mut_ptr()
    }

    #[inline]
    pub fn as_bytes_ptr(&self) -> *const Bytes<LEN> {
        self.0
    }

    #[inline]
    pub fn as_bytes_mut_ptr(&self) -> *mut Bytes<LEN> {
        self.0
    }

    /// # Safety
    /// The lifetime of the returned value must not intersect with those of other unique references
    /// to the slice.
    #[inline]
    pub unsafe fn as_bytes(&self) -> &Bytes<LEN> {
        &*self.as_bytes_ptr()
    }

    /// # Safety
    /// The lifetime of the returned value must not intersect with those of other references to the
    /// slice.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_mut_bytes(&self) -> &mut Bytes<LEN> {
        &mut *self.as_bytes_mut_ptr()
    }

    #[inline]
    pub fn as_slice_ptr(&self) -> *const [u8] {
        ptr::slice_from_raw_parts(self.0.cast(), LEN)
    }

    #[inline]
    pub fn as_slice_mut_ptr(&self) -> *mut [u8] {
        ptr::slice_from_raw_parts_mut(self.0.cast(), LEN)
    }
}

impl<const LEN: usize> ByteSlice for OwnedBytesCellPtr<LEN> {
    impl_reads!();
}

impl<const LEN: usize> ByteMutSliceOwnedPtr for OwnedBytesCellPtr<LEN> {
    impl_writes!();
}

impl<const LEN: usize> Drop for OwnedBytesCellPtr<LEN> {
    #[inline]
    fn drop(&mut self) {
        unsafe { dealloc(self.0 as *mut u8, Layout::new::<Bytes<LEN>>()) }
    }
}

impl<const LEN: usize> From<Box<Bytes<LEN>>> for OwnedBytesCellPtr<LEN> {
    #[inline]
    fn from(other: Box<Bytes<LEN>>) -> Self {
        OwnedBytesCellPtr(Box::into_raw(other))
    }
}

impl<const LEN: usize> From<OwnedBytesCellPtr<LEN>> for Box<Bytes<LEN>> {
    #[inline]
    fn from(other: OwnedBytesCellPtr<LEN>) -> Self {
        let result = unsafe { Box::from_raw(other.0) };
        mem::forget(other);
        result
    }
}

#[derive(Clone)]
pub struct BoxedByteSlice(mem::ManuallyDrop<Box<[u8]>>);

impl BoxedByteSlice {
    #[inline]
    pub fn new_zeroed(len: usize) -> Self {
        let layout = Layout::from_size_align((len + 7) & !7, 8).unwrap();
        unsafe {
            BoxedByteSlice(mem::ManuallyDrop::new(Box::from_raw(
                core::slice::from_raw_parts_mut(alloc_zeroed(layout), len),
            )))
        }
    }
}

impl Drop for BoxedByteSlice {
    #[inline]
    fn drop(&mut self) {
        let layout = Layout::from_size_align((self.0.len() + 7) & !7, 8).unwrap();
        unsafe {
            dealloc(
                Box::into_raw(mem::ManuallyDrop::take(&mut self.0)) as *mut u8,
                layout,
            )
        }
    }
}

impl ByteSlice for BoxedByteSlice {
    impl_reads!();
}

impl ByteMutSlice for BoxedByteSlice {
    impl_writes!(mut);
}

impl Deref for BoxedByteSlice {
    type Target = [u8];
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for BoxedByteSlice {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(C)]
pub struct OwnedByteSliceCellPtr(*mut u8, usize);

impl Clone for OwnedByteSliceCellPtr {
    fn clone(&self) -> Self {
        let layout = Layout::from_size_align((self.1 + 7) & !7, 8).unwrap();
        unsafe {
            let buffer = alloc(layout);
            ptr::copy_nonoverlapping(self.0, buffer, self.1);
            OwnedByteSliceCellPtr(buffer, self.1)
        }
    }
}

impl OwnedByteSliceCellPtr {
    /// # Safety
    /// The given pointer must point to a valid value of type `[u8]` with the specified length, be
    /// valid for reads and writes and be aligned to an 8-byte boundary.
    #[inline]
    pub unsafe fn new(ptr: *mut u8, len: usize) -> Self {
        OwnedByteSliceCellPtr(ptr, len)
    }

    #[inline]
    pub fn new_zeroed(len: usize) -> Self {
        let layout = Layout::from_size_align((len + 7) & !7, 8).unwrap();
        unsafe { OwnedByteSliceCellPtr(alloc_zeroed(layout), len) }
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.1
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.0.cast()
    }

    #[inline]
    pub fn as_mut_ptr(&self) -> *mut u8 {
        self.0.cast()
    }

    #[inline]
    pub fn as_slice_ptr(&self) -> *const [u8] {
        ptr::slice_from_raw_parts(self.0, self.1)
    }

    #[inline]
    pub fn as_slice_mut_ptr(&self) -> *mut [u8] {
        ptr::slice_from_raw_parts_mut(self.0, self.1)
    }

    /// # Safety
    /// The lifetime of the returned value must not intersect with those of other unique references
    /// to the slice.
    #[inline]
    pub unsafe fn as_slice(&self) -> &[u8] {
        &*self.as_slice_ptr()
    }

    /// # Safety
    /// The lifetime of the returned value must not intersect with those of other references to the
    /// slice.
    #[inline]
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_mut_slice(&self) -> &mut [u8] {
        &mut *self.as_slice_mut_ptr()
    }
}

impl Drop for OwnedByteSliceCellPtr {
    #[inline]
    fn drop(&mut self) {
        let layout = Layout::from_size_align((self.1 + 7) & !7, 8).unwrap();
        unsafe { dealloc(self.0, layout) }
    }
}

impl ByteSlice for OwnedByteSliceCellPtr {
    impl_reads!();
}

impl ByteMutSliceOwnedPtr for OwnedByteSliceCellPtr {
    impl_writes!();
}
