use super::*;

#[test]
fn find_common_path() {
    let paths = vec![
        PathBuf::from("/path/to/image1.jpg"),
        PathBuf::from("/path/to/image2.jpg"),
        PathBuf::from("/path/to/image3.jpg"),
    ];

    let common_path = get_common_path(&paths);
    assert_eq!(common_path, Some(PathBuf::from("/path/to")));

    let paths = vec![
        PathBuf::from("/path/to/test/image1.jpg"),
        PathBuf::from("/path/to/image2.jpg"),
        PathBuf::from("/path/to/image3.jpg"),
    ];

    let common_path = get_common_path(&paths);
    assert_eq!(common_path, Some(PathBuf::from("/path/to")));

    let paths = vec![
        PathBuf::from("/path/to/test/image1.jpg"),
        PathBuf::from("/path/image2.jpg"),
        PathBuf::from("/path/to/image3.jpg"),
    ];

    let common_path = get_common_path(&paths);
    assert_eq!(common_path, Some(PathBuf::from("/path")));
}
