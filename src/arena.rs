use bumpalo::Bump;

/// Arena consists of  bump allocators
///
/// no bound check ?
pub struct Arena {
    value: Bump,
}
