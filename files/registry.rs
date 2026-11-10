//! Windows registry read helpers for Affinity Photo 2.

pub const APP_KEY: &str = "Software\\Affinity\\Affinity Photo 2";
pub const UNINSTALL_KEY: &str = "Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\Affinity Photo 2";

pub struct RegValue<'a> {
    pub name: &'a str,
    pub value: String,
}

pub fn app_key_values() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "InstallPath", value: format!("%PROGRAMFILES%\\Affinity Photo 2") },
        RegValue { name: "Version",     value: String::from("v2.5.21") },
        RegValue { name: "Channel",     value: String::from("stable") },
    ]
}

pub fn uninstall_strings() -> Vec<RegValue<'static>> {
    vec![
        RegValue { name: "DisplayName",     value: String::from("Affinity Photo 2") },
        RegValue { name: "Publisher",       value: String::from("Affinity") },
        RegValue { name: "UninstallString", value: format!("\"%PROGRAMFILES%\\Affinity Photo 2\\uninstall.exe\"") },
    ]
}
