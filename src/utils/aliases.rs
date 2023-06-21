use crate::{constants::ERR_NO_LXINFO, utils::hyprland::HyprlandData};
use lxinfo::info;

/// Replaces `find` with `replace`.
fn replace(content: &mut String, find: &str, replace: &str) {
    *content = content.replace(find, replace);
}

/// Checks if the first and last character in `content` is `%`.
fn has_alias_chars(content: &str) -> bool {
    if content.len() <= 4 {
        return false;
    }

    let mut chars = content.chars();
    chars.next().unwrap() == '%' && chars.last().unwrap() == '%'
}

/// Checks if the `content` contains any of the built-in aliases, then replaces it with the real
/// value.
pub fn use_aliases(content: &str) -> String {
    if !has_alias_chars(content) {
        // Not an alias, execute and return.
        return execute!(content);
    }

    let mut content = content.to_owned();
    let data = HyprlandData::get_data();
    replace(&mut content, "%hl_workspace%", &data.workspace.to_string());
    replace(&mut content, "%hl_window%", &data.window);
    if !has_alias_chars(&content) {
        // Success
        return content;
    }

    if let Some(info) = info::get_system_information() {
        replace(&mut content, "%username%", &info.username);
        replace(&mut content, "%hostname%", &info.hostname);
        replace(&mut content, "%shell%", &info.shell);
        replace(&mut content, "%kernel%", &info.kernel);
        replace(&mut content, "%used_mem%", &info.used_mem);
        replace(&mut content, "%distro_id%", &info.distro_id);
        replace(&mut content, "%total_mem%", &info.total_mem);
        replace(&mut content, "%cached_mem%", &info.cached_mem);
        replace(&mut content, "%available_mem%", &info.available_mem);
        replace(&mut content, "%distro%", &info.distro_name);
        replace(&mut content, "%distro_build_id%", &info.distro_build_id);
        if !has_alias_chars(&content) {
            // Success
            return content;
        }
    }

    execute!(&content)
}
