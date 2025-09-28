use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<Color> for SerializableColor {
    fn from(color: Color) -> Self {
        match color {
            Color::Rgb(r, g, b) => SerializableColor { r, g, b },
            _ => SerializableColor {
                r: 255,
                g: 255,
                b: 255,
            }, // Default to white for other colors
        }
    }
}

impl From<SerializableColor> for Color {
    fn from(val: SerializableColor) -> Self {
        Color::Rgb(val.r, val.g, val.b)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableTheme {
    pub name: String,
    pub background: SerializableColor,
    pub foreground: SerializableColor,
    pub primary: SerializableColor,
    pub secondary: SerializableColor,
    pub accent: SerializableColor,
    pub surface0: SerializableColor,
    pub surface1: SerializableColor,
    pub surface2: SerializableColor,
    pub text: SerializableColor,
    pub subtext: SerializableColor,
    pub red: SerializableColor,
    pub yellow: SerializableColor,
    pub green: SerializableColor,
    pub blue: SerializableColor,
    pub mauve: SerializableColor,
    pub lavender: SerializableColor,
}

#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub surface0: Color,
    pub surface1: Color,
    pub surface2: Color,
    pub text: Color,
    pub subtext: Color,
    pub red: Color,
    pub yellow: Color,
    pub green: Color,
    pub blue: Color,
    pub mauve: Color,
    pub lavender: Color,
}

impl From<SerializableTheme> for Theme {
    fn from(theme: SerializableTheme) -> Self {
        Theme {
            name: theme.name,
            background: theme.background.into(),
            foreground: theme.foreground.into(),
            primary: theme.primary.into(),
            secondary: theme.secondary.into(),
            accent: theme.accent.into(),
            surface0: theme.surface0.into(),
            surface1: theme.surface1.into(),
            surface2: theme.surface2.into(),
            text: theme.text.into(),
            subtext: theme.subtext.into(),
            red: theme.red.into(),
            yellow: theme.yellow.into(),
            green: theme.green.into(),
            blue: theme.blue.into(),
            mauve: theme.mauve.into(),
            lavender: theme.lavender.into(),
        }
    }
}

impl From<Theme> for SerializableTheme {
    fn from(theme: Theme) -> Self {
        SerializableTheme {
            name: theme.name,
            background: theme.background.into(),
            foreground: theme.foreground.into(),
            primary: theme.primary.into(),
            secondary: theme.secondary.into(),
            accent: theme.accent.into(),
            surface0: theme.surface0.into(),
            surface1: theme.surface1.into(),
            surface2: theme.surface2.into(),
            text: theme.text.into(),
            subtext: theme.subtext.into(),
            red: theme.red.into(),
            yellow: theme.yellow.into(),
            green: theme.green.into(),
            blue: theme.blue.into(),
            mauve: theme.mauve.into(),
            lavender: theme.lavender.into(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::catppuccin_mocha()
    }
}

impl Theme {
    pub fn catppuccin_mocha() -> Self {
        Theme {
            name: "Catppuccin Mocha".to_string(),
            background: Color::Rgb(17, 17, 27),    // Crust
            foreground: Color::Rgb(205, 214, 244), // Text
            primary: Color::Rgb(203, 166, 247),    // Mauve
            secondary: Color::Rgb(180, 190, 254),  // Lavender
            accent: Color::Rgb(137, 220, 235),     // Sky
            surface0: Color::Rgb(49, 50, 68),      // Surface0
            surface1: Color::Rgb(69, 71, 90),      // Surface1
            surface2: Color::Rgb(88, 91, 112),     // Surface2
            text: Color::Rgb(205, 214, 244),       // Text
            subtext: Color::Rgb(186, 194, 222),    // Subtext1
            red: Color::Rgb(243, 139, 168),        // Red
            yellow: Color::Rgb(250, 179, 135),     // Peach
            green: Color::Rgb(166, 227, 161),      // Green
            blue: Color::Rgb(137, 220, 235),       // Sky
            mauve: Color::Rgb(203, 166, 247),      // Mauve
            lavender: Color::Rgb(180, 190, 254),   // Lavender
        }
    }

    pub fn catppuccin_latte() -> Self {
        Theme {
            name: "Catppuccin Latte".to_string(),
            background: Color::Rgb(239, 241, 245), // Base
            foreground: Color::Rgb(76, 79, 105),   // Text
            primary: Color::Rgb(136, 57, 239),     // Mauve
            secondary: Color::Rgb(114, 135, 253),  // Lavender
            accent: Color::Rgb(4, 165, 229),       // Sky
            surface0: Color::Rgb(204, 208, 218),   // Surface0
            surface1: Color::Rgb(188, 192, 204),   // Surface1
            surface2: Color::Rgb(172, 176, 190),   // Surface2
            text: Color::Rgb(76, 79, 105),         // Text
            subtext: Color::Rgb(92, 95, 119),      // Subtext1
            red: Color::Rgb(210, 15, 57),          // Red
            yellow: Color::Rgb(254, 100, 11),      // Peach
            green: Color::Rgb(64, 160, 43),        // Green
            blue: Color::Rgb(4, 165, 229),         // Sky
            mauve: Color::Rgb(136, 57, 239),       // Mauve
            lavender: Color::Rgb(114, 135, 253),   // Lavender
        }
    }

    pub fn dracula() -> Self {
        Theme {
            name: "Dracula".to_string(),
            background: Color::Rgb(40, 42, 54),    // Background
            foreground: Color::Rgb(248, 248, 242), // Foreground
            primary: Color::Rgb(189, 147, 249),    // Purple
            secondary: Color::Rgb(80, 250, 123),   // Green
            accent: Color::Rgb(255, 184, 108),     // Orange
            surface0: Color::Rgb(68, 71, 90),      // Current Line
            surface1: Color::Rgb(98, 114, 164),    // Selection
            surface2: Color::Rgb(139, 233, 253),   // Cyan
            text: Color::Rgb(248, 248, 242),       // Foreground
            subtext: Color::Rgb(98, 114, 164),     // Comment
            red: Color::Rgb(255, 85, 85),          // Red
            yellow: Color::Rgb(241, 250, 140),     // Yellow
            green: Color::Rgb(80, 250, 123),       // Green
            blue: Color::Rgb(139, 233, 253),       // Cyan
            mauve: Color::Rgb(189, 147, 249),      // Purple
            lavender: Color::Rgb(139, 233, 253),   // Cyan
        }
    }

    pub fn gruvbox_dark() -> Self {
        Theme {
            name: "Gruvbox Dark".to_string(),
            background: Color::Rgb(40, 40, 40),    // bg0
            foreground: Color::Rgb(235, 219, 178), // fg
            primary: Color::Rgb(211, 134, 155),    // purple
            secondary: Color::Rgb(142, 192, 124),  // green
            accent: Color::Rgb(254, 128, 25),      // orange
            surface0: Color::Rgb(60, 56, 54),      // bg1
            surface1: Color::Rgb(80, 73, 69),      // bg2
            surface2: Color::Rgb(102, 92, 84),     // bg3
            text: Color::Rgb(235, 219, 178),       // fg
            subtext: Color::Rgb(168, 153, 132),    // fg2
            red: Color::Rgb(251, 73, 52),          // red
            yellow: Color::Rgb(250, 189, 47),      // yellow
            green: Color::Rgb(142, 192, 124),      // green
            blue: Color::Rgb(131, 165, 152),       // aqua
            mauve: Color::Rgb(211, 134, 155),      // purple
            lavender: Color::Rgb(131, 165, 152),   // aqua
        }
    }

    pub fn nord() -> Self {
        Theme {
            name: "Nord".to_string(),
            background: Color::Rgb(46, 52, 64),    // Polar Night 0
            foreground: Color::Rgb(236, 239, 244), // Snow Storm 3
            primary: Color::Rgb(129, 161, 193),    // Frost 3
            secondary: Color::Rgb(136, 192, 208),  // Frost 2
            accent: Color::Rgb(163, 190, 140),     // Aurora 2
            surface0: Color::Rgb(59, 66, 82),      // Polar Night 1
            surface1: Color::Rgb(67, 76, 94),      // Polar Night 2
            surface2: Color::Rgb(76, 86, 106),     // Polar Night 3
            text: Color::Rgb(236, 239, 244),       // Snow Storm 3
            subtext: Color::Rgb(229, 233, 240),    // Snow Storm 2
            red: Color::Rgb(191, 97, 106),         // Aurora 0
            yellow: Color::Rgb(235, 203, 139),     // Aurora 1
            green: Color::Rgb(163, 190, 140),      // Aurora 2
            blue: Color::Rgb(129, 161, 193),       // Frost 3
            mauve: Color::Rgb(180, 142, 173),      // Aurora 4
            lavender: Color::Rgb(136, 192, 208),   // Frost 2
        }
    }

    pub fn get_builtin_themes() -> HashMap<String, Theme> {
        let mut themes = HashMap::new();

        themes.insert("catppuccin-mocha".to_string(), Self::catppuccin_mocha());
        themes.insert("catppuccin-latte".to_string(), Self::catppuccin_latte());
        themes.insert("dracula".to_string(), Self::dracula());
        themes.insert("gruvbox-dark".to_string(), Self::gruvbox_dark());
        themes.insert("nord".to_string(), Self::nord());

        themes
    }
}

#[derive(Debug, Clone)]
pub struct ThemeManager {
    current_theme: Theme,
    available_themes: HashMap<String, Theme>,
}

impl Default for ThemeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut manager = ThemeManager {
            current_theme: Theme::default(),
            available_themes: Theme::get_builtin_themes(),
        };

        // Load custom themes from config directory
        if let Err(e) = manager.load_custom_themes() {
            eprintln!("Warning: Failed to load custom themes: {}", e);
        }

        manager
    }

