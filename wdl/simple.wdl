version 1.2

task align {
    input {
        File bam
    }

    command <<<
        STAR --readFilesIn \ # FastQ files, separated by comma if there are multiple. The order of your R1 and R2 files has to match!
        --outSAMattrRGline $ALL_RG \                   # Read group lines in the same order as `readFilesIn` (derived from earlier `samtools split` step).
        --genomeDir $STARDB \                          # Directory containing the STAR genome
        --runThreadN $NCPU \
    >>>
}

workflow run {
    input {
        File vcf
    }

    call split_vcf { input: vcf }

    call filter_snps { input: vcf = split_vcf.snp_vcf }
    call filter_indels { input: vcf = split_vcf.indel_vcf }

    # (a) The outputs are connected to the inputs here.
    call merge_vcfs { input:
        snp_vcf = filter_snps.filtered_vcf,
        indel_vcf = filter_snps.filtered_vcf,
    }

    output {
        File combined = merge_vcfs.combined
    }
}
