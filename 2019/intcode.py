from typing import Optional


class IntcodeComputer:
    def __init__(
        self,
        program: list[int],
        inputs: Optional[list[int]] = None,
        amp_phase: Optional[int] = None,
        ask_for_input: bool = False,
        print_output: bool = False,
        return_output: bool = False,
    ) -> None:
        #: Copy of the original program (Never mutate the original reference)
        self.program = program[:]

        self.program_length = len(program)

        #: Return the output keeping the state of the computer intact.
        self.return_output = return_output

        #: A flag to indicate to ask for input instead of using the `inputs` parameter.
        self.ask_for_input = ask_for_input

        #: A flag to indicate to print the `output` from the computer to stdout.
        #: Defaults to ``True`` if `ask_for_input` is ``True``.
        self.print_output = ask_for_input or print_output

        #: If `inputs` is the sentinel value ``None``, then initialize it to an
        #: empty list.
        if inputs is None:
            inputs = []

        #: If the amplifier phase is given, then insert it at the start of the inputs
        #: and default the return_output to be ``True``.
        if amp_phase is not None:
            inputs.insert(0, amp_phase)
            self.return_output = True

        #: Copy of the original inputs (Never mutate the original reference)
        self.inputs = inputs[:]

        #: Keep the amplifier phase for later use, if any.
        #: TODO: This is not being used as of now, so remove it?
        self.amp_phase = amp_phase

        #: Value indicating the current position in the program.
        self.pointer = 0

        #: Store the output from the computer in the list to be returned once the
        #: program is finished.
        self.outputs = []

        #: Relative base value used for relative mode (Default: 0)
        self.relative_base = 0

        #: A dictionary representing the memory stored outside the program
        self.extra_memory = {}

        self.__original_program = program
        self.__original_inputs = inputs

    def reset(self) -> None:
        """Reset the computer to its initial state.

        Following properties will be reseted:
        - Program
        - Inputs
        - Pointer
        - Outputs
        """
        self.program = self.__original_program[:]
        self.inputs = self.__original_inputs[:]
        self.pointer = 0
        self.outputs = []

    @property
    def value(self) -> int:
        """Return the value in the program at the current pointer."""
        return self.program[self.pointer]

    def halted(self) -> bool:
        """Determine whether the computer has halted or not."""
        return self.value == 99

    def current_code(self) -> str:
        """Return the current instruction code at the current pointer."""
        return str(self.value)[-1]

    def run(self):
        while not self.halted():
            code = self.current_code()
            execute_func = getattr(self, f"_execute_code_{code}")
            output = execute_func()
            if output is not None:
                return output
        return self.outputs

    def _param_val_and_mode(self, param_qty: int) -> tuple[int, ...]:
        parameters = self.program[self.pointer + 1 : self.pointer + (param_qty + 1)]
        instruction = str(self.value).zfill(param_qty + 2)
        parameters_mode = [int(i) for i in instruction[-3::-1]]
        return (*parameters, *parameters_mode)

    def _value_for_mode(self, param, mode):
        if mode == 1:  # immediate mode
            return param
        elif mode == 0:  # position mode
            memory_index = param
        elif mode == 2:  # relative mode
            memory_index = self.relative_base + param

        if memory_index > self.program_length:
            return self.extra_memory.get(memory_index, 0)
        else:
            return int(self.program[memory_index])

    def _store_in_memory(self, index, mode, value):
        if mode == 2:
            memory_index = self.relative_base + index
        else:
            memory_index = index
        if memory_index > self.program_length:
            self.extra_memory[memory_index] = value
        else:
            self.program[memory_index] = value

    def _execute_code_1(self):
        p1, p2, p3, p1_mode, p2_mode, p3_mode = self._param_val_and_mode(3)
        v1 = self._value_for_mode(p1, p1_mode)
        v2 = self._value_for_mode(p2, p2_mode)
        self._store_in_memory(p3, p3_mode, v1 + v2)
        self.pointer += 4

    def _execute_code_2(self):
        p1, p2, p3, p1_mode, p2_mode, p3_mode = self._param_val_and_mode(3)
        v1 = self._value_for_mode(p1, p1_mode)
        v2 = self._value_for_mode(p2, p2_mode)
        self._store_in_memory(p3, p3_mode, v1 * v2)
        self.pointer += 4

    def _execute_code_3(self):
        p1, p1_mode = self._param_val_and_mode(1)
        if self.ask_for_input:
            self._store_in_memory(p1, p1_mode, int(input("Input: ")))
        else:
            self._store_in_memory(p1, p1_mode, self.inputs.pop(0))
        self.pointer += 2

    def _execute_code_4(self):
        p1, p1_mode = self._param_val_and_mode(1)
        v1 = self._value_for_mode(p1, p1_mode)
        self.outputs.append(v1)
        if self.print_output:
            print(f"Output: {v1}")
        self.pointer += 2
        if self.return_output:
            return v1

    def _execute_code_5(self):
        p1, p2, p1_mode, p2_mode = self._param_val_and_mode(2)
        check = self._value_for_mode(p1, p1_mode)
        jump = self._value_for_mode(p2, p2_mode)
        self.pointer = jump if check else self.pointer + 3

    def _execute_code_6(self):
        p1, p2, p1_mode, p2_mode = self._param_val_and_mode(2)
        check = self._value_for_mode(p1, p1_mode)
        jump = self._value_for_mode(p2, p2_mode)
        self.pointer = jump if not check else self.pointer + 3

    def _execute_code_7(self):
        p1, p2, p3, p1_mode, p2_mode, p3_mode = self._param_val_and_mode(3)
        v1 = self._value_for_mode(p1, p1_mode)
        v2 = self._value_for_mode(p2, p2_mode)
        self._store_in_memory(p3, p3_mode, int(v1 < v2))
        self.pointer += 4

    def _execute_code_8(self):
        p1, p2, p3, p1_mode, p2_mode, p3_mode = self._param_val_and_mode(3)
        v1 = self._value_for_mode(p1, p1_mode)
        v2 = self._value_for_mode(p2, p2_mode)
        self._store_in_memory(p3, p3_mode, int(v1 == v2))
        self.pointer += 4

    def _execute_code_9(self):
        p1, p1_mode = self._param_val_and_mode(1)
        p1 = self._value_for_mode(p1, p1_mode)
        self.relative_base += p1
        self.pointer += 2
