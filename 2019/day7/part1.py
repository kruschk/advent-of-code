# Part 1
import itertools, subprocess

INPUT_FILENAME = "input/day7-part1-input.txt"
seq_sig = [] # Stores the pairs of permutations and signals.
# Iterate over every permutation of "01234".
for perm in itertools.permutations("01234"):
    signal = '0' # The input/output signal for/from each amplifier.
    # Run each amplifier with a phase setting dictated by the permutation.
    for phase_setting in perm:
        # Generate the input for the amplifier.
        computer_input = '\n'.join((phase_setting, signal))
        # Run the amplifier (a process).
        proc = subprocess.run(
            ("./intcode", INPUT_FILENAME),\
            encoding='utf-8',\
            input=computer_input,\
            stdout=subprocess.PIPE)
        # Get the output from the amplifier, which becomes part of the input
        # for the next one.
        signal = proc.stdout
    # Add this permutation and the final signal to the list.
    seq_sig.append(('-'.join(perm), int(signal)))
# Get the phase sequence that results in the maximum thrust signal.
(phase_sequence, thrust_signal) = max(seq_sig, key=lambda x: x[1])
# Print the results.
print(f"Phase sequence with maximum thrust signal: {phase_sequence}.")
print(f"Maximum thrust signal: {thrust_signal}.")
