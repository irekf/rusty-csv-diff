from collections import defaultdict
from optparse import OptionParser

import sys

# CSV tools
TOOL_GEN     = "gen"
TOOL_SHUFFLE = "shuffle"

def generate_csv(options, args):
    print("generating a CSV file...")

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

def main():

    tool_name = sys.argv[1]
    args = sys.argv[2:]

    opt_parser = get_opt_parser(tool_name)

    (options, args) = opt_parser.parse_args(args)

    tools[tool_name](options, args)

if __name__ == "__main__":
    main()
