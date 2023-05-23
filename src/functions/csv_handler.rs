use super::song::Song;
use csv::ReaderBuilder;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

pub fn read_from_csv(file_path: &str) {
    // Path to the CSV file
    let csv_file_path = file_path;

    // Path to the folder containing files
    let folder_path = "Songs";

    // Open the CSV file
    let file = match fs::File::open(csv_file_path) {
        Ok(file) => file,
        Err(err) => {
            eprintln!("Failed to open CSV file: {}", err);
            return;
        }
    };

    // Create a CSV reader
    let mut csv_reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    // Read the CSV records and store them in a vector of CsvRow structs
    let mut csv_data: Vec<Song> = Vec::new();
    for result in csv_reader.records() {
        let record = match result {
            Ok(record) => record,
            Err(err) => {
                eprintln!("Failed to read CSV record: {}", err);
                return;
            }
        };
        let row = Song::new(
            record.get(0).unwrap_or("").to_owned(),
            record.get(1).unwrap_or("").to_owned(),
            record.get(2).unwrap_or("").to_owned(),
        );
        csv_data.push(row);
    }

    // Get all the file names in the folder and store them in a HashSet
    let folder_files: HashSet<_> = match fs::read_dir(folder_path) {
        Ok(entries) => entries
            .filter_map(Result::ok)
            .filter_map(|entry| entry.file_name().into_string().ok())
            .collect(),
        Err(err) => {
            eprintln!("Failed to read folder files: {}", err);
            return;
        }
    };

    // Check which names have corresponding files in the folder
    for row in csv_data {
        let file_name = row.get_filename().to_string();
        let file_path = Path::new(&folder_path).join(&file_name);

        if folder_files.contains(&file_name) {
            println!("File exists: {}", file_path.display());
        } else {
            println!("File does not exist: {}", file_path.display());
            let dl_path = row.download();
            row.delete(&dl_path);
        }
    }
}
