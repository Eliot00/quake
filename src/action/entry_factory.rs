use quake_core::entry::entry_defines::EntryDefines;
use quake_core::entry::{EntryDefine, EntryNodeInfo};
use std::fs;
use std::path::PathBuf;

pub fn entries_define_from_path(config_path: &PathBuf) -> Vec<EntryDefine> {
    let entries_str = fs::read_to_string(config_path).expect("cannot read entries-define.yaml");
    let entries: EntryDefines = serde_yaml::from_str(&*entries_str).unwrap();

    entries.entries
}

pub fn entry_info_from_path(entry_info_path: &PathBuf) -> EntryNodeInfo {
    if !entry_info_path.exists() {
        let info = EntryNodeInfo::default();
        fs::write(
            entry_info_path,
            serde_yaml::to_string(&info).expect("cannot serial"),
        )
        .expect("cannot write to file");

        return info;
    }

    let text = fs::read_to_string(&entry_info_path).expect("cannot read entry-info.yaml");
    let entry_info = serde_yaml::from_str(&*text).unwrap();
    entry_info
}
