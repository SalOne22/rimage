extern crate winres;
use std::env;
use winres::VersionInfo;

fn main() {
    // only run if target os is windows
    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        println!(
            "cargo:warning={:#?}",
            "This build script is only for windows target, skipping..."
        );
        return;
    }

    let mut res = winres::WindowsResource::new();

    match env::var("CARGO_PKG_VERSION_PRE") {
        Ok(success_info) => println!("{}", success_info),
        Err(err_info) => println!("{}", err_info),
    };

    //version    X.   X.    X.    X
    //           ⇑    ⇑     ⇑    ⇑
    //         MAJOR MINOR PATCH PRE
    let mut version: u64 = 0;
    version |= {
        env::var("CARGO_PKG_VERSION_MAJOR")
            .unwrap()
            .parse::<u64>()
            .unwrap()
            << 48
    };
    version |= {
        env::var("CARGO_PKG_VERSION_MINOR")
            .unwrap()
            .parse::<u64>()
            .unwrap()
            << 32
    };
    version |= {
        env::var("CARGO_PKG_VERSION_PATCH")
            .unwrap()
            .parse::<u64>()
            .unwrap()
            << 16
    };

    let product_version = version | {
        let temp = env::var("CARGO_PKG_VERSION_PRE").unwrap();
        if temp == *"" {
            0_u64
        } else {
            temp.parse::<u64>().unwrap_or(0_u64)
        }
    };

    res.set_version_info(VersionInfo::FILEVERSION, version)
        .set_version_info(VersionInfo::PRODUCTVERSION, product_version);

    if let Err(e) = res.compile() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
