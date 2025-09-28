# 📝 Advanced Todo List - Terminal UI

A feature-rich, terminal-based todo list application built with Rust. This isn't just another simple todo app - it's a comprehensive task management system with advanced features like smart date parsing, themes, subtasks, and more.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Terminal](https://img.shields.io/badge/Terminal-%23054020?style=for-the-badge&logo=gnubash&logoColor=white)

## ✨ Features

### 🎯 Core Task Management
- **Interactive Terminal UI**: Clean, intuitive interface using ratatui
- **Task Creation**: Add tasks with natural language input
- **Task Completion**: Toggle task completion status with visual feedback
- **Task Deletion**: Delete individual tasks with confirmation prompts
- **Persistent Storage**: Tasks automatically saved to JSON file

### 🏷️ Advanced Organization
- **Priority Levels**: High, Medium, Low priorities with visual indicators
- **Due Dates**: Set and display due dates with overdue highlighting
- **Subtasks**: Hierarchical task organization with expandable/collapsible subtasks
- **Tags**: Organize tasks with hashtag-style tags (`#work`, `#urgent`, etc.)
- **Search & Filter**: Powerful search by content, tags, priority, or completion status

### 🧠 Smart Features
- **Natural Language Processing**: Parse dates from task descriptions
  - "buy milk tomorrow" → automatically sets due date
  - "call mom at 3pm" → sets specific time
  - "meeting next monday" → calculates correct date
- **Focus Mode**: Hide completed tasks to concentrate on pending work
- **Smart Date Recognition**: Context-aware parsing (morning vs evening times)

### 🎨 Customization & Themes
- **5 Built-in Themes**:
  - Catppuccin Mocha (default)
  - Catppuccin Latte  
  - Dracula
  - Gruvbox Dark
  - Nord
- **Custom Themes**: Define your own color schemes
- **Theme Cycling**: Switch themes on-the-fly with `t` key
- **Responsive Design**: Adapts to different terminal sizes

### 🔧 User Experience
- **Comprehensive Help System**: Built-in help with `h` or `F1`
- **Confirmation Dialogs**: Prevents accidental deletion of tasks
- **Keyboard-Driven**: Efficient navigation without mouse
- **Zoom Controls**: Adjust UI scaling with `+`/`-`
- **Multiple Input Modes**: Dedicated modes for different operations

## 🚀 Quick Start

### Prerequisites
- **Rust** (1.70+): Install from [rustup.rs](https://rustup.rs/)
- **Terminal**: Any modern terminal emulator

### Installation & Running

```bash
# Clone the repository
git clone https://github.com/Ashref-dev/todo.git
cd todo

# Build the project
cargo build --release

# Run with default theme
cargo run

# Run with specific theme
cargo run -- --theme dracula

# List available themes
cargo run -- --list-themes

# Show help
cargo run -- --help
```

## 🎮 Usage & Controls

### Navigation
- `↑`/`↓` - Move selection up/down
- `Enter` - Toggle task completion
- `Esc` - Return to normal mode from any input mode

### Task Management
- `a` - Add new task
- `s` - Add subtask to selected task  
- `d` - Delete selected task (with confirmation)
- `p` - Cycle task priority (High/Medium/Low)
- `D` - Set due date for selected task

### View & Organization
- `/` - Search/filter tasks
- `f` - Toggle focus mode (hide completed tasks)
- `C` - Clear completed tasks (with confirmation)
- `+`/`-` - Zoom in/out

### Themes & Help
- `t` - Cycle through available themes
- `h` or `F1` - Show/hide help dialog
- `q` - Quit application

### Smart Task Creation Examples
```
# Natural language dates
"Buy groceries tomorrow"
"Call dentist monday at 2pm" 
"Submit report by friday"

# Tags for organization
"Review PR #work #urgent"
"Plan vacation #personal #planning"

# Priorities automatically assigned based on keywords
"URGENT: Fix production bug" → High priority
"Maybe clean desk" → Low priority
```

## 🏗️ Project Structure

```
todo/
├── src/
│   ├── main.rs          # Entry point, CLI args, event loop
│   ├── app.rs           # Core application logic & state management
│   ├── ui.rs            # All rendering & UI components  
│   ├── task.rs          # Task data structure & persistence
│   └── theme.rs         # Theme system & color management
├── Cargo.toml           # Dependencies & project metadata
├── README.md            # This file
└── plan.md              # Development roadmap & feature checklist
```

### Module Responsibilities

**`main.rs`**
- Command-line argument parsing with `clap`
- Terminal setup and event loop
- Keyboard input handling and routing
- Theme initialization from CLI args

**`app.rs`**  
- Core `App` struct with all application state
- Smart date parsing with context awareness
- Search and filtering logic
- Task management operations (add, delete, complete)
- Theme cycling and help system

**`ui.rs`**
- All rendering functions using `ratatui`
- Help dialog with comprehensive documentation
- Theme-aware color application
- Responsive layout management

**`task.rs`**
- `Task` struct definition with all fields
- JSON serialization/deserialization  
- File I/O operations for persistence
- Task loading and saving utilities

**`theme.rs`**
- Complete theming system with 5 built-in themes
- `ThemeManager` for theme switching
- Custom theme loading from config directory
- Color palette definitions

## 🔧 Technical Details

### Dependencies
- **`ratatui`** (0.26.1) - Terminal user interface framework
- **`crossterm`** (0.27.0) - Cross-platform terminal manipulation
- **`chrono`** & **`chrono-english`** - Date/time parsing and manipulation
- **`clap`** (4.0) - Command-line argument parsing
- **`serde`** & **`serde_json`** - Data serialization
- **`dirs`** (5.0) - Cross-platform directory detection
- **`regex`** (1.0) - Pattern matching for smart parsing

### Data Storage
- Tasks stored in `tasks.json` in current directory
- Custom themes stored in `~/.config/todo/themes/` (or OS equivalent)
- Automatic backup and recovery of task data
- Human-readable JSON format for easy editing

### Performance
- Efficient rendering with minimal redraws
- Fast search with optimized filtering
- Lazy loading of custom themes
- Memory-efficient task storage

## 🎨 Theme Customization

Create custom themes by adding JSON files to your config directory:

```json
{
  "name": "My Custom Theme",
  "background": {"r": 40, "g": 42, "b": 54},
  "foreground": {"r": 248, "g": 248, "b": 242},
  "primary": {"r": 189, "g": 147, "b": 249},
  "secondary": {"r": 80, "g": 250, "b": 123},
  "accent": {"r": 255, "g": 184, "b": 108}
}
```

Themes are automatically loaded on startup and available via CLI or theme cycling.

## 🐛 Troubleshooting

### Common Issues
- **Terminal too small**: Minimum 80x24 characters recommended
- **Colors not showing**: Ensure terminal supports 256 colors or TrueColor
- **Tasks not saving**: Check write permissions in current directory
- **Themes not loading**: Verify JSON syntax in custom theme files

### Development
```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run tests
cargo test

# Check for issues
cargo clippy

# Format code  
cargo fmt
```

## 📋 Feature Roadmap

All major features are complete! The app includes:

- ✅ **Phase 1**: Core enhancements (priorities, due dates, subtasks)
- ✅ **Phase 2**: Advanced features (NLP, tags, search/filter) 
- ✅ **Phase 3**: UI/UX polish (themes, focus mode, confirmations)
- ✅ **Bonus**: Help system and theme utilities

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes following Rust best practices
4. Run tests and ensure no clippy warnings (`cargo test && cargo clippy`)
5. Commit your changes (`git commit -m 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [ratatui](https://github.com/ratatui-org/ratatui) - excellent TUI framework
- Inspired by modern terminal applications and productivity tools
- Color themes inspired by popular editor themes (Catppuccin, Dracula, etc.)
