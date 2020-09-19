/// Structure representing the state of a FSB computation
/// r is the size of the IV, which is equivalent to w * 8. We'd
/// remove on of both, but seems interesting to keep one as a, e.g.,
/// u64, and the other as a `digest` constant, e.g. U16. So we keep
/// the variable `w` and we define the BlockBuffer with that.
macro_rules! impl_state {
    ($state:ident, $n:ident, $w:ident, $r:ident, $p:ident) => {

        #[derive(Clone, Default)]
        pub struct $state {
            state: [u8; $w], // number of bits, in particular `w`. Then in the default we take from the constants
            buffer: BlockBuffer<$w>,
        }
    }
}
#[derive(Clone)]
pub struct Whirlpool {
    bit_length: [u8; 32],
    buffer: BlockBuffer<U64>,
    #[cfg(not(feature = "asm"))]
    hash: [u64; 8],
    #[cfg(feature = "asm")]
    hash: [u8; 64],
}

impl Default for Whirlpool {
    fn default() -> Self {
        Self {
            bit_length: [0u8; 32],
            buffer: BlockBuffer::default(),
            #[cfg(not(feature = "asm"))]
            hash: [0u64; 8],
            #[cfg(feature = "asm")]
            hash: [0u8; 64],
        }
    }
}

