use std::path::Path;

use snap::raw::Decoder;

pub fn read_ssz_snappy<T: ssz::Decode>(path: &Path) -> Option<T> {
    let ssz_snappy = std::fs::read(path).ok()?;
    let mut decoder = Decoder::new();
    let ssz = decoder.decompress_vec(&ssz_snappy).unwrap();
    T::from_ssz_bytes(&ssz).ok()
}
