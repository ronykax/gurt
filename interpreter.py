def interpret(code: str):
    code = code.replace(" is ", " = ")
    code = code.replace("bet ", "def ")
    code = code.replace("yap", "print")
    code = code.replace("f_around:", "try:")
    code = code.replace("find_out:", "except:")
    code = code.replace("at_last:", "finally:")
    code = code.replace("lockin ", "for ")
    code = code.replace("sus ", "if ")
    code = code.replace("ong ", "while ")
    code = code.replace("get_out", "break")
    code = code.replace("yo ", "")

    exec(code)