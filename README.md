# Keydio
A fun Windows app that plays audio files when you press certain keys and common key combos, which definitely does not get annoying the more you use it! 

Written in Rust. because crustaceans are cool.
# Usage
Keydio can play a unique sound for each of the following keys/combos:
- **Enter** (*enter.mp3*)
- **Backspace** (*back.mp3*)
- **Space** (*space.mp3*)
- **Print Screen** (*prntscrn.mp3*)
- **Ctrl+S** (*save.mp3*)
- **Ctrl+Z** (*undo.mp3*)
- **Ctrl+C** (*copy.mp3*)
- **Ctrl+V** (*paste.mp3*)
- **Alt+F4** (*close.mp3*)
- **Ctrl+Alt+Del** (*panic.mp3*)
- **Ctrl+Shift+Esc** (*taskman.mp3*)

Pick a sound file that you want to use for a certain key or combo, give it the right filename according to the list above, then place it in the audio folder.

To change the sounds for each key, simply replace the audio for the key you want with a new one, and give it the same filename as the last one.

To remove a sound for a key or combo, simply delete its audio file. You can place it back later if you want to.

To exit Keydio, press **Ctrl+Alt+Q**. You can even change the sound that plays as it exits (*exit.mp3*)!

**Note:** Sounds must not be any longer than 4 seconds.
# Building
To build Keydio, first install the Rust toolchain.

## Windows
Download Rustup [here](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe), then follow the instructions.

## Linux
Run the following command in your terminal.

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
**Arch-based distros:** It's recommended to install Rustup with your package manager. For example:
```
sudo pacman -S rustup
```

---

After that, run the following lines in a terminal.

```
git clone https://github.com/mango-ki/keydio.git
cd keydio
cargo build
cargo run
```


# Attribution
App logo by Freepik.