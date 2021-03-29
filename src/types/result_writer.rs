use crate::binary::Encoder;
use crate::types::Block;
use crate::errors::Result;

pub struct ResultWriter<'a> {
    encoder: &'a mut Encoder,
    compress: bool,
}

impl<'a> ResultWriter<'a> {
    pub(crate) fn new(encoder : &'a mut Encoder, compress: bool) -> Self {
        Self {
            encoder,
            compress,
        }
    }

    pub fn write_block(&mut self, block: Block) -> Result<()> {
        block.send_server_data(&mut self.encoder, self.compress);
        Ok(())
    }

    pub fn finalize(&mut self) -> Result<()> {
        Block::new().send_server_data(&mut self.encoder, self.compress);
        Ok(())
    }
}