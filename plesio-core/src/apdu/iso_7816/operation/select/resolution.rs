pub enum Iso7816SelectResolution<'a> {
    /// Select by DF Name (AID).
    /// Typically used to select a specific Applet (e.g., PIV, OpenPGP).
    ByApplicationIdentifier(&'a [u8]),

    /// Select by File ID (2 bytes).
    /// Used to traverse the file system (e.g., Select Master File).
    ByFileId([u8; 2]),
}

impl<'a> Iso7816SelectResolution<'a> {
    pub fn parameter_1(&self) -> u8 {
        match self {
            Iso7816SelectResolution::ByApplicationIdentifier { .. } => 0x04,
            Iso7816SelectResolution::ByFileId { .. } => 0x00,
        }
    }

    pub fn data(&self) -> &[u8] {
        match self {
            Iso7816SelectResolution::ByApplicationIdentifier(data) => *data,
            Iso7816SelectResolution::ByFileId(data) => data,
        }
    }
}
