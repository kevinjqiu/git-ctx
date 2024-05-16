use ratatui::layout::Alignment;
use ratatui::prelude::{Color, Rect, Style};
use ratatui::widgets::Paragraph;
use tuirealm::command::{Cmd, CmdResult};
use tuirealm::props::{Borders, TextModifiers};
use tuirealm::{
    Application, AttrValue, Attribute, Frame, MockComponent, NoUserEvent, Props, State, StateValue,
};

struct Counter {
    props: Props,
    states: OwnStates,
}

impl Default for Counter {
    fn default() -> Self {
        return Self {
            props: Props::default(),
            states: OwnStates::default(),
        };
    }
}

impl Counter {
    pub fn label<S>(mut self, label: S) -> Self
    where
        S: AsRef<str>,
    {
        self.attr(
            Attribute::Title,
            AttrValue::Title((label.as_ref().to_string(), Alignment::Center)),
        );
        self
    }

    pub fn value(mut self, n: isize) -> Self {
        self.attr(Attribute::Value, AttrValue::Number(n));
        self
    }

    pub fn alignment(mut self, a: Alignment) -> Self {
        self.attr(Attribute::TextAlign, AttrValue::Alignment(a));
        self
    }

    pub fn foreground(mut self, c: Color) -> Self {
        self.attr(Attribute::Foreground, AttrValue::Color(c));
        self
    }

    pub fn background(mut self, c: Color) -> Self {
        self.attr(Attribute::Background, AttrValue::Color(c));
        self
    }

    pub fn modifiers(mut self, m: TextModifiers) -> Self {
        self.attr(Attribute::TextProps, AttrValue::TextModifiers(m));
        self
    }

    pub fn borders(mut self, b: Borders) -> Self {
        self.attr(Attribute::Borders, AttrValue::Borders(b));
        self
    }
}

impl MockComponent for Counter {
    fn view(&mut self, frame: &mut Frame, area: Rect) {
        // Check if visible
        if self.props.get_or(Attribute::Display, AttrValue::Flag(true)) == AttrValue::Flag(true) {
            // Get properties
            let text = self.states.counter.to_string();
            let alignment = self
                .props
                .get_or(Attribute::TextAlign, AttrValue::Alignment(Alignment::Left))
                .unwrap_alignment();
            let foreground = self
                .props
                .get_or(Attribute::Foreground, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let background = self
                .props
                .get_or(Attribute::Background, AttrValue::Color(Color::Reset))
                .unwrap_color();
            let modifiers = self
                .props
                .get_or(
                    Attribute::TextProps,
                    AttrValue::TextModifiers(TextModifiers::empty()),
                )
                .unwrap_text_modifiers();
            let title = self
                .props
                .get_or(
                    Attribute::Title,
                    AttrValue::Title((String::default(), Alignment::Center)),
                )
                .unwrap_title();
            let borders = self
                .props
                .get_or(Attribute::Borders, AttrValue::Borders(Borders::default()))
                .unwrap_borders();
            let focus = self
                .props
                .get_or(Attribute::Focus, AttrValue::Flag(false))
                .unwrap_flag();
            frame.render_widget(
                Paragraph::new(text)
                    .block(get_block(borders, title, focus))
                    .style(
                        Style::default()
                            .fg(foreground)
                            .bg(background)
                            .add_modifier(modifiers),
                    )
                    .alignment(alignment),
                area,
            );
        }
    }

    fn query(&self, attr: Attribute) -> Option<AttrValue> {
        self.props.get(attr)
    }

    fn attr(&mut self, attr: Attribute, value: AttrValue) {
        self.props.set(attr, value);
    }

    fn state(&self) -> State {
        State::One(StateValue::Isize(self.states.counter))
    }

    fn perform(&mut self, cmd: Cmd) -> CmdResult {
        match cmd {
            Cmd::Submit => {
                self.states.incr();
                CmdResult::Changed(self.state())
            }
            _ => CmdResult::None,
        }
    }
}

struct OwnStates {
    counter: isize,
}

impl Default for OwnStates {
    fn default() -> Self {
        return Self { counter: 0 };
    }
}

impl OwnStates {
    fn incr(&mut self) {
        self.counter += 1
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Id {
    DigitCounter,
    LetterCounter,
}

fn init_app() -> Application<Id, Msg, NoUserEvent> {}
