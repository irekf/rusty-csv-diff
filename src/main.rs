use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

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

}
