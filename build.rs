fn main() {
    #[cfg(feature = "graphics")]
    {
        res::generate(&[res::shader("gui").transform().graphic()]);
    }
}
