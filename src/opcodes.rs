#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
#[allow(unused)]
pub enum Opcode
// IM = immediate byte value following the opcode
{
    NOOP,

    // 8 bit A/B opcodes
    // Push/pop operations between registers do not set OVF and ZERO flags
    PUSH_A_IMMEDIATE,
    PUSH_B_IMMEDIATE,
    PUSH_A_ABSOLUTE,
    PUSH_B_ABSOLUTE,
    PUSH_A_B,
    PUSH_B_A,
    PUSH_HI_A,
    PUSH_LI_A,
    PUSH_HI_B,
    PUSH_LI_B,

    PUSH_EXIT_CODE_A, // push EXIT_CODE to A
    PUSH_EXIT_CODE_B,

    POP_A_ABSOLUTE,
    POP_B_ABSOLUTE,
    POP_A_HI,
    POP_A_LI,
    POP_B_HI,
    POP_B_LI,

    // Arithmetic operations set OVF and ZERO flags accordingly
    ADD_A_B,         // A + B -> push to B
    ADD_B_A,         // A + B -> push to A
    ADD_A_IMMEDIATE, // A + IM -> push to A
    ADD_B_IMMEDIATE, // B + IM -> push to B
    ADD_A_ABSOLUTE,
    ADD_B_ABSOLUTE,

    SUB_A_B,         // A - B -> push to B
    SUB_B_A,         // B - A -> push to A
    SUB_A_IMMEDIATE, // A - IM -> push to A
    SUB_B_IMMEDIATE, // B - IM -> push to B
    SUB_A_ABSOLUTE,
    SUB_B_ABSOLUTE,

    SHIFT_LEFT_A,  // set first bit to 0
    SHIFT_RIGHT_A, // set last bit to 0
    SHIFT_LEFT_B,
    SHIFT_RIGHT_B,

    AND_A_B, // A and B -> push to A
    AND_B_A, // A and B -> push to B
    OR_A_B,  // A or B -> push to A
    OR_B_A,  // A or B -> push to B
    XOR_A_B, // A xor B -> push to A
    XOR_B_A, // A xor B -> push to B
    NOT_A,   // not A -> push to A
    NOT_B,   // not B -> push to B

    // comparisons
    CMP_A_B, // compare A and B: set ZERO if equal, set OVF if A > B, clear ZERO and OVF if A < B
    CMP_A_IMMEDIATE, // flags set same as above
    CMP_B_IMMEDIATE,
    CMP_A_ABSOLUTE,
    CMP_B_ABSOLUTE,

    // 8 bit H/L Index opcodes
    PUSH_HI_IMMEDIATE,
    PUSH_LI_IMMEDIATE,
    PUSH_HI_ABSOLUTE,
    PUSH_LI_ABSOLUTE,

    STORE_HI_ABSOLUTE,
    STORE_LI_ABSOLUTE,

    CMP_HI_IMMEDIATE, // same as CMP_A_B
    CMP_LI_IMMEDIATE,
    CMP_HI_ABSOLUTE,
    CMP_LI_ABSOLUTE,

    INC_HI,
    INC_LI, // Increment LI, set OVF if overflown, do not change HI, set ZERO if zeroed

    DEC_HI,
    DEC_LI, // Decrement LI, set OVF if overflown, do not change HI, set ZERO if zeroed

    ZERO_HI, // set HI to 0, do not set ZERO flag
    ZERO_LI, // set LI to 0, do not set ZERO flag

    SWAP_HI_LI, // no flags set

    // 16 bit A/B opcodes
    PUSH_AB_IMMEDIATE,
    PUSH_AB_ABSOLUTE,
    PUSH_HLI_AB,

    POP_AB_ABSOLUTE,
    POP_AB_HLI,

    ADD_AB_IMMEDIATE,
    ADD_AB_ABSOLUTE,

    SUB_AB_IMMEDIATE,
    SUB_AB_ABSOLUTE,

    SHIFT_LEFT_AB, // same as 8 bit shift
    SHIFT_RIGHT_AB,

    AND_AB_IMMEDIATE, // same as 8 bit ops
    AND_AB_ABSOLUTE,

    OR_AB_IMMEDIATE,
    OR_AB_ABSOLUTE,

    XOR_AB_IMMEDIATE,
    XOR_AB_ABSOLUTE,

    NOT_AB,

    // 16 bit H/L Index opcodes
    PUSH_HLI_IMMEDIATE,
    PUSH_HLI_ABSOLUTE,

    STORE_HLI_ABSOLUTE,

    // 16 bit comparisons
    CMP_AB_IMMEDIATE,
    CMP_AB_ABSOLUTE,

    CMP_HLI_IMMEDIATE,
    CMP_HLI_ABSOLUTE,

    INC_HLI,

    DEC_HLI,

    ZERO_HLI, // set HI and LI to 0, do not set ZERO flag

    // Control flow opcodes
    JUMP_IF_ZERO,
    JUMP_IF_OVERFLOW,
    JUMP_IF_ERROR,
    JUMP_IF_OK,
    JUMP,

    // Stack
    PUSH_AB_STACK_ADDRESS, // Push `stack_address` to AB, set ZERO flag accordingly
    PUSH_AB_STACK_SIZE,    // Push `stack_size` to AB, set ZERO flag accordingly
    POP_AB_STACK_ADDRESS, // Pop value from AB to `stack_address`, setting `stack_pointer` to the same value. This effectively resets the stack, preventing any `RETURN`s from working!
    POP_AB_STACK_SIZE, // Pop value from AB to `stack_size`. Can be used to increase stack size on the fly, as it does not reset the stack.
    POP_AB_IRQ,
    PUSH_A_STACK,  // push value(s) from A to stack IMMEDIATE value times
    POP_A_STACK,   // pop value(s) from stack to A IMMEDIATE value times
    PUSH_B_STACK,  // push value(s) from B to stack IMMEDIATE value times
    POP_B_STACK,   // pop value(s) from stack to B IMMEDIATE value times
    PUSH_HI_STACK, // push HI to stack
    POP_HI_STACK,  // pop HI from stack
    PUSH_LI_STACK, // push LI to stack
    POP_LI_STACK,  // pop LI from stack

    // Functions. Note that `stack_size` == 0 disables `CALL` and all `RETURN` functions
    CALL, // push flags, index registers and current PC to stack and jump to specified address
    RETURN_OK, // pop flags, set the OK flag (do not pop ERR/OK flags) and set EXIT_CODE to 0, pop index registers and PC from stack and resume from PC
    RETURN_OK_EXIT_CODE, // same as above but set EXIT_CODE to IMMEDIATE value
    RETURN_ERR, // pop flags, set the ERR flag (do not pop ERR/OK flags) and set EXIT_CODE to 0, pop index registers and PC from stack and resume from PC
    RETURN_ERR_EXIT_CODE, // same as above but set EXIT_CODE to IMMEDIATE value

    // Flags
    SET_ERR,
    SET_IRQ, // Enable interrupts

    CLEAR_EXIT_CODE,
    CLEAR_ERR,
    CLEAR_IRQ, // Disable interrupts
    CLEAR_OVF,
    CLEAR_ZERO,

    HALT, // halt execution
}
