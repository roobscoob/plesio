use crate::apdu::iso_7816::operation::Iso7816Command;

pub struct CommandChunker<'a> {
    pub(super) base_command: Iso7816Command<'a>,
    pub(super) max_size: usize,
    pub(super) offset: usize,
    pub(super) done: bool,
}

impl<'a> Iterator for CommandChunker<'a> {
    type Item = Iso7816Command<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let total_len = self.base_command.data.len();

        // Edge case: If data is empty, we still want to send one command
        // containing 0 bytes, then finish.
        if total_len == 0 {
            self.done = true;
            return Some(self.base_command);
        }

        let remaining = total_len - self.offset;
        let chunk_size = core::cmp::min(remaining, self.max_size);

        // Calculate slices
        let start = self.offset;
        let end = self.offset + chunk_size;
        let data_slice = &self.base_command.data[start..end];

        // Advance offset
        self.offset += chunk_size;

        // Check if this is the last chunk
        let is_last = self.offset >= total_len;

        if is_last {
            self.done = true;
            // The last chunk uses the ORIGINAL class (chaining bit cleared/as-is)
            Some(Iso7816Command {
                data: data_slice,
                ..self.base_command
            })
        } else {
            // Intermediate chunks get the chaining bit set
            Some(Iso7816Command {
                class: self.base_command.class.with_chaining(),
                data: data_slice,
                ..self.base_command
            })
        }
    }
}
