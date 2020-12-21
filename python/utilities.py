def read_input(file_name):
    with open(file_name) as f:
        data = f.read()
    return data


def read_lines(file_name):
    return list(read_input(file_name).splitlines())


def read_int_input(file_name):
    data = read_lines(file_name)
    return list(map(int, data))


def read_blocks(file_name):
    return read_input(file_name).split('\n\n')
