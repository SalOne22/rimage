use winresource::{VersionInfo, WindowsResource};

// This is the pre-release version number.
const VERSION_PRE: u16 = 0;

fn main() {
    // only run if target os is windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        return;
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
