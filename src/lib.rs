use log::info;

pub fn read_file(filename: &str) -> String {
    info!("Reading file {}", filename);
    fs_err::read_to_string(filename).unwrap()
}

pub fn read_data(day: u32) -> String {
    read_file(&format!("data/day{}.txt", day))
}

pub fn read_test_data(day: u32) -> String {
    read_file(&format!("data/day{}.test.txt", day))
}
