use ethers::prelude::Abigen;

fn main() {
    rust_file_generation("ERC20", "./abi/ERC20.json", "./src/bindings/erc20.rs");
    rust_file_generation(
        "Timelock",
        "./abi/Timelock.json",
        "./src/bindings/timelock.rs",
    );
}

fn rust_file_generation(name: &str, abi_source: &str, path: &str) {
    let out_file = std::env::current_dir()
        .expect("Could not get current dir")
        .join(path);
    if out_file.exists() {
        std::fs::remove_file(&out_file).expect("Could not remove file");
    }
    Abigen::new(name, abi_source)
        .expect("Could not create Abigen")
        .generate()
        .expect("Could not generate abigen")
        .write_to_file(out_file)
        .expect("could not write bindings to file");
}
