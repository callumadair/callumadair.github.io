//! Build script based off the example [found here](https://github.com/LyonSyonII/dioxus-tailwindcss/blob/main/build.rs)

use strum::{
    AsRefStr,
    IntoStaticStr,
};

/// Checks package runner is installed and runs tailwind.
pub fn main() -> color_eyre::Result<()>
{
    if std::env::var("SKIP_BUILD_SCRIPT").unwrap_or("0".into()) == "1"
    {
        return Ok(());
    }
    println!("cargo:rerun-if-changed=src/**/*.rs");
    let toolchain = install_packages();

    // Compile TailwindCSS .css file
    std::process::Command::new(toolchain)
        .args(["tailwindcss", "-i", "./main.css", "-o", "./assets/out.css"])
        .output()?;
    Ok(())
}

#[derive(AsRefStr)]
#[strum(serialize_all = "lowercase")]
enum PackageManagers
{
    Bun,
    Npm,
}

#[derive(IntoStaticStr)]
#[strum(serialize_all = "lowercase")]
enum BuildTools
{
    BunX,
    NpX,
}

/// Installs required packages and selects toolchain to use.
///
///
/// # Panic
/// Will panic if none of the toolchains is installed.
fn install_packages() -> &'static str
{
    if std::process::Command::new(PackageManagers::Bun.as_ref())
        .arg("install")
        .spawn()
        .is_ok()
    {
        return BuildTools::BunX.into();
    }

    if std::process::Command::new(PackageManagers::Npm.as_ref())
        .arg("install")
        .spawn()
        .is_ok()
    {
        return BuildTools::NpX.into();
    }

    panic!("Bun is not installed");
}
