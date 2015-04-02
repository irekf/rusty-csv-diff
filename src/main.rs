use std::collections::{HashSet, HashMap};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn print_arg_error() {
    println!("incorrect arguments passed");
}

struct CsvDescriptor<'a> {
    file_path: &'a  Path,
    delimiter:      char,
    quote:          Option<char>,
}

impl<'a> std::fmt::Display for CsvDescriptor<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} {} {:?}", self.file_path.display(), self.delimiter, self.quote)
    }
}

fn parse_args<'a>(path_arg:         &'a String,
                  delimiter_arg:    &'a String,
                  quote_arg:        &'a String) -> Result<CsvDescriptor<'a>, &'static str>
{

    let csv_file_path = Path::new(path_arg);

    let csv_delimiter = match delimiter_arg.chars().next() {
                            Some(result) => result,
                            None         => return Err("incorrect delimiter"),
                        };

    let csv_quote     = quote_arg.chars().next();

    Ok(CsvDescriptor {file_path: &csv_file_path, delimiter: csv_delimiter, quote: csv_quote})
}

fn get_csv_cols(csv_desc: CsvDescriptor) -> Result<Vec<String>, String> {

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

    let csv_desc_1: CsvDescriptor = match parse_args(&args[1], &args[2], &args[3]) {
        Err(why)    => panic!("error parsing arguments for CSV #1: {}", why),
        Ok(result)  => result,
    };

    let csv_desc_2: CsvDescriptor = match parse_args(&args[4], &args[5], &args[6]) {
        Err(why)    => panic!("error parsing arguments for CSV #2: {}", why),
        Ok(result)  => result,
    };

    /*** 2&3 ***/
    let csv_cols_1: Vec<String> = match get_csv_cols(csv_desc_1) {
        Err(why) => panic!("couldn't get columns: {}", why),
        Ok(cols) => cols,
    };

    let csv_cols_2: Vec<String> = match get_csv_cols(csv_desc_2) {
        Err(why) => panic!("couldn't get columns: {}", why),
        Ok(cols) => cols,
    };

    /*** 5 ***/
    let mut csv_col_index_1 = HashMap::new();
    for i in 0..csv_cols_1.len() {
        let key = csv_cols_1[i].clone();
        if csv_col_index_1.contains_key(&key) {
            panic!("duplicate column found in CSV #1: {}", key);
        };
        csv_col_index_1.insert(key, i);
    }

    let mut csv_col_index_2 = HashMap::new();
    for i in 0..csv_cols_2.len() {
        let key = csv_cols_2[i].clone();
        if csv_col_index_2.contains_key(&key) {
            panic!("duplicate column found in CSV #1: {}", key);
        };
        csv_col_index_2.insert(key, i);
    }

    /*** 4 ***/
    let mut cols_to_compare = HashSet::new();
    for col_1 in csv_col_index_1.keys() {
        if csv_col_index_2.contains_key(col_1) {
            cols_to_compare.insert(col_1);
        };
    }
    println!("{:?}", cols_to_compare);
}
