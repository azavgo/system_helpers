use std::env;

//Types of the currently supported operating systems. Only linux and macos are supported
//const OS_TYPES: [&str; 3] = ["linux", "macos", "other"];
pub enum OsTypes {
    Linux, 
    Macos, 
    Other,
}

//Checks the installed OS
pub fn which_os() -> OsTypes {
    let os = env::consts::OS; 
    match os {
        "linux" => OsTypes::Linux, 
        "macos" => OsTypes::Macos, 
        _       => OsTypes::Other, 
    }
}

//Copy utf8 string to linux clipboard
pub fn clipboard_copy_linux() -> String {
    match which_os() {
        OsTypes::Linux => {"linux".to_string()}, 
        OsTypes::Macos => {"macos".to_string()},
        OsTypes::Other => {"other".to_string()},
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clipboard_copy_linux() {
        assert_eq!(clipboard_copy_linux(), "linux".to_string());
    }
}
