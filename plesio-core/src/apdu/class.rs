pub trait ApduClass {
    fn to_u8(&self) -> u8;
}

pub enum SecureMessaging {
    None,
    /// Authenticated but not encrypted
    Authenticated,
    /// Both authenticated and encrypted (privacy)
    Encrypted,
    /// Proprietary/Legacy format (Bit 5 is set in a non-standard way)
    Proprietary,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApduChannel {
    Basic(u8),
    Extended(u8),
}

impl Default for ApduChannel {
    fn default() -> Self {
        ApduChannel::Basic(0)
    }
}

pub enum Iso7816Class {
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

impl Iso7816Class {
    pub fn from_u8(class: u8) -> Option<Self> {
        if class >= 0x80 {
            return None;
        }

        let is_extended_range = (class & 0x40) != 0;
        let chaining = (class & 0x10) != 0;

        if is_extended_range {
            Some(Self::Extended {
                chaining,
                is_secure_messaging: (class & 0x20) != 0,
                extended_channel: (class & 0x0F) + 4,
            })
        } else {
            let secure_messaging = match (class & 0x0C) >> 2 {
                0 => SecureMessaging::None,
                1 => SecureMessaging::Proprietary,
                2 => SecureMessaging::Authenticated,
                3 => SecureMessaging::Encrypted,

                _ => unreachable!(),
            };

            Some(Self::Basic {
                chaining,
                secure_messaging,
                basic_channel: class & 0x03,
            })
        }
    }
}

impl ApduClass for Iso7816Class {
    fn to_u8(&self) -> u8 {
        match self {
            Self::Basic {
                chaining,
                secure_messaging,
                basic_channel,
            } => {
                let chaining_bit = if *chaining { 0x10 } else { 0 };
                let secure_messaging_bits = match *secure_messaging {
                    SecureMessaging::None => 0,
                    SecureMessaging::Proprietary => 1,
                    SecureMessaging::Authenticated => 2,
                    SecureMessaging::Encrypted => 3,
                } << 2;
                let basic_channel_bits = *basic_channel & 0x03;
                chaining_bit | secure_messaging_bits | basic_channel_bits
            }
            Self::Extended {
                chaining,
                is_secure_messaging,
                extended_channel,
            } => {
                let chaining_bit = if *chaining { 0x10 } else { 0 };
                let is_secure_messaging_bit = if *is_secure_messaging { 0x20 } else { 0 };
                let extended_channel_bits = (*extended_channel - 4) & 0x0F;
                chaining_bit | is_secure_messaging_bit | extended_channel_bits
            }
        }
    }
}
