pub mod iter;

pub struct TaggedSlice<'a> {
    tag: u8,
    value: &'a [u8],
}

impl<'a> TaggedSlice<'a> {
    pub fn next(slice: &'a [u8]) -> Option<(Self, &'a [u8])> {
        if slice.len() < 2 {
            return None;
        }

        let tag = slice[0];
        let length = slice[1] as usize;

        if slice.len() < 2 + length {
            return None;
        }

        let value = &slice[2..2 + length];
        let rest = &slice[2 + length..];

        Some((Self { tag, value }, rest))
    }

    pub fn from(tag: u8, value: &'a [u8]) -> Self {
        Self { tag, value }
    }

    pub fn tag(&self) -> u8 {
        self.tag
    }

    pub fn value(&self) -> &'a [u8] {
        self.value
    }
}
