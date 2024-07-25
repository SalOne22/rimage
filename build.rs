extern crate winres;
use winres::VersionInfo;

fn main() {
    // only run if target os is windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        println!(
            "cargo:warning={:#?}",
            "This build script is only for windows target, skipping..."
        );
        return;
    }

    let mut res = winres::WindowsResource::new();

    //version  X. X. X. X
    //         ⇑  ⇑  ⇑
    //         48 32 16
    let mut version: u64 = 0;
    version |= 0 << 48;
    version |= 11 << 32;
    version |= 0 << 16;
    version |= 2;

    res.set_version_info(VersionInfo::FILEVERSION, version)
        .set_version_info(VersionInfo::PRODUCTVERSION, version);

    if let Err(e) = res.compile() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
