use std::num::Int;
use datatypes::{Byte, Word};


/**
 * Return the "low" 8 bits of `val', e.g. given 0xBEEF returns 0xBE
 */
pub fn low8(val: Word) -> Byte {
    (val >> 8).to_u8().unwrap()
}
    
/**
 * Return the "high" 8 bits of val, e.g. given 0xBEEF returns 0xEF
 */
pub fn high8(val: Word) -> Byte {
    (val & 0xFF).to_u8().unwrap()
}

/**
 * Join two Bytes into a Word
 */
pub fn join8(low: Byte, high: Byte) -> Word {
    let mut word = high.to_u16().unwrap();
    let low = low.to_u16().unwrap();
    word = word << 8;
    word = word + low;
    word
}

/**
 * Replace the low byte of `val' with `low'
 */
pub fn join_low8(val: Word, low: Byte) -> Word {
    join8(high8(val), low)
}

/**
 * Replace the high byte of `val' with `high'
 */
pub fn join_high8(val: Word, high: Byte) -> Word {
    join8(high, low8(val))
}


/**
 * Calculate overflow flag for ADD operations
 */
fn add_overflow(l_sign: bool, r_sign: bool, result_sign: bool) -> bool {
    (result_sign != l_sign) && (l_sign == r_sign)
}

/**
 * Calculate overflow flag for SUB operations
 */
fn sub_overflow(l_sign: bool, r_sign: bool, result_sign: bool) -> bool {
    ((!l_sign && r_sign) && result_sign) ||
    ((l_sign && !r_sign) && !result_sign)
}

/**
 * Arithmetic functions. Functions generated from this macro take input
 * as Bytes or Words, calculate the output, and also compute the carry,
 * overflow, sign, and zero flags.
 */
macro_rules! arithmetic (
    (
        $name:ident,
        $input_type:ident,
        $un_op:ident $ch_op:ident,
        $overflow_fn:ident
    ) => {
        pub fn $name(left: $input_type, right: $input_type)
        -> ($input_type, bool, bool, bool, bool) {
            let result = left.$un_op(right);

            let l_sign: bool = left.leading_zeros() == 0;
            let r_sign: bool = right.leading_zeros() == 0;
            let result_sign: bool = result.leading_zeros() == 0;

            let overflow: bool = $overflow_fn(l_sign, r_sign, result_sign);
            let zero: bool = result == 0;
            let carry: bool = match left.$ch_op(right) {
                Some(_) => false,
                None => true,
            };

            (result, carry, overflow, result_sign, zero)
        }
    }
);

arithmetic!(b_add, Byte, add checked_add, add_overflow);
arithmetic!(b_sub, Byte, sub checked_sub, sub_overflow);
arithmetic!(w_add, Word, add checked_add, add_overflow);
arithmetic!(w_sub, Word, sub checked_sub, sub_overflow);
