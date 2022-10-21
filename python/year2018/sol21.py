def _():
    """This is the unabridged or "as-is" version of my instructions in Python.

    Do not run this as it'll run forever.
    """
    r0, r1, r2, r3, r4 = 0, 0, 0, 0, 0

    # Bitwise AND test
    while True:
        r2 = 123
        r2 &= 456
        if r2 == 72:
            break
    r2 = 0

    while True:
        r4 = r2 | 0x10000
        r2 = 6718165

        while True:
            r2 += r4 & 0xFF
            r2 &= 0xFFFFFF
            r2 *= 65899
            r2 &= 0xFFFFFF

            # r4 can only be < 256 when it's 0 as any number less than 256 divided
            # by 256 will return 0 as the quotient.
            if r4 < 256:
                break

            # This loop is basically finding the quotient of r4 divided by 256 and
            # assign that number back to r4.
            #
            # It can be summed up using `r4 >>= 8`, as 2⁸ = 256.
            r3 = 0
            while True:
                r1 = r3 + 1
                r1 *= 256
                if r1 > r4:
                    r4 = r3
                    break
                else:
                    r3 += 1

        # As we need to find the minimum value to put in register 0 to halt the program,
        # the value in register 2 the first time is the answer.
        if r2 == r0:
            break


def loop():
    """This is the simplified version of the above loop which returns the first and
    the last value which when specified in register 0 will cause the program to halt.

    The last value is the one after which the values are repeating.

    Following simplifications have been made:
    - No need to perform the bitwise AND test as Python is strictly typed, there are
      no string-as-numbers issues that the test guards against.
    - The inner most loop is just finding out the quotient of the number in register 4
      divided by 256 and assigning back to the same register.
    """
    first, last = -1, -1
    seen: set[int] = set()

    reg2 = 0
    while True:
        reg4 = reg2 | 0x10000
        reg2 = 6718165
        while reg4:
            reg2 += reg4 & 0xFF
            reg2 &= 0xFFFFFF
            reg2 *= 65899
            reg2 &= 0xFFFFFF
            reg4 >>= 8
        if first == -1:
            first = reg2
        elif reg2 in seen:
            break
        last = reg2
        seen.add(reg2)
    return first, last


if __name__ == "__main__":
    first, last = loop()

    print(f"Part 1: {first}")
    print(f"Part 2: {last}")
