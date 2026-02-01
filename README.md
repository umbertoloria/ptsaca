# PTSACA

Here are the commands this tool provides.

Generate a FASTA file with a certain number of pair bases.
> ptsaca gen-ff <number-of-pb> <output-fasta-file>

Run PTSACA on a given input file using a specified Chunk Size.
> ptsaca run <input-fasta-file> <chunk-size> <suffix_array_output_file>

Run PTSACA debug program (for development).
> ptsaca run-program

Note: you can run the program using "cargo run" during development.

Here are some examples of command one can run.

> cargo run gen-ff 700000 generated/output.fasta
> cargo run run generated/002_70.fasta 5 sa_output.txt
> cargo run run-program # Debug
