use bitvec::vec::BitVec;

use super::{iterator::ArrayIterator, Array, ArrayBuilder};

pub struct StringArray {
    data: Vec<u8>,
    offsets: Vec<u32>,
    bitmap: BitVec,
}

impl Array for StringArray {
    type Builder = StringBuilder;

    type OwnedItem = String;

    type RefItem<'a> = &'a str;

    fn get(&self, idx: usize) -> Option<Self::RefItem<'_>> {
        if self.bitmap[idx] {
            let range = self.offsets[idx] as usize..self.offsets[idx + 1] as usize;
            Some(unsafe { std::str::from_utf8_unchecked(&self.data[range]) })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.bitmap.len()
    }

    fn iter(&self) -> super::iterator::ArrayIterator<Self> {
        ArrayIterator::new(self)
    }
}

pub struct StringBuilder {
    data: Vec<u8>,
    offsets: Vec<u32>,
    bitmap: BitVec,
}

impl ArrayBuilder for StringBuilder {
    type Array = StringArray;

    fn with_capacity(capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(capacity + 1);
        offsets.push(0);
        Self {
            data: Vec::with_capacity(capacity),
            offsets,
            bitmap: BitVec::with_capacity(capacity),
        }
    }

    fn push(&mut self, value: Option<<Self::Array as Array>::RefItem<'_>>) {
        match value {
            Some(v) => {
                self.bitmap.push(true);
                self.data.extend(v.as_bytes());
                self.offsets.push(self.data.len() as u32);
            }
            None => {
                self.bitmap.push(false);
                self.offsets.push(self.data.len() as u32);
            }
        }
    }

    fn finish(self) -> Self::Array {
        StringArray {
            data: self.data,
            offsets: self.offsets,
            bitmap: self.bitmap,
        }
    }
}
