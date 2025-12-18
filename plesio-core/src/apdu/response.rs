use crate::apdu::status::ApduStatus;

#[derive(Clone, Copy)]
pub struct ApduResponse<'a> {
    data: &'a [u8],
    status: ApduStatus,
}

impl<'a> ApduResponse<'a> {
    pub fn expect_status(
        &self,
        matcher: impl FnOnce(&ApduStatus) -> Option<()>,
    ) -> Result<&'a [u8], Self> {
        self.status
            .expect(matcher)
            .map(|_| self.data)
            .map_err(|_| *self)
    }

    pub fn status(&self) -> ApduStatus {
        self.status
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }
}

pub trait FromResponse<'a> {
    fn from_response(response: ApduResponse<'a>) -> Self;
}
