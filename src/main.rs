mod functions;

use functions::csv_handler::read_from_csv;

fn main() {
    read_from_csv("Songs.csv")
}
