use std::path::Path;
mod file_reader;
mod day1;
fn main() {
    let path = Path::new("./src/inputs/day1.txt");
    let contents = file_reader::vector_of_lines(path);

    println!("Calibration number: {}", day1::get_calibration_value(contents));

}
