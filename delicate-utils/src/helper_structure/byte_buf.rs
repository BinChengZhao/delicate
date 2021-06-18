pub struct ByteBuf<'a>(pub &'a [u8]);

impl<'a> std::fmt::LowerHex for ByteBuf<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        for byte in self.0 {
            fmt.write_fmt(format_args!("{:02x}", byte))?;
        }
        Ok(())
    }
}
