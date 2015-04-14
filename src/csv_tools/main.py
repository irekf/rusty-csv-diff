from collections import defaultdict
from optparse import OptionParser

import csvgen
import sys

# CSV tools
TOOL_GEN     = "gen"
TOOL_SHUFFLE = "shuffle"

def generate_csv(options, args):
    print("generating a CSV file...")
    path = options.out_file_name
    row_num = options.row_num
    col_num = options.col_num
    delim = options.delim
    quote = options.quote
    csvgen.generate_csv(row_num, col_num, path, delim, quote)

def shuffle_csv(options, args):
    print("shuffling a CSV file...")

def unknown_tool(options, args):
    print("unknown tool found, please use either \"gen\" or \"shuffle\"")

def get_default_tool():
    return unknown_tool

tools = defaultdict(get_default_tool, {TOOL_GEN : generate_csv,
                    TOOL_SHUFFLE : shuffle_csv})

def get_opt_parser(tool):

    parser = OptionParser()

    if tool == TOOL_GEN:
        parser.add_option("-o", "--output",
                action="store", type="string", dest="out_file_name",
                help="name of output CSV file")
        parser.add_option("-r", "--row_number",
                action="store", type="int", dest="row_num",
                help="number of rows in output CSV file")
        parser.add_option("-c", "--col_number",
                action="store", type="int", dest="col_num",
                help="number of columns in output CSV file")
        parser.add_option("-d", "--delimiter",
                action="store", type="string", dest="delim",
                help="delimiter used in  output CSV file")
        parser.add_option("-q", "--quote",
                action="store", type="string", dest="quote",
                help="quote used in  output CSV file")
    elif tool == TOOL_SHUFFLE:
        parser.add_option("-i", "--input",
                action="store", type="string", dest="input_file_name",
                help="name of input CSV file")
        parser.add_option("-o", "--output",
                action="store", type="string", dest="out_file_name",
                help="name of output CSV file")
    else:
        pass

    return parser

def parse_args(tool_name, opt_parser, args):
    (options, args) = opt_parser.parse_args(args)
    if tool_name == TOOL_GEN:
        if options.out_file_name is None:
            opt_parser.error("output file name not given")
        if options.row_num is None:
            opt_parser.error("row number not given")
        if options.col_num is None:
            opt_parser.error("col_number not fiven")
    else:
        pass

    return (options, args)

def main():

    tool_name = sys.argv[1]
    args = sys.argv[2:]

    opt_parser = get_opt_parser(tool_name)

    (options, args) = parse_args(tool_name, opt_parser, args)

    tools[tool_name](options, args)

if __name__ == "__main__":
    main()
