# Windows Clicker

A small Windows mouse and keyboard auto-clicker written in Rust.

## Download

Download `windows-clicker.exe` from the GitHub Releases page once a release is
published. The executable is not committed to git; it is built locally and
uploaded as a release asset.

## Current Scope

- Windows GUI executable with English and Chinese UI text.
- Mouse hold-to-repeat for left, right, and middle buttons.
- Keyboard hold-to-repeat for one configured key.
- Global hotkeys for mouse arm, keyboard arm, and emergency stop.
- User-friendly speed presets such as `10 / sec`, plus custom interval ms.

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

- Language: choose `English` or `中文`.
- Mouse button: choose `Left`, `Right`, or `Middle`.
- Mouse speed: choose a preset such as `1 / sec`, `5 / sec`, or `10 / sec`.
- Mouse custom interval ms: optional advanced override. Leave it blank to use
  the speed preset. Minimum: `25`.
- Keyboard key: examples include `Space`, `Enter`, `Esc`, `A`, `7`, `J`, `F1`,
  `F12`, `Left`, `Right`, `Up`, and `Down`.
- Keyboard speed: choose a preset such as `1 / sec`, `5 / sec`, or `10 / sec`.
- Keyboard custom interval ms: optional advanced override. Leave it blank to use
  the speed preset. Minimum: `25`.
- `F6`: arm or disarm mouse hold-to-repeat.
- `F7`: arm or disarm keyboard hold-to-repeat.
- `F8`: emergency stop for both clickers.

When mouse hold-to-repeat is armed, holding the configured mouse button repeats
clicks at the configured speed. Releasing the button stops the repeat while
keeping mouse mode armed.

When keyboard hold-to-repeat is armed, holding the configured key repeats that
key at the configured speed. Releasing the key stops the repeat while keeping
keyboard mode armed.

The window must stay open while the clickers are running.

## Windows Security Notes

Windows Firewall does not normally delete local executables; it controls network
access. This app does not open network sockets.

Microsoft Defender or SmartScreen may still warn about a new unsigned executable,
especially because this tool sends synthetic mouse and keyboard input. The app
does not hide itself, install startup entries, or bypass system protections. For
release checks, compare the downloaded file's SHA256 hash with the value printed
by the release build.

## Game Compatibility

Some games, fullscreen modes, elevated windows, and anti-cheat/input-filtering
systems ignore normal Windows synthetic input. This app intentionally does not
bypass those protections.

If input works in Notepad but not in a game, try:

- Run the app as administrator only when the target game also runs as
  administrator.
- Prefer borderless-windowed or windowed mode over exclusive fullscreen.
- Check the game's keybind settings for raw input or controller-only input
  modes.

If the game still ignores it, that target is outside this app's supported
boundary.

## License

MIT
