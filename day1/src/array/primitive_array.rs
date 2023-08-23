use bitvec::vec::BitVec;
use std::fmt::Debug;

use super::{iterator::ArrayIterator, Array, ArrayBuilder};

pub trait PrimitiveType: Copy + Send + Sync + Default + Debug + 'static {}

impl PrimitiveType for i32 {}
impl PrimitiveType for i64 {}

pub struct PrimitiveArray<T: PrimitiveType> {
    data: Vec<T>,
    bitmap: BitVec,
}

impl<T: PrimitiveType> Array for PrimitiveArray<T> {
    type Builder = PrimitiveBuilder<T>;

    type OwnedItem = T;

    type RefItem<'a> = T;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        if self.bitmap[idx] {
            Some(self.data[idx])
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
    fn iter(&self) -> ArrayIterator<Self> {
        ArrayIterator::new(self)
    }
}

pub struct PrimitiveBuilder<T: PrimitiveType> {
    data: Vec<T>,
    bitmap: BitVec,
}

impl<T: PrimitiveType> ArrayBuilder for PrimitiveBuilder<T> {
    type Array = PrimitiveArray<T>;

    fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            bitmap: BitVec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        match value {
            Some(v) => {
                self.data.push(v);
                self.bitmap.push(true);
            }
            None => {
                self.data.push(T::default());
                self.bitmap.push(false);
            }
        }
    }

    fn finish(self) -> Self::Array {
        PrimitiveArray {
            data: self.data,
            bitmap: self.bitmap,
        }
    }
}
