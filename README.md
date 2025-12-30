# TUI Kanban Board

A simple, lightweight terminal-based kanban board built with Rust. Works on any Linux distribution.

<img width="1896" height="1030" alt="screenshot-2025-12-19_19-10-59" src="https://github.com/user-attachments/assets/359221a8-9e25-46a3-ac01-643b3e35b4d8" />
<img width="1896" height="1030" alt="screenshot-2025-12-19_19-11-17" src="https://github.com/user-attachments/assets/39e8f333-5797-4c4d-93a3-f3a05abd5b8e" />


## Features

- **Customizable columns**: Create, rename, and delete columns to match your workflow (default: To Do, In Progress, Testing, Done)
- **Multiple projects**: Organize tasks across different projects with easy switching (Ctrl+P)
- **Tag system**: Categorize tasks with tags (urgent, bug, feature, and more)
- **Color-coded tasks**: Visual distinction based on tags
- **Vim-style navigation**: Use hjkl or arrow keys
- **Task detail view**: Edit titles, add/remove tags, write multi-line descriptions
- **Bi-directional movement**: Move tasks forward and backward through columns
- **Persistent storage**: Tasks are saved automatically to `~/.config/tui-kanban/projects.json`
- **CI/CD**: Automated testing with GitHub Actions

## Installation

### From AUR (Arch-based distros)

```bash
# Using yay
yay -S tui-kanban-git

# Using paru
paru -S tui-kanban-git

# Manual with makepkg
git clone https://aur.archlinux.org/tui-kanban-git.git
cd tui-kanban-git
makepkg -si
```

### From Source

Requires Rust toolchain (rustc, cargo):

```bash
git clone https://github.com/xRipzch/TUI-Kanban.git
cd TUI-Kanban
cargo build --release
sudo install -Dm755 target/release/tui-kanban /usr/local/bin/tui-kanban
```

## Usage

Run the application:

```bash
tui-kanban
```

### Keyboard Shortcuts

#### Normal Mode
- **h/j/k/l** or **Arrow keys** - Navigate between columns and tasks
- **Enter** - Open task details
- **a** - Add a new task to the selected column
- **t** - Add a tag to the selected task
- **m** - Move task forward (TODO → IN PROGRESS → TESTING → DONE)
- **n** - Move task backward (DONE → TESTING → IN PROGRESS → TODO)
- **d** - Delete the selected task
- **Shift+C** - Add a new column
- **Shift+R** - Rename current column
- **Shift+D** - Delete current column (must be empty)
- **Ctrl+P** - Open project list
- **?** - Show help
- **q** - Quit the application

#### Editing
In any field while you are editing.
- **ctrl+e** - Open external editor ($EDITOR) 

#### Task Detail View
- **Tab or j/ Shift+Tab or k** - Switch between fields forwards/ backwards(Title, Tags, Description)
- **Enter** - Edit focused field
- **1-9** - Remove tag by number (when Tags field is focused)
- **Esc** - Close task detail view

#### Editing Title/Description
- **Enter** - Save title / Add newline in description
- **Esc** - Save description / Cancel title edit
- **Backspace** - Delete character

#### Project List
- **j/k** or **Arrow keys** - Navigate projects
- **Enter** - Select project
- **a** - Add new project
- **d** - Delete project
- **s** - Set selected project as default
- **Esc** - Close project list

### Tags

The following tags have special colors:
- **urgent** - Red (high priority)
- **security** - Light Red (security work)
- **bug** - Yellow (needs fixing)
- **feature** - Green (new feature)
- **performance** - Light Green (optimization)
- **enhancement** - Blue (improvement)
- **User** - Light Blue (user-facing work)
- **Dev** - Magenta (developer work)
- **documentation** - Cyan (documentation)
- **design** - Light Cyan (UI/UX work)
- **refactor** - Light Yellow (code quality)
- Other tags - White

## Data Storage

Projects and tasks are automatically saved to:
```
~/.config/tui-kanban/projects.json
```

If you're migrating from an older version, your data will be automatically migrated from the old location.

## Default Projects

TUI-Kanban supports setting a default project that opens automatically when you launch the application. There are two ways to set a default project:

### Global Default (Recommended for most users)

Set a project as default globally using the project list (Ctrl+P):
1. Press **Ctrl+P** to open the project list
2. Navigate to your desired project using **j/k** or arrow keys
3. Press **s** to set it as the default project

Your choice is saved to `~/.config/tui-kanban/config.json` and will apply everywhere.

### Directory-Specific Default

For advanced workflows where you work on multiple projects in different directories, you can create a `.tui-kanban-project` file in any directory:

```bash
# In your project directory
echo "MyProject" > .tui-kanban-project
```

When you run `tui-kanban` from that directory, it will automatically open "MyProject".

**Priority order:**
1. Directory-specific `.tui-kanban-project` file (if present in current directory)
2. Global default from `config.json` (set via 's' in project list)
3. First project in the list (default behavior)


https://github.com/user-attachments/assets/fa467298-e3c5-4770-b4b5-c40280f6f9ab

## Themes

TUI-Kanban supports multiple color themes to match your terminal preferences and improve readability. You can change the theme by editing your config file.

### Available Themes

- **high-contrast** (Default) - Bright, bold colors for maximum visibility on any terminal theme
- **classic** - Traditional color scheme with improved contrast over the original
- **solarized-dark** - Popular Solarized palette designed for reduced eye strain
- **gruvbox** - Warm, retro color scheme with earthy tones
- **nord** - Cool, arctic-inspired colors from the Nord palette

### Changing Your Theme

The theme can be changed from the UI by pressing `ctrl+t`.

Edit your configuration file at `~/.config/tui-kanban/config.json` (Linux/macOS) or `%APPDATA%\tui-kanban\config.json` (Windows):

```json
{
  "theme": "nord",
  "default_project": "Work"
}
```

Available theme names: `high-contrast`, `classic`, `solarized-dark`, `gruvbox`, `nord`

Changes take effect the next time you launch tui-kanban. If the theme name is invalid or not specified, it will default to `high-contrast`.

## Contributors

Special thanks to:
- [@mdetweil](https://github.com/mdetweil) - Customizable columns feature, tests, and CI/CD setup
- [@papitz](https://github.com/papitz) - Fixed Shift+T keybinding conflict preventing capital T input in text fields

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Support

For bugs and feature requests, please create an issue on the [GitHub repository](https://github.com/xRipzch/TUI-Kanban/issues).

---
