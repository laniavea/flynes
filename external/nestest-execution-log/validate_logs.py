PC = [0, 4]
FETCHED_BYTES = [6, 8] # to 13 inc
OP_ASSEMBLY = [15, 32] # to 46 inc
REG_A = [48, 4] # to 51 inc
REG_X = [53, 4] # to 56 inc
REG_Y = [58, 4] # to 61 inc
CPU_STATUS = [63, 4] # to 66 inc
STACK_POINTER = [68, 5] # to 72 inc
PPU = [74, 11] # to 84 inc
CYCLES = [86, 9] # to 94 inc

def main():
    ideal_log_path = "./nestest_full.log"
    check_log_path, columns_to_check = get_by_version(1)

    with open(ideal_log_path) as il:
        ideal_columns = take_columns_ideal(columns_to_check, il.read())

    with open(check_log_path) as cl:
        check_columns = take_columns_check(cl.read())

    assert len(ideal_columns[0]) == len(check_columns[0]), "Number of columns is different"

    row_counter = 1
    fails = 0
    tolerance = 10

    for (idead_row, check_row) in zip(ideal_columns, check_columns):
        for now_column in range(len(columns_to_check)):
            if idead_row[now_column] != check_row[now_column]:
                fails += 1
                print("=" * 20)
                print(f"ROW {row_counter} FAIL")
                print(f"VALUE MISMATCH: expected - '{idead_row[now_column]}', got - '{check_row[now_column]}'")
                print(f"\nIdeal log str: {idead_row[-1]}")
                print(f"Got log str: {check_row[-1]}")
                print("=" * 20)
                print()

        row_counter += 1
        if fails > tolerance:
            break

    if fails:
        print(f"LOG VALIDATION FAILED for {fails} rows")
    else:
        if len(check_columns) < len(ideal_columns):
            print(f"LOG PARTIALLY VALIDATED. SCANNED {row_counter} rows from {len(ideal_columns)}")
        else:
            print(f"LOG VALIDATED. SCANNED {row_counter} rows")


def get_by_version(version):
    if version == 1:
        columns_to_check = [
            PC,
            FETCHED_BYTES,
            REG_A,
            REG_X,
            REG_Y,
            CPU_STATUS,
            STACK_POINTER
        ]
        return "./nestest_v1.log", columns_to_check
    else:
        raise ValueError("Unknown version")

def take_columns_ideal(cols_to_take, ideal_log):
    parsed_log = []
    for now_str in ideal_log.split("\n"):
        parsed_str = []
        for now_col in cols_to_take:
            parsed_str.append(now_str[now_col[0]:now_col[0]+now_col[1]].strip())
        parsed_str.append(now_str)
        parsed_log.append(parsed_str)

    return parsed_log

def take_columns_check(check_log):
    parsed_log = []
    for now_str in check_log.split("\n"):
        parsed_log.append(list(map(lambda x: x.strip(), now_str.strip().split("\t"))))
        parsed_log[-1].append(now_str.strip())

    return parsed_log

if __name__ == "__main__":
    main()

