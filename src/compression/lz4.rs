use crate::protocol::buf::{ByteBuf, ByteBufMut};
use crate::protocol::{DecodeError, EncodeError};
use bytes::{Buf, BufMut, Bytes, BytesMut};
use std::io;

use lz4::{Decoder, EncoderBuilder};

use super::{compression_err, decompression_err, Compressor, Decompressor};

/// Gzip compression algorithm. See [Kafka's broker configuration](https://kafka.apache.org/documentation/#brokerconfigs_compression.type)
/// for more information.
pub struct Lz4;

const COMPRESSION_LEVEL: u32 = 4;

impl<B: ByteBufMut> Compressor<B> for Lz4 {
    type BufMut = BytesMut;
    fn compress<R, F>(buf: &mut B, f: F) -> Result<R, EncodeError>
    where
        F: FnOnce(&mut Self::BufMut) -> Result<R, EncodeError>,
    {
        // Write uncompressed bytes into a temporary buffer
        let mut tmp = BytesMut::new();
        let res = f(&mut tmp)?;

        let mut encoder = EncoderBuilder::new()
            .level(COMPRESSION_LEVEL)
            .build(buf.writer())
            .map_err(compression_err)?;

        io::copy(&mut tmp.reader(), &mut encoder).map_err(compression_err)?;
        let (_, result) = encoder.finish();
        result.map_err(compression_err)?;

        Ok(res)
    }
}

impl<B: ByteBuf> Decompressor<B> for Lz4 {
    type Buf = Bytes;
    fn decompress<R, F>(buf: &mut B, f: F) -> Result<R, DecodeError>
    where
        F: FnOnce(&mut Self::Buf) -> Result<R, DecodeError>,
    {
        let mut tmp = BytesMut::new().writer();

        // Allocate a temporary buffer to hold the uncompressed bytes
        let buf = buf.copy_to_bytes(buf.remaining());

        let mut decoder = Decoder::new(buf.reader()).map_err(decompression_err)?;
        io::copy(&mut decoder, &mut tmp).map_err(decompression_err)?;

        f(&mut tmp.into_inner().into())
    }
}

#[cfg(test)]
mod test {
    use crate::compression::{compression_err, decompression_err, Lz4};
    use crate::compression::{Compressor, Decompressor};
    use crate::protocol::{DecodeError, EncodeError};
    use bytes::BytesMut;
    use std::fmt::Write;
    use std::str;
    use zstd::zstd_safe::WriteBuf;

    #[test]
    fn test_lz4() {
        let mut compressed = BytesMut::new();
        let _ = Lz4::compress(&mut compressed, |buf| -> Result<(), EncodeError> {
            buf.write_str("hello lz4").map_err(compression_err)?;
            Ok(())
        })
        .unwrap();

        let _ = Lz4::decompress(&mut compressed, |buf| -> Result<(), DecodeError> {
            let decompressed_str = str::from_utf8(buf.as_slice()).map_err(decompression_err)?;
            assert_eq!(decompressed_str, "hello lz4");
            Ok(())
        })
        .unwrap();
    }
}
