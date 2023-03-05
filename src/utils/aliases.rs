use crate::constants::ERR_NO_LXINFO;
use crate::hyprland;
use lxinfo::info;

/// Replaces `find` with `replace` if found.
fn replace_if_present(content: &mut String, find: &str, replace: &str) {
    if content.contains(find) {
        *content = content.replace(find, replace);
    }
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
    if is_feature_active!("hyprland") {
        let data = hyprland::get_data();
        replace_if_present(&mut content, "%hl_workspace%", &data.workspace.to_string());
        replace_if_present(&mut content, "%hl_window%", &data.window);
        if !has_alias_chars(&content) {
            // Success
            return content;
        }
    }

    if is_feature_active!("systemd") {
        if let Some(info) = info::get_system_information() {
            replace_if_present(&mut content, "%username%", &info.username);
            replace_if_present(&mut content, "%hostname%", &info.hostname);
            replace_if_present(&mut content, "%shell%", &info.shell);
            replace_if_present(&mut content, "%kernel%", &info.kernel);
            replace_if_present(&mut content, "%used_mem%", &info.used_mem);
            replace_if_present(&mut content, "%distro_id%", &info.distro_id);
            replace_if_present(&mut content, "%total_mem%", &info.total_mem);
            replace_if_present(&mut content, "%cached_mem%", &info.cached_mem);
            replace_if_present(&mut content, "%available_mem%", &info.available_mem);
            replace_if_present(&mut content, "%distro%", &info.distro_name);
            replace_if_present(&mut content, "%distro_build_id%", &info.distro_build_id);
            if !has_alias_chars(&content) {
                // Success
                return content;
            }
        } else {
            panic!("{}", ERR_NO_LXINFO);
        }
    }

    execute!(&content)
}
