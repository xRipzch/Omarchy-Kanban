# Omarchy Kanban Board

A simple, lightweight TUI kanban board build for Omarchy.

## Features

- **Three-column kanban board**: To Do, In Progress, Done
- **Tag system**: Categorize tasks with tags (urgent, bug, feature)
- **Color-coded tasks**: Visual distinction based on tags
- **Vim-style navigation**: Use hjkl or arrow keys
- **Persistent storage**: Tasks are saved automatically to `~/.local/share/omarchy-kanban/board.json`

## Installation

### From AUR

```bash
# Using yay
yay -S omarchy-kanban-git

# Using paru
paru -S omarchy-kanban-git

# Manual with makepkg
git clone https://aur.archlinux.org/omarchy-kanban-git.git
cd omarchy-kanban-git
makepkg -si
```

### From Source

Requires Rust toolchain (rustc, cargo):

```bash
git clone https://github.com/xRipzch/Omarchy-Kanban.git
cd Omarchy-Kanban
cargo build --release
sudo install -Dm755 target/release/omarchy-kanban /usr/local/bin/omarchy-kanban
```

## Usage

Run the application:

```bash
omarchy-kanban
```

### Keyboard Shortcuts

#### Normal Mode
- **h/j/k/l** or **Arrow keys** - Navigate between columns and tasks
- **a** - Add a new task to the selected column
- **t** - Add a tag to the selected task
- **m** - Move the selected task to the next column (right)
- **d** - Delete the selected task
- **q** - Quit the application

#### Input Mode
- **Enter** - Submit input (add task/tag)
- **Esc** - Cancel input
- **Backspace** - Delete character

### Tags

The following tags have special colors:
- **urgent** - Red
- **bug** - Yellow
- **feature** - Green
- Other tags - White

## Data Storage

Tasks are automatically saved to:
```
~/.local/share/omarchy-kanban/board.json
```


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

For bugs and feature requests, please create an issue on the [GitHub repository](https://github.com/xRipzch/Omarchy-Kanban/issues).

---
