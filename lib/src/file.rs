use std::fs;
use std::path::{Path, PathBuf};

use crate::snappy::decode_snappy;
use crate::ssz::to_ssz;

pub fn read_file<T: ssz::Decode>(path: &Path) -> T {
    let raw_bytes =
        std::fs::read(path).unwrap_or_else(|e| panic!("Could not read file: {:?}: {}", path, e));
    let ssz_bytes = decode_snappy(&raw_bytes).unwrap_or_else(|e| {
        panic!("Could not decode snappy {:?}: {}", path, e);
    });
    to_ssz(&ssz_bytes).unwrap_or_else(|| {
        panic!("Could not decode ssz {:?}", path);
    })
}

pub fn get_test_cases(base_dir: &PathBuf) -> Vec<String> {
    let mut test_cases = Vec::new();

    if let Ok(entries) = fs::read_dir(base_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                if let Some(folder_name) = entry.file_name().to_str() {
                    test_cases.push(folder_name.to_string());
                }
            }
        }
    }

    test_cases
}
