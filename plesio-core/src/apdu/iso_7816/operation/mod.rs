pub mod chunking;
pub mod get_response;
pub mod select;

use crate::apdu::{
    command::ApduCommand,
    iso_7816::{class::Iso7816Class, operation::chunking::CommandChunker},
    response::ApduResponse,
};

pub trait Iso7816Operation<'a> {
    type Result;

    fn build<'b>(&'b mut self, class: Iso7816Class) -> (Iso7816Command<'b>, &'a mut [u8]);
    fn parse(self, response: &ApduResponse<'a>) -> Self::Result;
}

#[derive(Debug, Clone, Copy)]
pub struct Iso7816Command<'a> {
    pub(crate) class: Iso7816Class,
    pub(crate) instruction: u8,
    pub(crate) parameters: (u8, u8),
    pub(crate) data: &'a [u8],
}

impl<'a> Iso7816Command<'a> {
    pub fn chunk(self, max_size: usize) -> CommandChunker<'a> {
        CommandChunker {
            base_command: self,
            max_size,
            offset: 0,
            done: false,
        }
    }
}

impl<'a> ApduCommand for Iso7816Command<'a> {
    type Class = Iso7816Class;

    fn class(&self) -> Iso7816Class {
        self.class
    }

    fn instruction(&self) -> u8 {
        self.instruction
    }

    fn parameters(&self) -> (u8, u8) {
        self.parameters
    }

    fn data(&self) -> &[u8] {
        self.data
    }
}
