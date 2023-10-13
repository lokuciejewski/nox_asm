# Nox ASM

`Nox ASM` is an assembler created for the fictional 8/16bit cpu, [Nox CPU](https://github.com/lokuciejewski/nox_cpu).

## Syntax

`Nox ASM` syntax is loosely based on other 8-bit CPU assemblers, especially `6502`'s. It was designed to be as simple as possible while retaining readability and allowing the user to use all `Nox CPU` opcodes.

`Nox ASM` syntax is case-insensitive, meaning:

```asm
push a b == PUSH a b == push A B ect.
```

### Special tokens

- `$` - indicates a data stream. It can be a text, byte or multiple 8 or 16 bit values.

```asm
$ 0x1 0x23 0x02
or
$ "This is an ASCII text"
or
$ "This is an ASCII text that is followed by some values" 0x1234 0xde 0xad 0xbe 0xef
or 
$ 0x1 "Text" 0x2
etc.
```

- `&` - indicates an absolute addressing mode used in some instructions. By default, all calls involving labels are absolute. It is also used in indirect mode with the `HLI` register.

```asm
push 0x1234 AB  // This will push the value `0x1234` into `AB`
push &0x1234 AB // This will push the value read from address `0x1234` (and `0x1235`) into `AB`
```

```asm
jmp loop_end // This does not need the `&` symbol
```

```asm
PUSH 0xf000 HLI // `HLI` will now contain `0xf000`
INC HLI // `HLI` == `0xf001`
PUSH &HLI A // a value from `0xf001` will be pushed into `A`
```

- `//` - indicates the start of the comment. All following tokens are ignored

```asm
PUSH A B // This is a comment
// PUSH B A <= line is ignored completely
```

- `>` - indicates the absolute memory position of the following token. By default, the addressing starts at `0x0000`

```asm
PUSH A B // this is on 0x0000

> 0x1234
PUSH B A // this is on 0x1234
ADD 0x12 A // this is on 0x1235

> 0xffff
HALT // this is the last byte
```

- `*` - indicates the memory location of the label

```asm
PUSH text AB // This pushes the value at `0x1234` to `AB`
PUSH *text HLI // This pushes the `0x1234` to `HLI`

> 0x1234
text:
$ "Some text"
```

### Labels

Declaring a label is done by using any unique text (no whitespaces) followed by ":"

```asm
i_am_a_valid_label:
pop: // still valid
this:is:valid:too:
this is not a valid label:
```

To declare label at a specific address, use `>` token:

```asm
> 0x1234
i_am_at_0x1234:

JMP i_am_at_0x1234 // equal to JMP &0x1234

> 0xffff
i_am_the_last_byte:
```

Since the labels are assigned at second assembly pass, they do not need to be declared before the usage:

```asm
jmp this_is_not_yet_declared

this_is_not_yet_declared:
    POP A B
```

### Instructions

For full list of all instructions along with the cycles count and flag changes, please see [a full list of all Nox CPU opcodes](https://github.com/lokuciejewski/nox_cpu/blob/main/docs/opcodes.md)
