#!/bin/bash

OPERATION=$1

LOG_FILE="logs/execution_$OPERATION.log"
OUTPUT_FILE="summaries/summary_$OPERATION.md"

# Table Header
echo '| Operation | Test Case | Total Cycles | Syscall Count | Read Pre-State | Read Operation | Process | Convert SSZ | Commit |' > $OUTPUT_FILE
echo '|-----------|-----------|--------------|--------------|----------------|----------------|---------|-------------|--------|' >> $OUTPUT_FILE

awk '
BEGIN {
    op = "";
    test_case = "";
    total_cycles = 0;
    syscall_count = 0;
    read_pre_state = 0;
    read_operation = 0;
    process_op = 0;
    convert_ssz = 0;
    commit = 0;
}

/INFO \[.*\] Test case:/ {
    op = $3; 
    gsub(/[\[\]]/, "", op)
    test_case = $NF;
}

/INFO Number of cycles:/ {
    total_cycles = $NF;
}

/INFO Number of syscall count:/ {
    syscall_count = $NF;
}

/INFO read-pre-state:/ {
    read_pre_state = $NF;
}

/INFO read-operation-input:/ {
    read_operation = $NF;
}

/INFO process-operation:/ {
    process_op = $NF;
}

/INFO convert-to-ssz-bytes:/ {
    convert_ssz = $NF;
}

/INFO commit:/ {
    commit = $NF;
}

/INFO ----- Cycle Tracker End -----/ {
    printf "%s | %s | %d | %d | %d | %d | %d | %d | %d\n", op, test_case, total_cycles, syscall_count, read_pre_state, read_operation, process_op, convert_ssz, commit >> "'$OUTPUT_FILE'"

    # Initialize
    op = "";
    test_case = "";
    total_cycles = 0;
    syscall_count = 0;
    read_pre_state = 0;
    read_operation = 0;
    process_op = 0;
    convert_ssz = 0;
    commit = 0;
}
' $LOG_FILE