    pub fn get_current_theme(&self) -> &Theme {
        &self.current_theme
    }

    pub fn set_theme(&mut self, theme_name: &str) -> Result<(), String> {
        if let Some(theme) = self.available_themes.get(theme_name) {
            self.current_theme = theme.clone();
            Ok(())
        } else {
            Err(format!("Theme '{}' not found", theme_name))
        }
    }

    pub fn get_available_themes(&self) -> Vec<String> {
        self.available_themes.keys().cloned().collect()
    }

    pub fn get_theme_by_key(&self, key: &str) -> Option<&Theme> {
        self.available_themes.get(key)
    }

    pub fn load_custom_themes(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;

        // Create config directory if it doesn't exist
        let config_dir = self.get_config_dir()?;
        let themes_dir = config_dir.join("themes");

        if !themes_dir.exists() {
            fs::create_dir_all(&themes_dir)?;
            return Ok(());
        }

        // Load all .json files from themes directory
        for entry in fs::read_dir(&themes_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                match fs::read_to_string(&path) {
                    Ok(content) => match serde_json::from_str::<SerializableTheme>(&content) {
                        Ok(serializable_theme) => {
                            let theme = Theme::from(serializable_theme);
                            let theme_name = path
                                .file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or("custom")
                                .to_string();
                            self.available_themes.insert(theme_name, theme);
                        }
                        Err(e) => {
                            eprintln!("Warning: Failed to parse theme file {:?}: {}", path, e);
                        }
                    },
                    Err(e) => {
                        eprintln!("Warning: Failed to read theme file {:?}: {}", path, e);
                    }
                }
            }
        }

        Ok(())
    }

    fn get_config_dir(&self) -> Result<PathBuf, Box<dyn std::error::Error>> {
        if let Some(config_dir) = dirs::config_dir() {
            Ok(config_dir.join("todo"))
        } else {
            // Fallback to home directory
            if let Some(home_dir) = dirs::home_dir() {
                Ok(home_dir.join(".config").join("todo"))
            } else {
                Err("Could not determine config directory".into())
            }
        }
    }
}
