use crate::apdu::status::ApduStatus;

#[derive(Clone, Copy)]
pub struct ApduResponse<'a> {
    data: &'a [u8],
    status: ApduStatus,
}

impl ApduResponse<'static> {
    pub fn empty(status: ApduStatus) -> Self {
        Self { data: &[], status }
    }
}

impl<'a> ApduResponse<'a> {
    pub fn parse(data: &'a [u8]) -> Option<Self> {
        data.split_at_checked(data.len() - 2)
            .map(|(data, s)| ApduResponse {
                data,
                status: ApduStatus::new(s[0], s[1]),
            })
    }

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
