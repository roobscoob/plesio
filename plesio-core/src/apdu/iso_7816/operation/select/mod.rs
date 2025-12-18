pub mod resolution;

use crate::apdu::{
    iso_7816::{
        class::Iso7816Class,
        operation::{
            Iso7816Command, Iso7816Operation, select::resolution::Iso7816SelectResolution,
        },
        status,
        tlv::iter::TlvIterator,
    },
    response::ApduResponse,
    status::is,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SelectOccurrence {
    #[default]
    First,
    Last,
    Next,
    Previous,
}

pub enum FileControlFlag {
    None,
    WithFileControlInformation,
    WithFileControlParameters,
    WithFileManagementData,
}

pub struct Iso7816Select<'aid, 'res> {
    resolution: Iso7816SelectResolution<'aid>,
    file_control_flag: FileControlFlag,
    occurrence: SelectOccurrence,
    response: Option<&'res mut [u8]>,
}

impl<'aid, 'res> Iso7816Select<'aid, 'res> {
    pub fn new(resolution: Iso7816SelectResolution<'aid>, response: &'res mut [u8]) -> Self {
        Self {
            resolution,
            file_control_flag: FileControlFlag::None,
            occurrence: SelectOccurrence::First,
            response: Some(response),
        }
    }

    pub fn with_file_control_flag(mut self, file_control_flag: FileControlFlag) -> Self {
        self.file_control_flag = file_control_flag;
        self
    }

    pub fn with_occurrence(mut self, occurrence: SelectOccurrence) -> Self {
        self.occurrence = occurrence;
        self
    }
}

impl<'aid, 'res> Iso7816Operation<'res> for Iso7816Select<'aid, 'res> {
    type Result = Result<TlvIterator<'res>, ApduResponse<'res>>;

    fn build<'b>(&'b mut self, class: Iso7816Class) -> (Iso7816Command<'b>, &'res mut [u8]) {
        let occurrence = match self.occurrence {
            SelectOccurrence::First => 0b00,
            SelectOccurrence::Last => 0b01,
            SelectOccurrence::Next => 0b10,
            SelectOccurrence::Previous => 0b11,
        };

        let file_control_flag = match self.file_control_flag {
            FileControlFlag::None => 0b0000,
            FileControlFlag::WithFileControlInformation => 0b0100,
            FileControlFlag::WithFileControlParameters => 0b1000,
            FileControlFlag::WithFileManagementData => 0b1100,
        };

        let command = Iso7816Command::<'b> {
            class,
            instruction: 0xA4,
            parameters: (
                self.resolution.parameter_1(),
                file_control_flag | occurrence,
            ),
            data: self.resolution.data(),
        };

        (command, self.response.take().unwrap_or(&mut []))
    }

    fn parse(self, response: &ApduResponse<'res>) -> Self::Result {
        response.expect_status(is(status::OK)).map(TlvIterator::new)
    }
}
