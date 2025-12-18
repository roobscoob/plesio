use crate::apdu::class::ApduClass;

pub trait ApduCommand {
    type Class: ApduClass;

    fn class(&self) -> Self::Class;
    fn instruction(&self) -> u8;
    fn parameters(&self) -> (u8, u8);
    fn data(&self) -> &[u8];
}
