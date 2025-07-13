import sys
from interpreter import interpret

if len(sys.argv) < 2:
    print("yo where's the .gurt file?")
    sys.exit(1)

with open(sys.argv[1], "r") as f:
    gurt = f.read()

interpret(gurt)
