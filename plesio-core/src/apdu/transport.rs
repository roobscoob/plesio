use crate::apdu::{command::ApduCommand, response::ApduResponse};

pub trait ApduTransport {
    type TransportError;

    fn execute<'r>(
        &mut self,
        command: impl ApduCommand,
        reply_buffer: &'r mut [u8],
    ) -> impl Future<Output = Result<ApduResponse<'r>, Self::TransportError>>;
}
