use {BlockType, Bits, BitsMut, BitsPush, BitSliceable, BitSlice, BitSliceMut};
use super::BitVec;
use iter::BlockIter;

use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Range, RangeFrom, RangeTo, RangeFull};
#[cfg(inclusive_range)]
use std::ops::{RangeInclusive, RangeToInclusive};

impl<Block: BlockType> Bits for BitVec<Block> {
    type Block = Block;

    fn bit_len(&self) -> u64 {
        self.len()
    }

    fn get_block(&self, position: usize) -> Block {
        assert!( position < self.block_len(),
                 "BitVec::get_block: out of bounds" );
        let count = Block::block_bits(self.bit_len(), position);
        self.bits[position].get_bits(0, count)
    }
}

impl<Block: BlockType> BitsMut for BitVec<Block> {
    fn set_block(&mut self, position: usize, value: Block) {
        assert!( position < self.block_len(),
                 "BitVec::set_block: out of bounds" );
        // This may set out-of-bounds bits, but that's okay because
        // oob bits are never observed.
        self.bits[position] = value;
    }
}

impl<Block: BlockType> BitsPush for BitVec<Block> {
    fn push_bit(&mut self, value: bool) {
        self.push(value);
    }

    fn pop_bit(&mut self) -> Option<bool> {
        self.pop()
    }

    fn align_block(&mut self, value: bool) {
        let keep_bits = Block::mod_nbits(self.len);
        if keep_bits > 0 {
            let last_index = self.block_len() - 1;
            let last = &mut self.bits[last_index];
            if value {
                *last = *last | !Block::low_mask(keep_bits);
            } else {
                *last = *last & Block::low_mask(keep_bits);
            }
            self.len += (Block::nbits() - keep_bits) as u64;
        }
    }

    fn push_block(&mut self, value: Block) {
        self.align_block(false);
        self.block_reserve(1);
        self.len += Block::nbits() as u64;
        let last = self.block_len() - 1;
        self.set_block(last, value);
    }
}

impl<'a, Block: BlockType> BitSliceable<Range<u64>> for &'a BitVec<Block> {
    type Slice = BitSlice<'a, Block>;

    fn bit_slice(self, range: Range<u64>) -> BitSlice<'a, Block> {
        self.as_slice().bit_slice(range)
    }
}

impl<'a, Block: BlockType> BitSliceable<Range<u64>> for &'a mut BitVec<Block> {
    type Slice = BitSliceMut<'a, Block>;

    fn bit_slice(self, range: Range<u64>) -> BitSliceMut<'a, Block> {
        self.as_mut_slice().bit_slice(range)
    }
}

#[cfg(inclusive_range)]
impl<'a, Block: BlockType> BitSliceable<RangeInclusive<u64>> for &'a BitVec<Block> {
    type Slice = BitSlice<'a, Block>;

    fn bit_slice(self, range: RangeInclusive<u64>) -> BitSlice<'a, Block> {
        self.as_slice().bit_slice(range)
    }
}

#[cfg(inclusive_range)]
impl<'a, Block: BlockType> BitSliceable<RangeInclusive<u64>> for &'a mut BitVec<Block> {
    type Slice = BitSliceMut<'a, Block>;

    fn bit_slice(self, range: RangeInclusive<u64>) -> BitSliceMut<'a, Block> {
        self.as_mut_slice().bit_slice(range)
    }
}

impl<'a, Block: BlockType> BitSliceable<RangeFrom<u64>> for &'a BitVec<Block> {
    type Slice = BitSlice<'a, Block>;

    fn bit_slice(self, range: RangeFrom<u64>) -> BitSlice<'a, Block> {
        self.as_slice().bit_slice(range)
    }
}

impl<'a, Block: BlockType> BitSliceable<RangeFrom<u64>> for &'a mut BitVec<Block> {
    type Slice = BitSliceMut<'a, Block>;

    fn bit_slice(self, range: RangeFrom<u64>) -> BitSliceMut<'a, Block> {
        self.as_mut_slice().bit_slice(range)
    }
}

impl<'a, Block: BlockType> BitSliceable<RangeTo<u64>> for &'a BitVec<Block> {
    type Slice = BitSlice<'a, Block>;

    fn bit_slice(self, range: RangeTo<u64>) -> BitSlice<'a, Block> {
        self.as_slice().bit_slice(range)
    }
}

impl<'a, Block: BlockType> BitSliceable<RangeTo<u64>> for &'a mut BitVec<Block> {
    type Slice = BitSliceMut<'a, Block>;

    fn bit_slice(self, range: RangeTo<u64>) -> BitSliceMut<'a, Block> {
        self.as_mut_slice().bit_slice(range)
    }
}

#[cfg(inclusive_range)]
impl<'a, Block: BlockType> BitSliceable<RangeToInclusive<u64>> for &'a BitVec<Block> {
    type Slice = BitSlice<'a, Block>;

    fn bit_slice(self, range: RangeToInclusive<u64>) -> BitSlice<'a, Block> {
        self.as_slice().bit_slice(range)
    }
}

#[cfg(inclusive_range)]
impl<'a, Block: BlockType> BitSliceable<RangeToInclusive<u64>> for &'a mut BitVec<Block> {
    type Slice = BitSliceMut<'a, Block>;

    fn bit_slice(self, range: RangeToInclusive<u64>) -> BitSliceMut<'a, Block> {
        self.as_mut_slice().bit_slice(range)
    }
}

impl<'a, Block: BlockType> BitSliceable<RangeFull> for &'a BitVec<Block> {
    type Slice = BitSlice<'a, Block>;

    fn bit_slice(self, _: RangeFull) -> BitSlice<'a, Block> {
        self.as_slice()
    }
}

impl<'a, Block: BlockType> BitSliceable<RangeFull> for &'a mut BitVec<Block> {
    type Slice = BitSliceMut<'a, Block>;

    fn bit_slice(self, _: RangeFull) -> BitSliceMut<'a, Block> {
        self.as_mut_slice()
    }
}

impl_index_from_bits! {
    impl[Block: BlockType] Index<u64> for BitVec<Block>;
}

impl<Other: Bits> PartialEq<Other> for BitVec<Other::Block> {
    fn eq(&self, other: &Other) -> bool {
        BlockIter::new(self) == BlockIter::new(other)
    }
}

impl<Block: BlockType> PartialOrd for BitVec<Block> {
    fn partial_cmp(&self, other: &BitVec<Block>) -> Option<Ordering> {
        let iter1 = BlockIter::new(self);
        let iter2 = BlockIter::new(other);
        iter1.partial_cmp(iter2)
    }
}

impl<Block: BlockType> Eq for BitVec<Block> {}

impl<Block: BlockType> Ord for BitVec<Block> {
    fn cmp(&self, other: &Self) -> Ordering {
        let iter1 = BlockIter::new(self);
        let iter2 = BlockIter::new(other);
        iter1.cmp(iter2)
    }
}

impl<Block: BlockType + Hash> Hash for BitVec<Block> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state);
    }
}

impl<Block: BlockType> fmt::Debug for BitVec<Block> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_slice().fmt(f)
    }
}