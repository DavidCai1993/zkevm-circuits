// Currently using 32 - each hash byte goes into its own cell, this might be compressed
// for optimization purposes in the future.
pub const HASH_WIDTH: usize = 32;
pub const WITNESS_ROW_WIDTH: usize = 68;
pub const LAYOUT_OFFSET: usize = 2;
pub const KECCAK_INPUT_WIDTH: usize = 1;
pub const KECCAK_OUTPUT_WIDTH: usize = 4;
// for s: RLP 2 cells + HASH_WIDTH cells
// for c: RLP 2 cells + HASH_WIDTH cells
pub const RLP_NUM: usize = 2;
pub const S_RLP_START: usize = 0;
pub const S_START: usize = RLP_NUM;
pub const C_RLP_START: usize = RLP_NUM + HASH_WIDTH;
pub const C_START: usize = RLP_NUM + HASH_WIDTH + RLP_NUM;
pub const BRANCH_0_S_START: usize = 2; // row 0 of a branch contains some RLP data
pub const BRANCH_0_C_START: usize = 5;
pub const BRANCH_0_KEY_POS: usize = 8; // row 0 of a branch contains info (deriving from key) about which branch node is being modified