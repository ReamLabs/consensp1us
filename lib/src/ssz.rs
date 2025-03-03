pub fn to_ssz<T: ssz::Decode>(ssz_bytes: &[u8]) -> Option<T> {
    T::from_ssz_bytes(ssz_bytes).ok()
}
