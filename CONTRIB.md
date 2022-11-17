# Contribution
## Style
Look around the project to get a view on how the code is structured, then adapt to that.

One example is with `expect(1)`, **always** end your message with `\n`.

### Why?
Because it makes the custom message at the first line, whereas the stack-trace is on the line below it. Making the whole thing easier to read.

Also make sure you format your code accordingly. The config I use for it is just vanilla LSP with Rust. Config is [here](https://github.com/vars1ty/NeoRS).

## Error Handling
If the possibility of an error happening is super low (like never), then you may use `unwrap()`. An example of this can be seen [here](https://github.com/vars1ty/HybridBar/blob/main/src/loop.rs#L39).

If it may happen frequently however, then please use `expect(1)` with an informative message. Or if it makes sense; guard the logic and try to make it not panic-crash the application.

## Caching
If you are just accessing a value once or twice upon startup and **it's not heavy to do so**; don't cache it.

On the other hand, if it's heavy (an example of this could be seen at issue #13) - Try and cache what you are accessing and go from there.

## Widgets
When adding widgets, make sure you follow the relatively simple widget structure that already exists.

Also make sure you document your widgets behavior, keys (if any), etc.

## Breaking Changes
If your commit contains breaking changes, or ones that are unstable, please use the `new-release` branch.

### Why?
Because `new-release` isn't cloned when the user uses `hybrid-bar-git`, therefore making it a better spot for testing.

## I found some bad code, can I redo it?
You may submit a PR with **your** new code, just make sure it's easily readable and doesn't have worse performance than the original code.
