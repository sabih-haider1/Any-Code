fn main() {
    // libgit2-sys doesn't emit this itself on the current MSVC toolchain: it uses
    // legacy CryptoAPI, registry, and token functions (CryptAcquireContextA,
    // RegOpenKeyExW, OpenProcessToken, ...) that live in advapi32, not one of the
    // libs it already links. Without this, linking anycode-git on Windows fails
    // with ~19 unresolved externals.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        println!("cargo:rustc-link-lib=advapi32");
    }
}
