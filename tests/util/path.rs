use std::io;
use std::path::PathBuf;

pub fn test_root() -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.join("tests")
}

pub fn fixtures() -> io::Result<PathBuf> {
    Ok(test_root().join("fixtures"))
}

pub fn fixture(name: &str) -> io::Result<PathBuf> {
    Ok(fixtures()?.join(name))
}
