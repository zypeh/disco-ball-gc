use std::clone::Clone;
use std::ops::{Deref, DerefMut};
use std::marker::{Copy, PhantomPinned};
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

pub struct Gc<'a, T>(pub &'a T, pub Tag);

impl<'a, T> Gc<'a, T> {
    #[inline(always)]
    pub fn new(t: &'a T) -> Self {
        Gc(t, Tag(PhantomPinned))
    }
}

// Consider `PhantomData`?
// Prevent construction of data inside `Gc`
#[derive(Clone, Copy)]
pub struct Tag(PhantomPinned);

// Clone implemented for both GC and type T
impl<'a, T> Clone for Gc<'a, T> {
    fn clone(&self) -> Self {
        *self
    }
}

// Required to deal with Copy trait
impl<'a, T> Copy for Gc<'a, T> {} // TODO: what is this

/// A temporarily owned mutable value allocated in arena
pub struct Heap<'a, T>(pub &'a mut T, pub Tag);

impl<'a, T> Heap<'a, T> {
    pub fn new(t: &'a mut T) -> Self {
        Heap(t, Tag(PhantomPinned))
    }
}

impl<'a, T> Deref for Heap<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.0
    }
}

// Adding mut variant of Heap
impl<'a, T> DerefMut for Heap<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.0
    }
}

pub struct Root<T> {
    intern: *const RootIntern<T>,
}

pub struct RootIntern<T> {
    // using atomic pointer to prevent adding locks
    pub gc_ptr: AtomicPtr<T>,
    pub ref_count: AtomicUsize,
}

#[test]
fn create_gc() {
    use super::*;
    let gc = Gc::new(&String::from("a"));
}