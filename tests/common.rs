use glob::glob;

#[allow(unused)]
pub fn cleanup(pattern: &str) {
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        let path = entry.expect("Failed to get path");
        if path.is_file() && path.extension().map_or(false, |ext| ext == "out") {
            let _ = std::fs::remove_file(path);
        }
    }
}
