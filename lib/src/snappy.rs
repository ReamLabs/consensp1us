use snap::raw::Decoder;

pub fn decode_snappy(raw_bytes: &[u8]) -> Result<Vec<u8>, snap::Error> {
    let mut decoder = Decoder::new();
    decoder.decompress_vec(raw_bytes)
}
