use crate::config::SYSINFO;

/// Replaces `find` with `replace` if found.
fn replace_if_present(content: &mut String, find: &str, replace: &str, found_any: &mut bool) {
    if content.contains(find) {
        *content = content.replace(find, replace);
        *found_any = true;
    }
}

/// Checks if the `content` contains any of the built-in aliases, then replaces it with the real
/// value.
pub fn use_aliases(content: &str) -> String {
    // TODO: Clean this up.
    let mut found_any = false;
    let mut content = content.to_owned();
    replace_if_present(
        &mut content,
        "%username%",
        &SYSINFO.username,
        &mut found_any,
    );
    replace_if_present(
        &mut content,
        "%hostname%",
        &SYSINFO.hostname,
        &mut found_any,
    );
    replace_if_present(&mut content, "%shell%", &SYSINFO.shell, &mut found_any);
    replace_if_present(&mut content, "%kernel%", &SYSINFO.kernel, &mut found_any);
    replace_if_present(
        &mut content,
        "%used_mem%",
        &SYSINFO.used_mem,
        &mut found_any,
    );
    replace_if_present(
        &mut content,
        "%distro_id%",
        &SYSINFO.distro_id,
        &mut found_any,
    );
    replace_if_present(
        &mut content,
        "%total_mem%",
        &SYSINFO.total_mem,
        &mut found_any,
    );
    replace_if_present(
        &mut content,
        "%cached_mem%",
        &SYSINFO.cached_mem,
        &mut found_any,
    );
    replace_if_present(
        &mut content,
        "%available_mem%",
        &SYSINFO.available_mem,
        &mut found_any,
    );
    replace_if_present(
        &mut content,
        "%distro%",
        &SYSINFO.distro_name,
        &mut found_any,
    );
    replace_if_present(
        &mut content,
        "%distro_build_id%",
        &SYSINFO.distro_build_id,
        &mut found_any,
    );

    if !found_any {
        // Couldn't find any aliases present, run execute.
        return execute!(&content);
    }

    content
}
