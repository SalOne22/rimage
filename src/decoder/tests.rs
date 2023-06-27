use std::fs;

use regex::Regex;

use super::*;

#[test]
fn decode_unsupported() {
    let path = path::Path::new("tests/files/test.bmp");

    let file = fs::File::open(path).unwrap();

    let decoder = Decoder::new(path, file);
    let result = decoder.decode();
    assert!(result.is_err());
}

#[test]
fn decode_grayscale() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/[^x]&[^t].+0g\d\d((\.png)|(\.jpg))").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    })
}

#[test]
fn decode_grayscale_alpha() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/[^x].+4a\d\d\.png").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    })
}

#[test]
fn decode_rgb() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^^tests/files/[^x]&[^t].+2c\d\d((\.png)|(\.jpg))").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    })
}

#[test]
fn decode_rgb_transparent() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^^tests/files/[^x]&[t].+2c\d\d((\.png)|(\.jpg))").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    })
}

#[test]
fn decode_rgba() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/[^x].+6a\d\d\.png$").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    })
}

#[test]
fn decode_indexed() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/[^x]&[^t].+3p\d\d\.png$").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    })
}

#[test]
fn decode_indexed_transparent() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/[^x]&[t].+3p\d\d\.png$").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let image = Decoder::new(path, file).decode().unwrap();

        assert_ne!(image.data().len(), 0);
        assert_ne!(image.size(), (0, 0));
    })
}

#[test]
fn decode_corrupted() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/x.+\d\d\.png$").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let d = Decoder::new(path, file);

        let img = d.decode();
        assert!(img.is_err());
    })
}

#[test]
fn decode_webp() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/.+.webp$").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let d = Decoder::new(path, file);

        let img = d.decode().unwrap();
        println!("{:?}", img.size());
        println!("{:?}", img.data().len());

        assert_ne!(img.data().len(), 0);
        assert_ne!(img.size(), (0, 0));
    })
}

#[test]
fn decode_avif() {
    let files: Vec<path::PathBuf> = fs::read_dir("tests/files/")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();
            entry.path()
        })
        .filter(|path| {
            let re = Regex::new(r"^tests/files/.+.avif$").unwrap();
            re.is_match(path.to_str().unwrap_or(""))
        })
        .collect();

    files.iter().for_each(|path| {
        println!("{path:?}");
        let file = fs::File::open(path).unwrap();
        let d = Decoder::new(path, file);

        let img = d.decode().unwrap();
        println!("{:?}", img.size());
        println!("{:?}", img.data().len());

        assert_ne!(img.data().len(), 0);
        assert_ne!(img.size(), (0, 0));
    })
}
