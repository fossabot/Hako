use iced::widget::{button as iced_button, row, text as iced_text};
use iced::{Background, Border, Element, Length, Padding, Shadow, Theme};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Primary,
    Secondary,
    Danger,
    Subtle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Default for Variant {
    fn default() -> Self {
        Self::Primary
    }
}

impl Default for ButtonSize {
    fn default() -> Self {
        Self::Medium
    }
}

pub struct Button<'a, Message> {
    text: String,
    variant: Variant,
    size: ButtonSize,
    disabled: bool,
    on_press: Option<Message>,
    width: Length,
    height: Length,
    icon: Option<Element<'a, Message>>, // TODO: 引入icon :D
}

impl<'a, Message> Button<'a, Message> {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            variant: Variant::default(),
            size: ButtonSize::default(),
            disabled: false,
            on_press: None,
            width: Length::Shrink,
            height: Length::Shrink,
            icon: None,
        }
    }

    pub fn variant(mut self, variant: Variant) -> Self {
        self.variant = variant;
        self
    }

    pub fn size(mut self, size: ButtonSize) -> Self {
        self.size = size;
        self
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn icon(mut self, icon: impl Into<Element<'a, Message>>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

impl<'a, Message> From<Button<'a, Message>> for Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(mut button: Button<'a, Message>) -> Self {
        let padding = button.get_padding();
        let text_size = button.get_text_size();
        let text = button.text;
        let variant = button.variant;
        let size = button.size;

        let mut content = row![];
        if let Some(icon) = button.icon.take() {
            content = content.push(icon);
        }
        content = content.push(iced_text(text).size(text_size));

        let mut btn = iced_button(content)
            .padding(padding)
            .width(button.width)
            .height(button.height)
            .style(move |theme, status| Button::<Message>::style(variant, size, theme, status));

        if let Some(message) = button.on_press
            && !button.disabled
        {
            btn = btn.on_press(message);
        }

        btn.into()
    }
}

impl<'a, Message> Button<'a, Message> {
    fn get_padding(&self) -> Padding {
        match self.size {
            ButtonSize::Small => Padding::from([4.0, 8.0]),
            ButtonSize::Medium => Padding::from([8.0, 16.0]),
            ButtonSize::Large => Padding::from([12.0, 24.0]),
        }
    }

    fn get_text_size(&self) -> f32 {
        match self.size {
            ButtonSize::Small => 12.0,
            ButtonSize::Medium => 14.0,
            ButtonSize::Large => 16.0,
        }
    }

    fn style(
        variant: Variant,
        _size: ButtonSize,
        theme: &Theme,
        status: iced::widget::button::Status,
    ) -> iced::widget::button::Style {
        let palette = theme.extended_palette();
        let (bg_color, text_color, border_color) = match variant {
            Variant::Primary => (palette.primary.base.color, palette.primary.base.text, None),
            Variant::Secondary => (
                iced::Color::TRANSPARENT,
                palette.background.base.text,
                Some(palette.background.strong.color),
            ),
            Variant::Danger => (palette.danger.base.color, palette.danger.base.text, None),
            Variant::Subtle => (
                palette.background.weakest.color,
                palette.background.base.text,
                None,
            ),
        };

        let (final_bg, final_text, final_border) = match status {
            iced::widget::button::Status::Active | iced::widget::button::Status::Pressed => {
                (bg_color, text_color, border_color)
            }
            iced::widget::button::Status::Hovered => match variant {
                Variant::Primary => (
                    palette.primary.strong.color,
                    palette.primary.strong.text,
                    None,
                ),
                Variant::Secondary => (
                    palette.background.strong.color.scale_alpha(0.1),
                    text_color,
                    border_color,
                ),
                Variant::Danger => (
                    palette.danger.strong.color,
                    palette.danger.strong.text,
                    None,
                ),
                Variant::Subtle => (palette.background.weaker.color, text_color, None),
            },
            iced::widget::button::Status::Disabled => (
                bg_color.scale_alpha(0.5),
                text_color.scale_alpha(0.5),
                border_color.map(|c| c.scale_alpha(0.5)),
            ),
        };

        iced::widget::button::Style {
            background: if final_bg == iced::Color::TRANSPARENT {
                None
            } else {
                Some(Background::Color(final_bg))
            },
            text_color: final_text,
            border: Border {
                width: if final_border.is_some() { 1.0 } else { 0.0 },
                radius: 8.0.into(),
                color: final_border.unwrap_or(iced::Color::TRANSPARENT),
            },
            shadow: Shadow::default(),
            snap: true,
        }
    }
}
