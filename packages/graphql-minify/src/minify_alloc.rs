use bumpalo::Bump;

#[derive(Default)]
pub struct MinifyAllocator {
    /// allocator used to store intermediate data during block strings minification
    pub(crate) block_string: Bump,
}
