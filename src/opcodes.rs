#[derive(Debug, Clone)]
#[allow(non_camel_case_types)]
#[allow(unused)]
/// @brief All opcodes follow the convention: OPERATION SOURCE TARGET
/// Example: PUSH_A_B = PUSH value from A to B
pub(crate) enum Opcode
// IM = immediate byte value following the opcode
{
    NOOP,

    // 8 bit A/B opcodes
    // Push/pop operations between registers do not set OVF and ZERO flags
    PUSH_IMMEDIATE_A,
    PUSH_IMMEDIATE_B,
    PUSH_ABSOLUTE_A,
    PUSH_ABSOLUTE_B,
    PUSH_INDIRECT_A, // Push value from address in HLI into A
    PUSH_INDIRECT_B, // Push value from address in HLI into B
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
    POP_A_INDIRECT, // Pop value from A into address in HLI
    POP_B_INDIRECT, // Pop value from B into address in HLI
    POP_A_HI,
    POP_A_LI,
    POP_B_HI,
    POP_B_LI,

    PEEK_A_ABSOLUTE,
    PEEK_B_ABSOLUTE,
    PEEK_A_INDIRECT, // Peek value from A into address in HLI
    PEEK_B_INDIRECT, // Peek value from B into address in HLI

    // Arithmetic operations set OVF and ZERO flags accordingly
    ADD_A_B,         // A + B -> push to B
    ADD_B_A,         // A + B -> push to A
    ADD_IMMEDIATE_A, // A + IM -> push to A
    ADD_IMMEDIATE_B, // B + IM -> push to B
    ADD_ABSOLUTE_A,
    ADD_ABSOLUTE_B,

    SUB_A_B,         // A - B -> push to B
    SUB_B_A,         // B - A -> push to A
    SUB_IMMEDIATE_A, // A - IM -> push to A
    SUB_IMMEDIATE_B, // B - IM -> push to B
    SUB_ABSOLUTE_A,
    SUB_ABSOLUTE_B,

    SHIFT_LEFT_A,  // set first bit to 0
    SHIFT_RIGHT_A, // set last bit to 0
    SHIFT_LEFT_B,
    SHIFT_RIGHT_B,

    AND_A_B, // A and B -> push to B
    AND_B_A, // A and B -> push to A
    OR_A_B,  // A or B -> push to B
    OR_B_A,  // A or B -> push to A
    XOR_A_B, // A xor B -> push to B
    XOR_B_A, // A xor B -> push to A
    NOT_A,   // not A -> push to A
    NOT_B,   // not B -> push to B

    // comparisons
    CMP_A_B, // compare A and B: set ZERO if equal, set OVF if A > B, clear ZERO and OVF if A < B
    CMP_IMMEDIATE_A, // flags set same as above
    CMP_IMMEDIATE_B,
    CMP_ABSOLUTE_A,
    CMP_ABSOLUTE_B,

    // 8 bit H/L Index opcodes
    PUSH_IMMEDIATE_HI,
    PUSH_IMMEDIATE_LI,
    PUSH_ABSOLUTE_HI,
    PUSH_ABSOLUTE_LI,

    STORE_HI_ABSOLUTE,
    STORE_LI_ABSOLUTE,

    CMP_IMMEDIATE_HI, // same as CMP_A_B
    CMP_IMMEDIATE_LI,
    CMP_ABSOLUTE_HI,
    CMP_ABSOLUTE_LI,

    INC_HI,
    INC_LI, // Increment LI, set OVF if overflown, do not change HI, set ZERO if zeroed

    DEC_HI,
    DEC_LI, // Decrement LI, set OVF if overflown, do not change HI, set ZERO if zeroed

    ZERO_HI, // set HI to 0, do not set ZERO flag
    ZERO_LI, // set LI to 0, do not set ZERO flag

    SWAP_HI_LI, // no flags set

    // 16 bit A/B opcodes
    PUSH_IMMEDIATE_AB,
    PUSH_ABSOLUTE_AB,
    PUSH_INDIRECT_AB, // Push value from address in HLI into AB
    PUSH_HLI_AB,

    POP_AB_ABSOLUTE,
    POP_AB_INDIRECT, // Pop value from AB into address in HLI
    POP_AB_HLI,

    PEEK_AB_ABSOLUTE,
    PEEK_AB_INDIRECT, // Peek value from AB into address in HLI

    ADD_IMMEDIATE_AB,
    ADD_ABSOLUTE_AB,

    SUB_IMMEDIATE_AB,
    SUB_ABSOLUTE_AB,

    SHIFT_LEFT_AB, // same as 8 bit shift
    SHIFT_RIGHT_AB,

    AND_IMMEDIATE_AB, // same as 8 bit ops
    AND_ABSOLUTE_AB,

    OR_IMMEDIATE_AB,
    OR_ABSOLUTE_AB,

    XOR_IMMEDIATE_AB,
    XOR_ABSOLUTE_AB,

    NOT_AB,

    // 16 bit H/L Index opcodes
    PUSH_IMMEDIATE_HLI,
    PUSH_ABSOLUTE_HLI,

    STORE_HLI_ABSOLUTE,

    // 16 bit comparisons
    CMP_IMMEDIATE_AB,
    CMP_ABSOLUTE_AB,

    CMP_IMMEDIATE_HLI,
    CMP_ABSOLUTE_HLI,

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
    POP_STACK_ADDRESS_AB, // Pop value from AB to `stack_address`, setting `stack_pointer` to the same value. This effectively resets the stack, preventing any `RETURN`s from working!
    POP_STACK_SIZE_AB, // Pop value from AB to `stack_size`. Can be used to increase stack size on the fly, as it does not reset the stack.
    POP_AB_IRQ,
    PUSH_A_STACK,  // push value(s) from A to stack IMMEDIATE value times
    POP_STACK_A,   // pop value(s) from stack to A IMMEDIATE value times
    PUSH_B_STACK,  // push value(s) from B to stack IMMEDIATE value times
    POP_STACK_B,   // pop value(s) from stack to B IMMEDIATE value times
    PUSH_HI_STACK, // push HI to stack
    POP_STACK_HI,  // pop HI from stack
    PUSH_LI_STACK, // push LI to stack
    POP_STACK_LI,  // pop LI from stack

    // Functions. Note that `stack_size` == 0 disables `CALL` and all `RETURN` functions
    CALL, // push flags, index registers, current PC, `stack_size` and `stack_address` to stack. Then, setup a new stack starting at current `stack_pointer` with size `stack_size` - (`stack_pointer` - `stack_address`) and jump to specified address
    RETURN_OK, // return to the old `stack_address` and `stack_size` values, then pop flags, set the OK flag (do not pop ERR/OK flags) and set EXIT_CODE to 0, pop index registers and PC from stack and resume from PC
    RETURN_OK_EXIT_CODE, // same as above but set EXIT_CODE to IMMEDIATE value
    RETURN_ERR, // return to the old `stack_address` and `stack_size` values, then pop flags, set the ERR flag (do not pop ERR/OK flags) and set EXIT_CODE to 0, pop index registers and PC from stack and resume from PC
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
