use super::constants;

pub fn parse(filenames: Option<clap::Values>) -> Vec<&str> {
    match filenames {
        None => vec![constants::STDIN],
        Some(filenames) => {
            let filenames: Vec<&str> = filenames.collect();
            if filenames.contains(&constants::STDIN) {
                vec![constants::STDIN]
            } else {
                filenames
            }
        }
    }
}
