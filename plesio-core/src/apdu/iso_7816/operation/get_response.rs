use crate::apdu::{
    iso_7816::{
        class::Iso7816Class,
        operation::{Iso7816Command, Iso7816Operation},
    },
    response::ApduResponse,
};

pub struct GetResponse<'a> {
    current_data: &'a [u8],
    current_trailer: Option<&'a mut [u8]>,
}

impl<'a> GetResponse<'a> {
    pub fn new(data: &'a mut [u8], offset: usize) -> Self {
        let (current_data, current_trailer) = data.split_at_mut(offset);

        Self {
            current_data,
            current_trailer: Some(current_trailer),
        }
    }
}

impl<'a> Iso7816Operation<'a> for GetResponse<'a> {
    type Result = ApduResponse<'a>;

    fn build(&mut self, class: Iso7816Class) -> (Iso7816Command, &'a mut [u8]) {
        let command = Iso7816Command {
            class,
            data: &[],
            instruction: 0xC0,
            parameters: (0x00, 0x00),
        };

        (command, self.current_trailer.take().unwrap())
    }

    fn parse(self, reply: &ApduResponse<'a>) -> Self::Result {
        todo!()
    }
}
