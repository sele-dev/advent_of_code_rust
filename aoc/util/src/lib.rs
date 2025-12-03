use std::path::Path;
use dirs::cache_dir;

pub fn get_inputs_filepath(filename: &str) -> String {
    // TODO handle missing dir as separate util to set up local caching
    let cache_dir = match cache_dir() {
        Some(pathbuf) => pathbuf,
        None => { println!("No cache dir found!"); Path::new("./").to_path_buf() },
    };

    // TODO temporary required here? handle year?
    let full_pathbuf = cache_dir.join("aoc/inputs/2025/").join(filename);
    let filepath = full_pathbuf.to_str().unwrap();
    return filepath.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_root_found() {
        let result = get_workspace_root();
        // TODO
        assert_eq!(*result.as_path(), *Path::new(""));
    }
}
