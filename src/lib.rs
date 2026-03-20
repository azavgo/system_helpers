use std::env;
use crossclip::{Clipboard, SystemClipboard};
use std::{thread, time::Duration};

//Custom error
#[derive(Debug)]
pub enum SystemHelpersError { 
    ClipboardError(crossclip::ClipboardError),   
}

impl From<crossclip::ClipboardError> for SystemHelpersError {
    fn from(error: crossclip::ClipboardError) -> Self {
        SystemHelpersError::ClipboardError(error)
    }
}


//Types of the currently supported operating systems. Only linux and macos are supported
//const OS_TYPES: [&str; 3] = ["linux", "macos", "other"];
pub enum OsTypes {
    Linux, 
    Macos, 
    Windows, 
    FreeBSD,
    Other,
}

//Checks the installed OS
pub fn which_os() -> OsTypes {
    let os = env::consts::OS; 
    match os {
        "linux"   => OsTypes::Linux, 
        "macos"   => OsTypes::Macos,
        "windows" => OsTypes::Windows, 
        "freebsd" => OsTypes::FreeBSD,
        _         => OsTypes::Other, 
    }
}

//Copy utf8 string to system clipboard using https://crates.io/crates/crossclip 
//crossclip has been tested for Linux, MacOS, Windows, and FreeBSD
pub fn copy_to_clipboard(s: String) -> Result<(), SystemHelpersError> {
    let clipboard = SystemClipboard::new()?;
    clipboard.set_string_contents(s)?;
    thread::sleep(Duration::from_millis(10000));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(2, 2);
    }
}
