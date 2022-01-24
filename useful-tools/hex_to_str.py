# expected command: "python hex_to_str.py 00  61  73  6d  01  00  00  00"
# previous command's output is "asm"

import binascii
import sys

def hex_to_str(string):
    return binascii.a2b_hex(string)

args = sys.argv

str_args = ""

for i in args[1:]:
    str_args += str(i)

print(hex_to_str(str_args))
