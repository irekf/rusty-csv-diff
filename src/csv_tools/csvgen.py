import random
import string

MAX_COL_SIZE = 32

def col_generator(col_num, quote=None):
    if quote == None:
        quote = '\''
    i = 0
    while i < col_num:
        col_val = ''.join(random.choice(string.ascii_uppercase + \
                                        string.ascii_lowercase + \
                                        string.digits +
                                        ' ') for _ in range(random.randint(1, MAX_COL_SIZE)))
        yield quote + col_val + quote
        i += 1


def row_generator(row_num, col_num, delim=None, quote=None):
    if delim == None:
        delim = ','
    i = 0
    while i < row_num:
        yield delim.join(col for col in col_generator(col_num, quote))
        i += 1

def generate_csv(row_num, col_num, path, delim=',', quote='\''):
    with open(path, 'w+') as f:
        for row in row_generator(row_num, col_num, delim, quote):
            f.write(row + '\n')
