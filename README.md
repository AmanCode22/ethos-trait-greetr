# Ethos Trait Rust Template

Template for creating Rust Hard Traits for Ethos.

Fork this repo to build your own native extension. CI workflows automatically build binaries for all 8 platforms (Linux, macOS, Windows, Termux) and submit your trait to Ethos Foundry.
After that go to repo settings-> actions-> general and allow read and write permission to workflow for release editing.

## Quick Start

1. **Fork this repo** → Rename it to `ethos-trait-<your-trait-name>`.
2. **Edit `manifest.json`** → Set your trait name, version, and description.
3. **Edit `Cargo.toml`** → Set trait name and also accordingly if required change version.
4. **(OPTIONAL) Create tags.txt → It helps someone find your trait by searching those specfic keywords.
3. **Write your code** → Edit `src/trait.rs` with your Rust functions.
4. **Tag a release** → Push a tag like `v1.0.0`.
5. **Wait for CI** → Workflows build binaries
6. **Open a PR to foundry** → You must open a pr to foundry for each version bump also, you can find manifest.json in releases use that for PR as that is different.
That's it. I review the PR. If it passes, your trait is live.

## Requirements

- **All 8 platforms must be supported or a issue must be opened if not.** Your code must compile on Linux, macOS, Windows, and Termux. If not supported then first open issue on foundry otherwise pr would be rejected.
- **No platform-specific code** unless wrapped in `#[cfg]` checks.
- **Public license.** MIT, Apache-2.0, etc. No proprietary licenses for Foundry traits.

## File Structure

- `src/trait.rs` → Your Rust code. Export functions here.
- `Cargo.toml` → Build config. Justedit to put your trait name or edit version.Don't change anything else unless you know Cargo.toml.
- `manifest.json` → Metadata (name, author, description). Used to generate `manifest.json` on foundry and used as manifest in forge. Use Ethos docs for reference on editing manifest.json.  Just do not add binary in manifest.json as foundry based hard traits does not need it.
- `.github/workflows/` → CI/CD. Don't edit unless adding new platforms.
- `LICENSE` → Any non proprietary license.
- (OPTIONAL)`tags.txt` → List of tags seperated by a newline that may help users find your trait.
## Writing Functions

Every function you want to use in Ethos must be exported in `src/trait.rs`.

Example:
```rust
use core::ffi::c_int;

#[no_mangle]
pub extern "C" fn add(a: c_int, b: c_int) -> c_int {
    a + b
}

```

Then list it in `manifest.json` under `functions`. The CI workflow reads this and generates the Foundry manifest. For more see Ethos docs on editing manifest. Every function must be wrapped in pub extern "C"  and no magle, to prevent function mangling to be compatible with ctypes. Also see docs for data type mapping also. Use of core::ffi is necessary for ctypes compatible binary




## Releasing

1. Push your changes.
2. Create a GitHub Release with a tag (e.g., `v1.0.0`).
3. CI triggers automatically.
4. Binaries upload to Releases.
5. Open PR a on `ethos-foundry` with your manifest(get it from releases of each release as it is not same as ones you created here).

## Updating

For example to release v2.0.0:
1. Update code.
2. Tag `v2.0.0`.
3. CI opens a new PR to Foundry with updated binaries.

## Rules
- **Trait name** in `manifest.json` must match the repo name (e.g., `ethos-trait-mymath` → name: `mymath`. Same name should also be used in `CMakeLists.txt` first line as project name.
- **No binaries in repo.** Only source code. Binaries go to GitHub Releases.
- **All platforms required.** If your code doesn't compile on Windows or Termux, then open issue on foundry first otherwise pr would be rejected as ethos is aimed to be mostly cross platform.

## Links
- [Ethos Foundry](https://github.com/AmanCode22/ethos-foundry)
- [Ethos Docs](https://github.com/AmanCode22/ethos-lang/blob/main/DOCS.md)
- [Forge](https://github.com/AmanCode22/forge)

## License

MIT. Free to use, free to contribute.

---

Built by Aman (Class 9, India). Solo project. Questions? Open an issue.
