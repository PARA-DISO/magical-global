fn main() {
    println!("cargo::rerun-if-changed=src/hello.c");

    cc::Build::new()
        .file("assets/magical-global.c")
        .compile("magical-global");
}
