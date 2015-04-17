from collections import defaultdict

import argparse
import csvgen
import shuffle
import sys

# CSV tools
TOOL_GEN     = "gen"
TOOL_SHUFFLE = "shuffle"

def generate_csv(args):
    print("generating a CSV file...")
    path = args.output_file
    row_num = args.row_num
    col_num = args.col_num
    delim = args.delim
    quote = args.quote
    csvgen.generate_csv(row_num, col_num, path, delim, quote)

def shuffle_csv(args):
    print("shuffling a CSV file...")
    input_path = args.input_file
    output_path = args.output_file
    shuffle.shuffle_csv(input_path, output_path)

def unknown_tool(args):
    print("unknown tool found, please use either \"gen\" or \"shuffle\"")

def get_default_tool():
    return unknown_tool

tools = defaultdict(get_default_tool, {TOOL_GEN : generate_csv,
                    TOOL_SHUFFLE : shuffle_csv})

def get_arg_parser(tool):

    parser = argparse.ArgumentParser()

    if tool == TOOL_GEN:
        parser.add_argument("gen", type=str,
                            help="CSV generate tool")
        parser.add_argument("row_num", type=int, help="number of rows in CSV")
        parser.add_argument("col_num", type=int, help="number of columns in CSV")
        parser.add_argument("output_file", type=str, help="name of output file")
        parser.add_argument("-d", "--delimiter",
                action="store", type=str, dest="delim", default=',',
                help="delimiter used in  output CSV file")
        parser.add_argument("-q", "--quote",
                action="store", type=str, dest="quote", default='\'',
                help="quote used in  output CSV file")
    elif tool == TOOL_SHUFFLE:
        parser.add_argument("shuffle", type=str,
                            help="shuffle tool")
        parser.add_argument("input_file", type=str, help="name of input CSV file")
        parser.add_argument("output_file", type=str, help="name of output CSV file")
    else:
        parser.add_argument("tool", type=str, choices=["gen", "shuffle"],
                            help="CSV tool")

    return parser

def main():

    try:
        tool_name = sys.argv[1]
    except IndexError:
        tool_name = None

    arg_parser = get_arg_parser(tool_name)

    args = arg_parser.parse_args()

    tools[tool_name](args)

if __name__ == "__main__":
    main()
