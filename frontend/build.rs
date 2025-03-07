//! Build script based off the example [found here](https://github.com/LyonSyonII/dioxus-tailwindcss/blob/main/build.rs)

use strum::{
    AsRefStr,
    IntoStaticStr,
};

/// Checks package runner is installed and runs tailwind.
pub fn main()
{
    let toolchain = install_packages();

    // Compile TailwindCSS .css file
    std::process::Command::new(toolchain)
        .args(["tailwindcss", "-i", "./main.css", "-o", "./assets/out.css"])
        .output()
        .expect("Failed to run tailwind.");
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
