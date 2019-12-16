# Part 2
import itertools, subprocess

INPUT_FILENAME = "input/day7-part2-input.txt"
seq_sig = [] # Stores the pairs of permutations and signals.
# Iterate over every permutation of "56789".
for perm in itertools.permutations("56789"):
    amplifiers = [] # The amplifier processes.
    signal = '0' # The input/output signal for/from each amplifier.
    # Initialize each amplifier with the phase setting and signal (both are
    # different for each process).
    for phase_setting in perm:
        # Each amplifier is an Intcode computer subprocess.
        proc = subprocess.Popen(
            ("./intcode", INPUT_FILENAME),
            encoding='utf-8',
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            universal_newlines=True)
        # Write the phase setting and signal to the process's stdin (the flush
        # is important!).
        #print(f"Sending `{phase_setting}\\n{signal}\\n` to subprocess...");
        proc.stdin.write(f"{phase_setting}\n{signal}\n")
        proc.stdin.flush()
        # Read the signal from the process's stdout.
        signal = proc.stdout.readline().strip()
        #print(f"Received `{signal}` from subprocess.");
        # Add this amplifier to the list of amplifiers.
        amplifiers.append(proc)
    # Continue until the processes end.
    cont = True
    while cont:
        for proc in amplifiers:
            if proc.poll() is not None:
                cont = False
                #print("Process ending...");
                continue
            #print(f"Sending `{signal}\\n` to subprocess...");
            proc.stdin.write(f"{signal}\n")
            proc.stdin.flush()
            signal = proc.stdout.readline().strip()
            #print(f"Received `{signal}` from subprocess.");
    seq_sig.append(('-'.join(perm), int(signal)))
# Make sure all the subprocesses have ended.
for proc in amplifiers:
    proc.terminate()
# Get the phase sequence that results in the maximum thrust signal.
(phase_sequence, thrust_signal) = max(seq_sig, key=lambda x: x[1])
print(f"Phase sequence with maximum thrust signal: {phase_sequence}.")
print(f"Maximum thrust signal: {thrust_signal}.")
