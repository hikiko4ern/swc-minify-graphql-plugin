use bumpalo::Bump;

#[derive(Default)]
pub struct MinifyAllocator {
    /// allocator used to store intermediate data during block string minification
    pub(crate) block_string: Bump,
}
