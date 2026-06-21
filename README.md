# Windows Clicker

A small Windows mouse and keyboard auto-clicker written in Rust.

## Download

Download `windows-clicker.exe` from the GitHub Releases page once a release is
published. The executable is not committed to git; it is built locally and
uploaded as a release asset.

## Current Scope

- Windows GUI executable.
- Mouse auto-clicking for left, right, and middle buttons.
- Keyboard auto-clicking for one configured key.
- Global hotkeys for mouse toggle, keyboard toggle, and emergency stop.
- Conservative minimum interval to avoid saturating the desktop.

This tool uses normal Windows input injection APIs. It does not bypass game
anti-cheat systems, elevated-window isolation, or applications that reject
synthetic input.

## Development

Core validation logic can be tested from WSL:

```bash
cargo test
```

Build the Windows executable from Windows PowerShell.

If the project is under WSL, use the helper script. It copies the source to a
Windows-local temp directory because the GNU Windows linker cannot always write
release artifacts directly to `\\wsl.localhost\...` paths.

```powershell
.\scripts\build-release.ps1
```

The executable will be copied to `dist\windows-clicker.exe`.

## Usage

- Mouse button: choose `Left`, `Right`, or `Middle`.
- Mouse interval ms: repeat delay for mouse clicks. Minimum: `25`.
- Keyboard key: examples include `Space`, `Enter`, `Esc`, `A`, `7`, `F1`,
  `F12`, `Left`, `Right`, `Up`, and `Down`.
- Keyboard interval ms: repeat delay for keyboard presses. Minimum: `25`.
- `F6`: start or stop mouse auto-clicking.
- `F7`: start or stop keyboard auto-clicking.
- `F8`: emergency stop for both clickers.

The window must stay open while the clickers are running.

## License

MIT
