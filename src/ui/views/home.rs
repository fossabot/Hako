use crate::ui::widgets::button::{Button, ButtonSize, Variant};
use iced::widget::{column, row, text, text_input};
use iced::{Element, Length};

#[derive(Default, Debug, Clone)]
pub struct State {
    pub sub: Sub,
    pub stack: Vec<Page>,
    pub content: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Sub {
    #[default]
    List,
    Stats,
}

#[derive(Debug, Clone)]
pub enum Page {
    Detail(String),
}

#[derive(Clone, Debug)]
pub enum Message {
    SwitchSub(Sub),
    PushDetail(String),
    Pop,
    ContentChanged(String),
}

pub fn update(state: &mut State, message: Message) {
    match message {
        Message::SwitchSub(s) => state.sub = s,
        Message::PushDetail(t) => state.stack.push(Page::Detail(t)),
        Message::Pop => {
            state.stack.pop();
        }
        Message::ContentChanged(c) => state.content = c,
    }
}

pub fn view(state: &State) -> Element<'_, Message> {
    let side = column![
        Button::new("List")
            .variant(Variant::Primary)
            .size(ButtonSize::Medium)
            .on_press(Message::SwitchSub(Sub::List)),
        Button::new("Stats")
            .variant(Variant::Secondary)
            .size(ButtonSize::Medium)
            .on_press(Message::SwitchSub(Sub::Stats)),
    ];

    let main: iced::Element<'_, Message> = if let Some(top) = state.stack.last() {
        match top {
            Page::Detail(t) => iced::widget::Column::new()
                .push(text(format!("Detail: {} (depth {})", t, state.stack.len())))
                .push(
                    Button::new("Open Next")
                        .variant(Variant::Primary)
                        .size(ButtonSize::Medium)
                        .on_press(Message::PushDetail(format!("{}-next", t))),
                )
                .push(
                    Button::new("Back")
                        .variant(Variant::Danger)
                        .size(ButtonSize::Medium)
                        .on_press(Message::Pop),
                )
                .into(),
        }
    } else {
        match state.sub {
            Sub::List => column![
                text("Button Demo"),
                row![
                    Button::new("Primary")
                        .variant(Variant::Primary)
                        .size(ButtonSize::Small)
                        .on_press(Message::PushDetail("A".into())),
                    Button::new("Secondary")
                        .variant(Variant::Secondary)
                        .size(ButtonSize::Medium)
                        .on_press(Message::PushDetail("B".into())),
                    Button::new("Danger")
                        .variant(Variant::Danger)
                        .size(ButtonSize::Large)
                        .on_press(Message::PushDetail("C".into())),
                ]
                .spacing(8),
                row![
                    Button::new("Subtle")
                        .variant(Variant::Subtle)
                        .size(ButtonSize::Medium)
                        .on_press(Message::PushDetail("D".into())),
                    Button::new("Disabled")
                        .variant(Variant::Primary)
                        .size(ButtonSize::Medium)
                        .disabled(true)
                        .on_press(Message::PushDetail("E".into())),
                    Button::new("No Action")
                        .variant(Variant::Secondary)
                        .size(ButtonSize::Medium),
                ]
                .spacing(8),
                text_input("Type...", &state.content)
                    .on_input(Message::ContentChanged)
                    .size(16),
            ]
            .spacing(12)
            .into(),
            Sub::Stats => column![text("Stats View")].into(),
        }
    };

    if state.stack.is_empty() {
        row![side, main].spacing(16).height(Length::Fill).into()
    } else {
        row![main].height(Length::Fill).into()
    }
}
