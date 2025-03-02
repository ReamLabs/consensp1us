use std::fs;
use std::path::PathBuf;

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
