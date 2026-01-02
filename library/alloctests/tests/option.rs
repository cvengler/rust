use std::bstr::{ByteStr, ByteString};
use std::ffi::{CStr, CString, OsStr, OsString};
use std::path::{Path, PathBuf};

#[test]
fn test_to_owned() {
    let _str: Option<String> = Some(<&str>::default()).map_owned();
    let _byte_str: Option<ByteString> = Some(<&ByteStr>::default()).map_owned();
    let _c_str: Option<CString> = Some(<&CStr>::default()).map_owned();
    let _os_str: Option<OsString> = Some(<&OsStr>::default()).map_owned();
    let _path: Option<PathBuf> = Some(Path::new("")).map_owned();
    let _arr: Option<Vec<u8>> = Some([].as_slice()).map_owned();
}
