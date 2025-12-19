use crate::apdu::status::ApduStatus;

pub const OK: ApduStatus = ApduStatus::new(0x90, 0x00);

pub fn has_more_data(status: &ApduStatus) -> Option<usize> {
    (status.as_u16() & 0xFF00 >> 8 == 0x61).then(|| {
        let size = status.as_u16() & 0x00FF;

        if size == 0 {
            u8::MAX as usize + 1
        } else {
            size as usize
        }
    })
}

pub fn has_wrong_length(status: &ApduStatus) -> Option<usize> {
    (status.as_u16() & 0xFF00 >> 8 == 0x6C).then(|| {
        let size = status.as_u16() & 0x00FF;

        if size == 0 {
            u8::MAX as usize + 1
        } else {
            size as usize
        }
    })
}
