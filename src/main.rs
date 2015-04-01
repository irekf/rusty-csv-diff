use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn print_arg_error() {
    println!("incorrect arguments passed");
}

struct CSV_descriptor<'a> {
    file_path: &'a  Path,
    delimiter:      char,
    quote:          Option<char>,
}

impl<'a> std::fmt::Display for CSV_descriptor<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {:?}", self.file_path.display(), self.delimiter, self.quote)
    }
}

fn parse_args<'a>(path_arg:         &'a String,
                  delimiter_arg:    &'a String,
                  quote_arg:        &'a String) -> Result<CSV_descriptor<'a>, &'static str>
{

    let csv_file_path = Path::new(path_arg);

    let csv_delimiter = match delimiter_arg.chars().next() {
                            Some(result) => result,
                            None         => return Err("incorrect delimiter"),
                        };

    let csv_quote     = quote_arg.chars().next();

    Ok(CSV_descriptor {file_path: &csv_file_path, delimiter: csv_delimiter, quote: csv_quote})
}

fn get_csv_cols(csv_desc: CSV_descriptor) -> Result<Vec<String>, String> {

    let csv_file = match File::open(csv_desc.file_path) {
        Err(why) => panic!("couldn't open csv @ {}: {}", csv_desc.file_path.display(), why),
        Ok(file) => file,
    };

    let csv_reader = BufReader::new(csv_file);

    let mut csv_line_iter = csv_reader.lines();

    let csv_header: String = match csv_line_iter.next() {
        Some(result) => match result {
                            Err(why) => return Err(format!("error getting csv header: {}", why)),
                            Ok(header) => header,
                        },
        None         => return Err("csv header reading failed".to_string()),
    };

    let csv_cols: Vec<String> = {
        let cols_iter = csv_header.split(csv_desc.delimiter);
        match csv_desc.quote {
            Some(q) => cols_iter.map(|s| {s.trim_matches(q).to_string()}).collect(),
            None    => cols_iter.map(|s| {s.to_string()}).collect(),
        }
    };

    Ok(csv_cols)
}

fn main() {

/*

1. Parse arguments
2. Open CSV files
3. Get columns (cols_N)
4. Get intersection of those two sets of columns(cols_to_compare)
5. Create {column name : column index in cols_N} dicts
6. Create {CSV_col_value : CSV row index in file} dicts, where CSV_col_value is a unique key made of the value of several CSV columns.
   For example, {Alex38 : 76}. In this example a name and age form a unique key for the 76th CSV row.
7. Get intersection of key sets of dicts from step 6 (row_keys_to_compare)
8. Loop through row_keys_to_compare, use dicts from step 6 to get line numbers for CSV files
    8.1 Loop through cols_to_compare, use dicts from step 5 to extract column values from CSV rows(step 8)
    8.2 Compare values

Input parameters: CSV paths, delimiters, quotes

For example, ./main file_1.csv "," "'" file_2.csv " " "" 

*/

    /*** 1 ***/
    let args: Vec<String> = env::args().collect();
    if args.len() != 7 {
        print_arg_error();
        return;
    };

    let csv_desc: CSV_descriptor = match parse_args(&args[1], &args[2], &args[3]) {
        Err(why)    => panic!("error parsing arguments: {}", why),
        Ok(result)  => result,
    };
    println!("{}", csv_desc);

    let csv_file = match File::open(csv_desc.file_path) {
        Err(why) => panic!("couldn't open csv @ {}: {}", csv_desc.file_path.display(), why),
        Ok(file) => file,
    };

    /*** 2 ***/
    //let csv_cols: Vec<String> = get_csv_cols(csv_desc);

}
