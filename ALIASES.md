# Aliases
These aliases are only valid for commands (+ tooltip commands).
***
- `%username%` - Username
- `%hostname%` - Hostname
- `%shell%` - Active Session Shell
- `%distro%` - Distribution name
- `%distro_id%` - Distribution ID, for example: `arch`
- `%distro_build_id%` - Distribution Build ID, for example `rolling`
- `%total_mem%` - Total amount of installed memory (in GB)
- `%cached_mem%` - Total amount of cached memory (in GB)
- `%available_mem%` - Total amount of available memory (in GB)
- `%used_mem%` - Total amount of used memory (in GB)

## I can just use `whoami`, why all of this?
You may use completely dynamic commands like `whoami` if you want, the benefit over aliases though are:

1. Lower overhead due to being cached and retrieved via `libc`, rather than expensive commands
2. A lot faster to process
3. Cached at startup, then reused afterwards
