from itertools import permutations

try:
    from intcode import IntcodeComputer
except ImportError:
    from .intcode import IntcodeComputer


def max_signal(intcode_program, phase_range):
    thruster_signals = {}
    for phase_setting in permutations(phase_range, 5):
        computers = [
            IntcodeComputer(intcode_program, amp_phase=phase) for phase in phase_setting
        ]
        signal_input = 0
        current_computer = 0
        while not (comp := computers[current_computer]).halted():
            comp.inputs.append(signal_input)
            signal_input = comp.run()
            current_computer += 1
            if current_computer == 5:
                current_computer = 0
        thruster_signals[signal_input] = phase_setting
    max_signal = max(thruster_signals)
    print("Amplifier phase setting =>", thruster_signals[max_signal])
    return max_signal


if __name__ == "__main__":
    with open("input/07.txt") as fd:
        input_program = list(map(int, fd.readline().strip().split(",")))

    print("Maximum signal, Part A =>", max_signal(input_program, range(5)))
    print("Maximum signal, Part B =>", max_signal(input_program, range(5, 10)))
