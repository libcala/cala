fn main() {
    #[cfg(feature = "draw")]
    {
        res::generate(&[res::shader("gui").transform().graphic()]);
    }
}
