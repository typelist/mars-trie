use std;
use entry::Entry;

#[derive(Copy, Clone, Debug)]
struct Union {
    // weight or terminal
    bits_: u32,
}

impl Union {
    fn new() -> Union {
        Union { bits_: 0 }
    }

    fn get_weight(&self) -> f32 {
        unsafe { std::mem::transmute(self.bits_) }
    }
    fn get_terminal(&self) -> usize {
        self.bits_ as usize
    }

    fn set_weight(&mut self, weight: f32) {
        self.bits_ = unsafe { std::mem::transmute(weight) };
    }
    fn set_terminal(&mut self, terminal: usize) {
        assert!(terminal <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.bits_ = terminal as u32;
    }
}

pub trait IKey<'a> {
    // Could replace this with Index trait
    fn at(&self, i: usize) -> u8;

    fn subslice(&mut self, pos: usize, length: usize);

    fn set_slice(&mut self, slice: &'a[u8]);
    fn set_weight(&mut self, weight: f32);
    fn set_terminal(&mut self, terminal: usize);
    fn set_id(&mut self, id: usize);

    fn get_slice(&self) -> &'a[u8];
    fn get_weight(&self) -> f32;
    fn get_terminal(&self) -> usize;
    fn get_id(&self) -> usize;

    fn len(&self) -> usize;
}

#[derive(Copy, Clone, Debug)]
pub struct Key<'a> {
    slice_: &'a[u8],
    union_: Union,
    id_: u32,
}

impl<'a> Key<'a> {
    pub fn new(slice: &'a[u8]) -> Key<'a> {
        Key { slice_: slice, union_: Union::new(), id_: 0 }
    }
    pub fn with_weight(&self, weight: f32) -> Self {
        let mut out = *self;
        out.set_weight(weight);
        out
    }
    pub fn from_key<T: IKey<'a>>(key: &T) -> Self {
        Self::new(key.get_slice()).with_weight(key.get_weight())
    }
}

impl<'a> From<&'a[u8]> for Key<'a> {
    fn from(slice: &'a[u8]) -> Key<'a> {
        Key::new(slice)
    }
}

impl<'a> IKey<'a> for Key<'a> {
    fn at(&self, i: usize) -> u8 {
        self.slice_[i]
    }
    fn subslice(&mut self, pos: usize, length: usize) {
        assert!(length <= self.slice_.len(), "MARISA_BOUND_ERROR");
        assert!(pos <= self.slice_.len() - length, "MARISA_BOUND_ERROR");
        self.slice_ = &self.slice_[pos..pos+length];
    }
    fn set_slice(&mut self, slice: &'a[u8]) {
        assert!(slice.len() <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.slice_ = slice;
    }
    fn set_weight(&mut self, weight: f32) {
        self.union_.set_weight(weight);
    }
    fn set_terminal(&mut self, terminal: usize) {
        self.union_.set_terminal(terminal);
    }
    fn set_id(&mut self, id: usize) {
        assert!(id <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.id_ = id as u32;
    }
    fn get_slice(&self) -> &'a[u8] {
        self.slice_
    }
    fn get_weight(&self) -> f32 {
        self.union_.get_weight()
    }
    fn get_terminal(&self) -> usize {
        self.union_.get_terminal()
    }
    fn get_id(&self) -> usize {
        self.id_ as usize
    }
    fn len(&self) -> usize {
        self.get_slice().len()
    }
}

impl<'a> PartialEq for Key<'a> {
    fn eq(&self, rhs: &Self) -> bool {
        self.get_slice() == rhs.get_slice()
    }
}

impl<'a> Eq for Key<'a> {}

impl<'a> PartialOrd for Key<'a> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.get_slice().partial_cmp(rhs.get_slice())
    }
}

impl<'a> Ord for Key<'a> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.get_slice().cmp(rhs.get_slice())
    }
}

/// Just like Key, except we index and subslice from the end of the slice
#[derive(Copy, Clone, Debug)]
pub struct ReverseKey<'a> {
    slice_: &'a[u8],
    union_: Union,
    id_: u32,
}

// FIXME: Reduce amount of identical code between Key and ReverseKey. Only
//        at() and subslice() are different at all!

impl<'a> ReverseKey<'a> {
    pub fn new(slice: &'a[u8]) -> ReverseKey<'a> {
        ReverseKey { slice_: slice, union_: Union::new(), id_: 0 }
    }
    pub fn with_weight(&self, weight: f32) -> Self {
        let mut out = *self;
        out.set_weight(weight);
        out
    }
    pub fn from_key<T: IKey<'a>>(key: &T) -> Self {
        Self::new(key.get_slice()).with_weight(key.get_weight())
    }
}

impl<'a> From<&'a[u8]> for ReverseKey<'a> {
    fn from(slice: &'a[u8]) -> ReverseKey<'a> {
        ReverseKey::new(slice)
    }
}

impl<'a> IKey<'a> for ReverseKey<'a> {
    fn at(&self, i: usize) -> u8 {
        self.slice_[self.slice_.len() - i - 1]
    }
    fn subslice(&mut self, pos: usize, length: usize) {
        assert!(length <= self.slice_.len(), "MARISA_BOUND_ERROR");
        assert!(pos <= self.slice_.len() - length, "MARISA_BOUND_ERROR");
        let new_end = self.slice_.len() - pos;
        let new_begin = new_end - length;
        self.slice_ = &self.slice_[new_begin..new_end];
    }
    fn set_slice(&mut self, slice: &'a[u8]) {
        assert!(slice.len() <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.slice_ = slice;
    }
    fn set_weight(&mut self, weight: f32) {
        self.union_.set_weight(weight);
    }
    fn set_terminal(&mut self, terminal: usize) {
        self.union_.set_terminal(terminal);
    }
    fn set_id(&mut self, id: usize) {
        assert!(id <= std::u32::MAX as usize, "MARISA_SIZE_ERROR");
        self.id_ = id as u32;
    }
    fn get_slice(&self) -> &'a[u8] {
        self.slice_
    }
    fn get_weight(&self) -> f32 {
        self.union_.get_weight()
    }
    fn get_terminal(&self) -> usize {
        self.union_.get_terminal()
    }
    fn get_id(&self) -> usize {
        self.id_ as usize
    }
    fn len(&self) -> usize {
        self.get_slice().len()
    }
}

impl<'a> PartialEq for ReverseKey<'a> {
    fn eq(&self, rhs: &Self) -> bool {
        self.get_slice() == rhs.get_slice()
    }
}

impl<'a> Eq for ReverseKey<'a> {}

impl<'a> PartialOrd for ReverseKey<'a> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        self.get_slice().partial_cmp(rhs.get_slice())
    }
}

impl<'a> Ord for ReverseKey<'a> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.get_slice().cmp(rhs.get_slice())
    }
}

