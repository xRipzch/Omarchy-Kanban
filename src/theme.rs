use ratatui::style::Color;

#[derive(Debug, Clone)]
pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub success: Color,
    pub danger: Color,
    pub border_normal: Color,
    pub border_focused: Color,
    pub background_selected: Color,
    // Tag colors
    pub tag_urgent: Color,
    pub tag_security: Color,
    pub tag_bug: Color,
    pub tag_feature: Color,
    pub tag_performance: Color,
    pub tag_enhancement: Color,
    pub tag_user: Color,
    pub tag_dev: Color,
    pub tag_documentation: Color,
    pub tag_design: Color,
    pub tag_refactor: Color,
    pub tag_default: Color,
}

impl Theme {
    pub fn get_tag_color(&self, tag: &str) -> Color {
        match tag {
            "urgent" => self.tag_urgent,
            "security" => self.tag_security,
            "bug" => self.tag_bug,
            "feature" => self.tag_feature,
            "performance" => self.tag_performance,
            "enhancement" => self.tag_enhancement,
            "User" => self.tag_user,
            "Dev" => self.tag_dev,
            "documentation" => self.tag_documentation,
            "design" => self.tag_design,
            "refactor" => self.tag_refactor,
            _ => self.tag_default,
        }
    }

    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "high-contrast" => Some(Self::high_contrast()),
            "classic" => Some(Self::classic()),
            "solarized-dark" => Some(Self::solarized_dark()),
            "gruvbox" => Some(Self::gruvbox()),
            "nord" => Some(Self::nord()),
            _ => None,
        }
    }

    pub fn all_theme_names() -> Vec<&'static str> {
        vec![
            "high-contrast",
            "classic",
            "solarized-dark",
            "gruvbox",
            "nord",
        ]
    }

    fn high_contrast() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Cyan,
            accent: Color::LightYellow,
            text_primary: Color::White,
            text_secondary: Color::White,
            success: Color::LightGreen,
            danger: Color::LightRed,
            border_normal: Color::White,
            border_focused: Color::Cyan,
            background_selected: Color::Blue,
            tag_urgent: Color::LightRed,
            tag_security: Color::Red,
            tag_bug: Color::LightYellow,
            tag_feature: Color::LightGreen,
            tag_performance: Color::Green,
            tag_enhancement: Color::LightBlue,
            tag_user: Color::Cyan,
            tag_dev: Color::LightMagenta,
            tag_documentation: Color::LightCyan,
            tag_design: Color::Cyan,
            tag_refactor: Color::Yellow,
            tag_default: Color::White,
        }
    }

    fn classic() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Cyan,
            accent: Color::Yellow,
            text_primary: Color::White,
            text_secondary: Color::White,
            success: Color::Green,
            danger: Color::Red,
            border_normal: Color::White,
            border_focused: Color::Cyan,
            background_selected: Color::Blue,
            tag_urgent: Color::Red,
            tag_security: Color::LightRed,
            tag_bug: Color::Yellow,
            tag_feature: Color::Green,
            tag_performance: Color::LightGreen,
            tag_enhancement: Color::Blue,
            tag_user: Color::LightBlue,
            tag_dev: Color::Magenta,
            tag_documentation: Color::Cyan,
            tag_design: Color::LightCyan,
            tag_refactor: Color::LightYellow,
            tag_default: Color::White,
        }
    }

    fn solarized_dark() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Blue,
            accent: Color::Yellow,
            text_primary: Color::LightCyan,
            text_secondary: Color::Cyan,
            success: Color::Green,
            danger: Color::Red,
            border_normal: Color::Cyan,
            border_focused: Color::Cyan,
            background_selected: Color::DarkGray,
            tag_urgent: Color::Red,
            tag_security: Color::LightRed,
            tag_bug: Color::Yellow,
            tag_feature: Color::Green,
            tag_performance: Color::LightGreen,
            tag_enhancement: Color::Blue,
            tag_user: Color::LightBlue,
            tag_dev: Color::Magenta,
            tag_documentation: Color::Cyan,
            tag_design: Color::LightCyan,
            tag_refactor: Color::LightYellow,
            tag_default: Color::White,
        }
    }

    fn gruvbox() -> Self {
        Self {
            primary: Color::LightYellow,
            secondary: Color::LightGreen,
            accent: Color::Yellow,
            text_primary: Color::LightYellow,
            text_secondary: Color::Yellow,
            success: Color::Green,
            danger: Color::Red,
            border_normal: Color::Yellow,
            border_focused: Color::LightYellow,
            background_selected: Color::DarkGray,
            tag_urgent: Color::Red,
            tag_security: Color::LightRed,
            tag_bug: Color::Yellow,
            tag_feature: Color::Green,
            tag_performance: Color::LightGreen,
            tag_enhancement: Color::Blue,
            tag_user: Color::LightBlue,
            tag_dev: Color::Magenta,
            tag_documentation: Color::Cyan,
            tag_design: Color::LightCyan,
            tag_refactor: Color::LightYellow,
            tag_default: Color::White,
        }
    }

    fn nord() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::LightBlue,
            accent: Color::White,
            text_primary: Color::White,
            text_secondary: Color::LightCyan,
            success: Color::Green,
            danger: Color::Red,
            border_normal: Color::Blue,
            border_focused: Color::Cyan,
            background_selected: Color::DarkGray,
            tag_urgent: Color::Red,
            tag_security: Color::LightRed,
            tag_bug: Color::Yellow,
            tag_feature: Color::Green,
            tag_performance: Color::LightGreen,
            tag_enhancement: Color::Blue,
            tag_user: Color::LightBlue,
            tag_dev: Color::Magenta,
            tag_documentation: Color::Cyan,
            tag_design: Color::LightCyan,
            tag_refactor: Color::LightYellow,
            tag_default: Color::White,
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::high_contrast()
    }
}
