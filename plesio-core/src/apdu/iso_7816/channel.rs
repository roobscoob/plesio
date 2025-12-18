#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Iso7816Channel {
    Basic(u8),
    Extended(u8),
}

impl Default for Iso7816Channel {
    fn default() -> Self {
        Iso7816Channel::Basic(0)
    }
}
