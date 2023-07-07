use std::path::PathBuf;

pub fn get_files_by_regex(re: &str) -> Vec<PathBuf> {
    use std::fs;

    use regex::Regex;

    fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(re).unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect()
}
