use crate::apdu::class::ApduClass;

#[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
pub enum SecureMessaging {
    #[default]
    None,
    /// ISO 7816-4: SM used, Command header not authenticated (Value 10b)
    Authenticated,
    /// ISO 7816-4: SM used, Command header authenticated (Value 11b)
    HeaderAuthenticated,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Iso7816ClassState {
    Basic {
        chaining: bool,
        secure_messaging: SecureMessaging,
        basic_channel: u8,
    },
    Extended {
        chaining: bool,
        is_secure_messaging: bool,
        extended_channel: u8,
    },
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Iso7816Class {
    state: Iso7816ClassState,
}

impl Default for Iso7816Class {
    fn default() -> Self {
        Self {
            state: Iso7816ClassState::Basic {
                chaining: false,
                secure_messaging: SecureMessaging::None,
                basic_channel: 0,
            },
        }
    }
}

impl Iso7816Class {
    pub fn from_u8(class: u8) -> Option<Self> {
        if class >= 0x80 {
            return None;
        }

        let is_extended_range = (class & 0x40) != 0;
        let chaining = (class & 0x10) != 0;

        if is_extended_range {
            Some(Self {
                state: Iso7816ClassState::Extended {
                    chaining,
                    is_secure_messaging: (class & 0x20) != 0,
                    extended_channel: (class & 0x0F) + 4,
                },
            })
        } else {
            if (class & 0x20) != 0 {
                return None;
            }

            let secure_messaging = match (class & 0x0C) >> 2 {
                0 => SecureMessaging::None,
                1 => return None,
                2 => SecureMessaging::Authenticated,
                3 => SecureMessaging::HeaderAuthenticated,

                _ => unreachable!(),
            };

            Some(Self {
                state: Iso7816ClassState::Basic {
                    chaining,
                    secure_messaging,
                    basic_channel: class & 0x03,
                },
            })
        }
    }
}

impl ApduClass for Iso7816Class {
    fn to_u8(&self) -> u8 {
        match &self.state {
            Iso7816ClassState::Basic {
                chaining,
                secure_messaging,
                basic_channel,
            } => {
                let chaining_bit = if *chaining { 0x10 } else { 0 };
                let secure_messaging_bits = match *secure_messaging {
                    SecureMessaging::None => 0,
                    SecureMessaging::Authenticated => 2,
                    SecureMessaging::HeaderAuthenticated => 3,
                } << 2;
                let basic_channel_bits = *basic_channel & 0x03;
                chaining_bit | secure_messaging_bits | basic_channel_bits
            }
            Iso7816ClassState::Extended {
                chaining,
                is_secure_messaging,
                extended_channel,
            } => {
                let is_extended_range_bit = 0x40;
                let chaining_bit = if *chaining { 0x10 } else { 0 };
                let is_secure_messaging_bit = if *is_secure_messaging { 0x20 } else { 0 };
                let extended_channel_bits = (*extended_channel - 4) & 0x0F;

                chaining_bit
                    | is_secure_messaging_bit
                    | extended_channel_bits
                    | is_extended_range_bit
            }
        }
    }
}
