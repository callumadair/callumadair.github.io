fn main()
{
    let toolchain = install_packages();
    // Compile TailwindCSS .css file
    std::process::Command::new(toolchain)
        .args([
            "@tailwind/cli",
            "-i",
            "./main.css",
            "-o",
            "./assets/out.css",
            "--minify",
        ])
        .env("NODE_ENV", "production")
        .spawn()
        .expect("Failed to run tailwind.");
}
/// Installs required packages and selects toolchain to use.
///
///
/// # Panic
/// Will panic if none of the toolchains is installed.
fn install_packages() -> &'static str
{
    let bun = "bunx";

    if std::process::Command::new(bun)
        .arg("install")
        .spawn()
        .is_ok()
    {
        return bun;
    }
    panic!("Bunx is not installed");
}
