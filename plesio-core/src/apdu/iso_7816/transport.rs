use crate::apdu::{
    iso_7816::{
        class::Iso7816Class,
        operation::{Iso7816Command, Iso7816Operation, get_response::GetResponse},
        status,
    },
    transport::ApduTransport,
};

pub struct Iso7816Transport<T: ApduTransport> {
    transport: T,
    state: Iso7816Class,
}

impl<T: ApduTransport> Iso7816Transport<T> {
    async fn execute_single<'a, O: Iso7816Operation<'a>>(
        &mut self,
        mut operation: O,
    ) -> Result<O::Result, T::TransportError> {
        let (command, reply) = operation.build(self.state);
        let result = self.transport.execute(command, reply).await?;
        Ok(operation.parse(&result))
    }

    pub async fn execute<'a, O: Iso7816Operation<'a>>(
        &mut self,
        mut operation: O,
    ) -> Result<O::Result, T::TransportError> {
        let (command, reply) = operation.build(self.state);
        let mut result = self.transport.execute(command, reply).await?;

        while result.status().matches_if(status::has_more_data).is_ok() {
            let offset = result.data().len();
            drop(result);
            let mut get_response = GetResponse::new(reply, offset);
            let (command, reply) = get_response.build(self.state);
            let get_response_result = self.transport.execute(command, reply).await?;
            result = get_response.parse(&get_response_result);
        }

        Ok(operation.parse(&result))
    }
}
