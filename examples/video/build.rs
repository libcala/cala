fn main() {
    res::generate(&[res::shader("color")
        .transform()
        .gradient()
        .num_instances(2)]);
}
