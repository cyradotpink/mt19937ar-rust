fn main() {
    cc::Build::new()
        .file("csrc/python-mt19937ar.c")
        .compile("mt19937ar");
}
