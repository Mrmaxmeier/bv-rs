use BlockType;

use std::cmp::min;
use std::ops::{Index, IndexMut};
use std::ptr;

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Inner<Block>(Option<Box<[Block]>>);
// Invariant: self.invariant()

impl<Block: BlockType> Inner<Block> {
    #[allow(dead_code)]
    fn invariant(&self) -> bool {
        match self.0 {
            Some(ref b) => b.len() > 0,
            None        => true,
        }
    }

    pub fn new(init: Block, nblocks: usize) -> Self {
        Inner(if nblocks == 0 {
            None
        } else {
            Some(vec![init; nblocks].into_boxed_slice())
        })
    }

    pub fn clone_resize(&self, len: usize, new_cap: usize) -> Self {
        if new_cap == 0 {
            return Inner(None);
        }

        let mut result = vec![Block::zero(); new_cap].into_boxed_slice();

        for i in 0 .. min(len, new_cap) {
            result[i] = self[i];
        }

        Inner(Some(result))
    }

    pub fn len(&self) -> usize {
        match self.0 {
            Some(ref b) => b.len(),
            None        => 0,
        }
    }

    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn into_boxed_slice(self) -> Box<[Block]> {
        self.0.unwrap_or_else(<Box<[Block]>>::default)
    }

    pub fn as_ptr(&self) -> *const Block {
        match self.0 {
            Some(ref b) => b.as_ptr(),
            None        => ptr::null(),
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut Block {
        match self.0 {
            Some(ref mut b) => b.as_mut_ptr(),
            None            => ptr::null_mut(),
        }
    }
}

impl<Block> Index<usize> for Inner<Block> {
    type Output = Block;

    fn index(&self, index: usize) -> &Self::Output {
        match self.0 {
            Some(ref b) => &b[index],
            None        => panic!("BitVec::index: empty access"),
        }
    }
}

impl<Block> IndexMut<usize> for Inner<Block> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match self.0 {
            Some(ref mut b) => &mut b[index],
            None            => panic!("BitVec::index_mut: empty mut access"),
        }
    }
}