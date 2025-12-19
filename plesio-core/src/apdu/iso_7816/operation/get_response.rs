use crate::apdu::{
    iso_7816::{
        class::Iso7816Class,
        operation::{Iso7816Command, Iso7816Operation},
    },
    response::ApduResponse,
};

pub struct GetResponse<'a> {
    trailer: Option<&'a mut [u8]>,
}

impl<'a> GetResponse<'a> {
    pub fn new(trailer: &'a mut [u8]) -> Self {
        Self {
            trailer: Some(trailer),
        }
    }
}

impl<'a> Iso7816Operation<'a> for GetResponse<'a> {
    type Result = ApduResponse<'a>;

    fn build(&mut self, class: Iso7816Class) -> (Iso7816Command<'a>, &'a mut [u8]) {
        let command = Iso7816Command {
            class,
            data: &[],
            instruction: 0xC0,
            parameters: (0x00, 0x00),
        };

        (command, self.trailer.take().unwrap())
    }

    fn parse(self, reply: &ApduResponse<'a>) -> Self::Result {
        *reply
    }
}
