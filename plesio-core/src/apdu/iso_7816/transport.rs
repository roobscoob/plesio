use crate::apdu::{
    iso_7816::{
        class::Iso7816Class,
        operation::{Iso7816Command, Iso7816Operation, get_response::GetResponse},
        status,
    },
    response::ApduResponse,
    status::is,
    transport::{ApduTransport, PayloadTooLarge, TransportError},
};

/// The error type for ISO 7816 transport operations.
/// Wraps the underlying transport error or indicates protocol-specific issues.
#[derive(Debug)]
pub enum Iso7816TransportError<E> {
    Transport(E),
    ResponseBufferTooSmall { expected: usize, hint: usize },
}

pub struct Iso7816Transport<T: ApduTransport> {
    transport: T,
    state: Iso7816Class,
}

impl<T: ApduTransport> Iso7816Transport<T> {
    async fn execute_single<'a, O: Iso7816Operation<'a>>(
        &mut self,
        mut operation: O,
    ) -> Result<O::Result, Iso7816TransportError<T::TransportError>> {
        let (command, reply) = operation.build(self.state);
        // Map the underlying transport error to our wrapper
        let result = self
            .transport
            .execute(command, reply)
            .await
            .map_err(Iso7816TransportError::Transport)?;

        Ok(operation.parse(&result))
    }

    async fn execute_command_chunked<'a, 'b, 'c>(
        &mut self,
        command: Iso7816Command<'a>,
        reply: &'b mut [u8],
        chunk_reply_buffer: &'c mut [u8; 2],
    ) -> Result<ApduResponse<'c>, Iso7816TransportError<T::TransportError>>
    where
        'b: 'c,
    {
        let result = self
            .transport
            .execute(command, reply)
            .await
            .map_err(|e| (e.is_payload_too_large(), e));

        match result {
            Ok(_) => return Ok(ApduResponse::parse(reply).unwrap()),
            // Wrap the transport error
            Err((None, e)) => return Err(Iso7816TransportError::Transport(e)),
            Err((Some(PayloadTooLarge { max_size }), _)) => {
                if max_size == 0 {
                    panic!("Transport max size cannot be zero.");
                }

                for chunk in command.chunk(max_size) {
                    if !chunk.class.is_chaining() {
                        return self
                            .transport
                            .execute(chunk, reply)
                            .await
                            .map_err(Iso7816TransportError::Transport);
                    };

                    let response = self
                        .transport
                        .execute(chunk, chunk_reply_buffer)
                        .await
                        .map_err(Iso7816TransportError::Transport)?;

                    if response.status().expect(is(status::OK)).is_err() {
                        return Ok(ApduResponse::empty(response.status()));
                    }
                }
            }
        };

        unreachable!()
    }

    pub async fn execute<'a, O: Iso7816Operation<'a>>(
        &mut self,
        mut operation: O,
    ) -> Result<O::Result, Iso7816TransportError<T::TransportError>> {
        let mut chunk_reply_buffer = [0u8; 2];

        let (command, reply) = operation.build(self.state);

        let reply_length = reply.len();

        let mut response = self
            .execute_command_chunked(command, reply, &mut chunk_reply_buffer)
            .await?;

        if let Ok(size) = response.status().matches_if(status::has_wrong_length) {
            if size > reply_length {
                return Err(Iso7816TransportError::ResponseBufferTooSmall {
                    expected: size,
                    hint: reply_length,
                });
            }

            response = self
                .execute_command_chunked(command, &mut reply[0..size], &mut chunk_reply_buffer)
                .await?;
        }

        let mut offset = response.data().len();

        while let Ok(size) = response.status().matches_if(status::has_more_data) {
            if (offset + size) > reply_length {
                return Err(Iso7816TransportError::ResponseBufferTooSmall {
                    expected: offset + size,
                    hint: reply_length,
                });
            }

            response = self
                .execute_single(GetResponse::new(&mut reply[offset..offset + size]))
                .await?;

            offset += response.data().len();
        }

        Ok(operation.parse(&ApduResponse::parse(reply).unwrap()))
    }
}
