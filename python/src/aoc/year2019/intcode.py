from typing import Optional


class IntcodeComputer:
    """The Intcode Computer used throughout the Advent of Code puzzles.

    Options:
        program: The intcode program to run on the computer.
        inputs: The input to provide when asked for.
        amp_phase: Amplifier phase for the current computer.
        ask_for_input: Ask for input to the user instead of using the inputs parameter.
        return_before_input: Stop the intcode computer right when the next instruction
            is the input instruction and return the sentinel_return object.
        sentinel_return: A sentinel value used to return if return_before_input is True.
        print_output: Print the output from the output instruction to stdout.
        return_output: Return the output from the output instruction keeping the
            state of the computer intact. Defaults to True if amp_phase is given.
        gather_output: Store all the output from the output instruction in a list
            and return it once the program ends.
    """

    def __init__(
        self,
        program: list[int],
        inputs: Optional[list[int]] = None,
        amp_phase: Optional[int] = None,
        ask_for_input: bool = False,
        return_before_input: bool = False,
        sentinel_return: object = object(),
        print_output: bool = False,
        return_output: bool = False,
        gather_output: bool = False,
    ) -> None:
        if inputs is None:
            inputs = []
        if amp_phase is not None:
            inputs.insert(0, amp_phase)

        self.return_before_input = return_before_input
        self.sentinel_return = sentinel_return
        self.ask_for_input = ask_for_input
        self.print_output = print_output
        self.return_output = return_output
        self.gather_output = gather_output
        self._returned = False

        self._pointer = 0
        self._outputs = []
        self._relative_base = 0

        # Internally, the entire program is stored in dictionary to account for an
        # arbitrary sized memory.
        internal = {index: value for index, value in enumerate(program)}
        self._inputs = inputs.copy()
        self._memory = internal.copy()
        self._original_program = internal
        self._original_inputs = inputs

    def reset(self) -> None:
        """Reset the computer to its initial state.

        Following properties will be reseted:
        - Program
        - Inputs
        - Pointer
        - Outputs
        """
        self._memory = self._original_program.copy()
        self._inputs = self._original_inputs.copy()
        self._pointer = 0
        self._outputs = []

    @property
    def value(self) -> int:
        """Return the value in the program at the current pointer."""
        return self._memory[self._pointer]

    def halted(self) -> bool:
        """Determine whether the computer has halted or not."""
        return self.value == 99

    def current_code(self) -> str:
        """Return the current instruction code at the current pointer."""
        return str(self.value)[-1]

    def append_inputs(self, *inputs) -> None:
        """Append the given inputs for the computer to use."""
        for i in inputs:
            self._inputs.append(i)

    def run(self):
        while not self.halted():
            code = self.current_code()
            if code == "3" and self.return_before_input and not self._returned:
                self._returned = True
                return self.sentinel_return
            execute_func = getattr(self, f"_execute_code_{code}")
            output = execute_func()
            if output is not None:
                return output
            if self._returned:
                self._returned = False
        if self.gather_output:
            return self._outputs

    def _param_val_and_mode(self, param_qty: int) -> tuple[int, ...]:
        p = self._pointer
        parameters = [self._memory[k] for k in range(p + 1, p + param_qty + 1)]
        instruction = str(self.value).zfill(param_qty + 2)
        parameters_mode = [int(i) for i in instruction[-3::-1]]
        return (*parameters, *parameters_mode)

    def _value_for_mode(self, index, mode):
        if mode == 1:  # immediate mode
            return index
        elif mode == 2:  # relative mode
            index += self._relative_base
        return self._memory.get(index, 0)

    def _store_in_memory(self, index, mode, value):
        if mode == 2:
            index += self._relative_base
        self._memory[index] = value

    def _execute_code_1(self):
        p1, p2, p3, p1_mode, p2_mode, p3_mode = self._param_val_and_mode(3)
        v1 = self._value_for_mode(p1, p1_mode)
        v2 = self._value_for_mode(p2, p2_mode)
        self._store_in_memory(p3, p3_mode, v1 + v2)
        self._pointer += 4

    def _execute_code_2(self):
        p1, p2, p3, p1_mode, p2_mode, p3_mode = self._param_val_and_mode(3)
        v1 = self._value_for_mode(p1, p1_mode)
        v2 = self._value_for_mode(p2, p2_mode)
        self._store_in_memory(p3, p3_mode, v1 * v2)
        self._pointer += 4

    def _execute_code_3(self):
        p1, p1_mode = self._param_val_and_mode(1)
        if self.ask_for_input:
            self._store_in_memory(p1, p1_mode, int(input("Input: ")))
        else:
            self._store_in_memory(p1, p1_mode, self._inputs.pop(0))
        self._pointer += 2

    def _execute_code_4(self):
        p1, p1_mode = self._param_val_and_mode(1)
        v1 = self._value_for_mode(p1, p1_mode)
        if self.gather_output:
            self._outputs.append(v1)
        if self.print_output:
            print(f"Output: {v1}")
        self._pointer += 2
        if self.return_output:
            return v1

    def _execute_code_5(self):
        p1, p2, p1_mode, p2_mode = self._param_val_and_mode(2)
        check = self._value_for_mode(p1, p1_mode)
        jump = self._value_for_mode(p2, p2_mode)
        self._pointer = jump if check else self._pointer + 3

    def _execute_code_6(self):
        p1, p2, p1_mode, p2_mode = self._param_val_and_mode(2)
        check = self._value_for_mode(p1, p1_mode)
        jump = self._value_for_mode(p2, p2_mode)
        self._pointer = jump if not check else self._pointer + 3

    def _execute_code_7(self):
        p1, p2, p3, p1_mode, p2_mode, p3_mode = self._param_val_and_mode(3)
        v1 = self._value_for_mode(p1, p1_mode)
        v2 = self._value_for_mode(p2, p2_mode)
        self._store_in_memory(p3, p3_mode, int(v1 < v2))
        self._pointer += 4

    def _execute_code_8(self):
        p1, p2, p3, p1_mode, p2_mode, p3_mode = self._param_val_and_mode(3)
        v1 = self._value_for_mode(p1, p1_mode)
        v2 = self._value_for_mode(p2, p2_mode)
        self._store_in_memory(p3, p3_mode, int(v1 == v2))
        self._pointer += 4

    def _execute_code_9(self):
        p1, p1_mode = self._param_val_and_mode(1)
        p1 = self._value_for_mode(p1, p1_mode)
        self._relative_base += p1
        self._pointer += 2
