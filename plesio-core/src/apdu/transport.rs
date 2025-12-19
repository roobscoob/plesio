use crate::apdu::{command::ApduCommand, response::ApduResponse};

#[derive(Debug, Clone, Copy)]
pub struct PayloadTooLarge {
    pub max_size: usize,
}

pub trait TransportError: core::fmt::Debug {
    /// If the error is due to the payload size being too large,
    /// returns the maximum allowed payload size for this transport.
    fn is_payload_too_large(&self) -> Option<PayloadTooLarge> {
        None
    }
}

pub trait ApduTransport {
    type TransportError: TransportError;

    fn execute<'r>(
        &mut self,
        command: impl ApduCommand,
        reply_buffer: &'r mut [u8],
    ) -> impl Future<Output = Result<ApduResponse<'r>, Self::TransportError>>;

    fn max_payload_size(&self) -> usize;
}
