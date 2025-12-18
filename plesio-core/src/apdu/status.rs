#[derive(Clone, Copy)]
pub struct ApduStatus(u8, u8);

impl ApduStatus {
    pub const fn new(code1: u8, code2: u8) -> Self {
        Self(code1, code2)
    }

    pub const fn as_u16(&self) -> u16 {
        (self.0 as u16) << 8 | self.1 as u16
    }

    pub const fn code1(&self) -> u8 {
        self.0
    }

    pub const fn code2(&self) -> u8 {
        self.1
    }

    pub fn expect(
        &self,
        matcher: impl FnOnce(&ApduStatus) -> Option<()>,
    ) -> Result<(), ApduStatus> {
        matcher(self).map(Ok).unwrap_or(Err(*self))
    }

    pub fn matches_if<T>(
        &self,
        matcher: impl FnOnce(&ApduStatus) -> Option<T>,
    ) -> Result<T, ApduStatus> {
        matcher(self).map(Ok).unwrap_or(Err(*self))
    }
}

pub fn is(status: ApduStatus) -> impl Fn(&ApduStatus) -> Option<()> {
    move |other| (other.as_u16() == status.as_u16()).then_some(())
}
