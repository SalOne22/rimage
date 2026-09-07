use winresource::{VersionInfo, WindowsResource};

fn main() {
    // only run if target os is windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
    }

    // The winresource-based version-info build script only supports the MSVC
    // toolchain. Reject Windows GNU builds instead of failing later with a
    // confusing linker/resource error.
    if std::env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default() == "gnu" {
        eprintln!(
            "rimage on Windows only supports the MSVC toolchain; \
             x86_64-pc-windows-gnu / i686-pc-windows-gnu are not supported"
        );
        std::process::exit(1);
    }

    let pack = |pre: u16| -> u64 {
        (env_u64("CARGO_PKG_VERSION_MAJOR") << 48)
            | (env_u64("CARGO_PKG_VERSION_MINOR") << 32)
            | (env_u64("CARGO_PKG_VERSION_PATCH") << 16)
            | u64::from(pre)
    };

    let mut res = WindowsResource::new();

    res.set_version_info(VersionInfo::FILEVERSION, pack(VERSION_PRE))
        .set_version_info(VersionInfo::PRODUCTVERSION, pack(VERSION_PRE));

    if let Err(e) = res.compile() {
        eprintln!("{e}");
        std::process::exit(1);
    }
}

fn env_u64(name: &str) -> u64 {
    std::env::var(name).unwrap_or_default().parse().unwrap_or(0)
}
