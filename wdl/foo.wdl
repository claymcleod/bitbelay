version 1.2

task stepA {
    command <<<
        echo "Standard mode"
    >>>

    output {
        Boolean qc_passed = true
    }
}

task stepA_alternate {
    command <<<
        echo "Alternate mode"
    >>>

    output {
        Boolean qc_passed = false
    }
}

task stepB {
    command <<<
        echo "QC passed!"
    >>>

    output {
        String message = read_string(stdout())
    }
}

workflow run {
    input {
        Boolean alternate_execution_mode
    }

    # `alternate_execution_mode` is a required input that must be
    # passed in to the workflow. Based on that value, either...
    if (alternate_execution_mode) {
        # the alternate execution mode will run, or...
        call stepA_alternate {
        }
    }

    if (!alternate_execution_mode) {
        # the standard execution mode will run.
        call stepA {
        }
    }

    # If a QC check in `stepA` passes, a further analysis path
    # starting with `stepB` will be executed.
    if (select_first([
        stepA.qc_passed,
        stepA_alternate.qc_passed,
    ])) {
        call stepB {}
    }

    output {
        String? stepB_message = stepB.message
    }
}
