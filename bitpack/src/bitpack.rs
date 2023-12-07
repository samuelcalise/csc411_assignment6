//use std::convert::TryInto;

/// Returns true iff the signed value `n` fits into `width` signed bits.
/// 
/// # Arguments:
/// * `n`: A signed integer value
/// * `width`: the width of a bit field
pub fn fitss(n: i64, width: u64) -> bool {

    if n < 0 //the signed value is negative
    {
        //take the opposite of the value and compare left shift by one
        (-n) as u64 <= (1 << (width - 1))
    }
    else
    {
        (n as u64) < (1 << (width - 1))
    }
}

/// Returns true iff the unsigned value `n` fits into `width` unsigned bits.
/// 
/// # Arguments:
/// * `n`: An usigned integer value
/// * `width`: the width of a bit field
pub fn fitsu(n: u64, width: u64) -> bool {

    if n >> width == 0
    {
        //the value fits within the width of unsigned bits
        return true;
    }
    if width == 0
    {
        //confirms the case where is the width of the bit is 0, then is
        //n also 0(true, false)?
        n == 0
    }
    else //doesnt work
    {
        false
    }
    
}

/// Retrieve a signed value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn gets(word: u64, width: u64, lsb: u64) -> i64 {
/*  //left shifting to get the most  |   //After the left shift, then
    //significant bit position in    |   //right shift off the uneeded
    //the signed value               |   //bits from the initial left shift */
    (word << (64 - width - lsb)) as i64 >> (64 - width)
}

/// Retrieve an unsigned value from `word`, represented by `width` bits
/// beginning at least-significant bit `lsb`.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
pub fn getu(word: u32, lsb: u32, width: u32) -> u32 {
/*  //left shifting to get the most  |   //After the left shift, then
    //significant bit position in    |   //right shift off the uneeded
    //the unsigned value             |   //bits from the initial left shift */
    //eprintln!("{:032b} this is the word {},{}", word, width, lsb);
    return (word << (32 - width - lsb)) >> (32 - width)
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the unsigned `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` unsigned bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the unsigned value to place into that bit field
pub fn newu(word: u64, width: u64, lsb: u64, value: u64) -> Option<u64> {
    
    //checks the passed in value and it it fits within the
    //the width of the unsigned bit
    if fitsu(value, width)
    {
        //When true, the value passed in the then left-shifted by the
        //least significant bit then the BitOR opertaion updates the 
        //passed in word, then returns it as Some(newer_word)
        return Some(value << lsb | word);
    } 
    else 
    {
        //doesnt work 
        return None;
    }
}

/// Return a modified version of the unsigned `word`,
/// which has been updated so that the `width` bits beginning at
/// least-significant bit `lsb` now contain the signed `value`.
/// Returns an `Option` which will be None iff the value does not fit
/// in `width` signed bits.
/// 
/// # Arguments:
/// * `word`: An unsigned word
/// * `width`: the width of a bit field
/// * `lsb`: the least-significant bit of the bit field
/// * `value`: the signed value to place into that bit field
pub fn news(word: u64, width: u64, lsb: u64, value: i64) -> Option<u64> {
    
    //if the signed value does not if within the width of the bit
    if fitss(value, width)
    {
        //temp holds the number of bits of the signed value
        let temp = (1_u64 << width) - 1;

        //Returning some option value, of the updated word passed in
        return Some( word | ((value as u64 & temp) << lsb));
    }

    None

    
}