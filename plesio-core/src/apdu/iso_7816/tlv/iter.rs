use crate::apdu::iso_7816::tlv::TaggedSlice;

#[derive(Clone, Copy)]
pub struct TlvIterator<'a> {
    data: &'a [u8],
}

impl<'a> TlvIterator<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    pub fn get(mut self, tag: u8) -> Option<TaggedSlice<'a>> {
        self.find(|slice| slice.tag() == tag)
    }
}

impl<'a> Iterator for TlvIterator<'a> {
    type Item = TaggedSlice<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let (slice, next_data) = TaggedSlice::next(self.data)?;
        self.data = next_data;
        Some(slice)
    }
}
