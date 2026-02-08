use iced::widget::{button, column, container, text};
use iced::{Center, Element, Length, Subscription, Task, Font};
use iced::event::{self, Event};
use iced::mouse;
use iced::window;

// Default font
const DEFAULT_FONT: Font = Font::DEFAULT;

pub fn main() -> iced::Result {
    iced::application(Counter::new, Counter::update, Counter::view)
        .subscription(Counter::subscription)
        .run()
}

#[derive(Default)]
struct Counter {
    value: i64,
    mouse_position: Option<mouse::Cursor>,
    mouse_buttons: String,
    last_event: String,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    EventOccurred(Event),
}

impl Counter {
    fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Increment => {
                self.value += 1;
                log_to_console("Increment clicked", &format!("New value: {}", self.value));
            }
            Message::Decrement => {
                self.value -= 1;
                log_to_console("Decrement clicked", &format!("New value: {}", self.value));
            }
            Message::EventOccurred(event) => {
                self.handle_event(event);
            }
        }
        Task::none()
    }

    fn view(&self) -> Element<'_, Message> {
        let mouse_info = if let Some(cursor) = self.mouse_position {
            match cursor.position() {
                Some(pos) => format!("Mouse: ({:.1}, {:.1})", pos.x, pos.y),
                None => "Mouse: Unknown".to_string(),
            }
        } else {
            "Mouse: Not detected".to_string()
        };

        container(
            column![
                text("🖱️ Mouse Event Demo")
                    .size(30)
                    .font(DEFAULT_FONT),
                text(mouse_info)
                    .size(16)
                    .font(DEFAULT_FONT),
                text(self.mouse_buttons.clone())
                    .size(16)
                    .font(DEFAULT_FONT),
                text(format!("Counter: {}", self.value))
                    .size(50)
                    .font(DEFAULT_FONT),
                button("Increment ➕")
                    .on_press(Message::Increment),
                button("Decrement ➖")
                    .on_press(Message::Decrement),
                text(format!("Last event: {}", self.last_event))
                    .size(14)
                    .font(DEFAULT_FONT)
            ]
            .spacing(20)
            .padding(20)
            .align_x(Center)
        )
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::EventOccurred)
    }

    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Mouse(mouse_event) => {
                self.handle_mouse_event(mouse_event);
            }
            Event::Window(window_event) => {
                self.handle_window_event(window_event);
            }
            _ => {}
        }
    }

    fn handle_mouse_event(&mut self, mouse_event: mouse::Event) {
        match mouse_event {
            mouse::Event::CursorMoved { position } => {
                self.mouse_position = Some(mouse::Cursor::Available(position));
                let pos_str = format!("({}, {})", position.x, position.y);
                self.last_event = format!("Mouse moved: {}", pos_str);
                log_to_console("🖱️ Mouse move", &pos_str);
            }
            mouse::Event::ButtonPressed(button) => {
                let button_name = match button {
                    mouse::Button::Left => "Left",
                    mouse::Button::Right => "Right",
                    mouse::Button::Middle => "Middle",
                    mouse::Button::Back => "Back",
                    mouse::Button::Forward => "Forward",
                    mouse::Button::Other(_) => "Other",
                };
                self.mouse_buttons = format!("🖱️ Pressed: {}", button_name);
                self.last_event = format!("Mouse pressed: {}", button_name);
                log_to_console("🖱️ Mouse press", button_name);
            }
            mouse::Event::ButtonReleased(button) => {
                let button_name = match button {
                    mouse::Button::Left => "Left",
                    mouse::Button::Right => "Right",
                    mouse::Button::Middle => "Middle",
                    mouse::Button::Back => "Back",
                    mouse::Button::Forward => "Forward",
                    mouse::Button::Other(_) => "Other",
                };
                self.mouse_buttons = format!("🖱️ Released: {}", button_name);
                self.last_event = format!("Mouse released: {}", button_name);
                log_to_console("🖱️ Mouse release", button_name);
            }
            mouse::Event::CursorEntered => {
                self.last_event = "Mouse entered window".to_string();
                log_to_console("🖱️ Mouse enter", "Cursor entered window");
            }
            mouse::Event::CursorLeft => {
                self.mouse_position = None;
                self.last_event = "Mouse left window".to_string();
                log_to_console("🖱️ Mouse leave", "Cursor left window");
            }
            mouse::Event::WheelScrolled { delta } => {
                let delta_str = match delta {
                    mouse::ScrollDelta::Lines { x, y } => {
                        format!("({:.1}, {:.1}) lines", x, y)
                    }
                    mouse::ScrollDelta::Pixels { x, y } => {
                        format!("({:.1}, {:.1}) pixels", x, y)
                    }
                };
                self.last_event = format!("Wheel scrolled: {}", delta_str);
                log_to_console("🖱️ Wheel scroll", &delta_str);
            }
        }
    }

    fn handle_window_event(&mut self, window_event: window::Event) {
        match window_event {
            window::Event::Resized(size) => {
                self.last_event = format!("Window resized: {}x{}", size.width, size.height);
                log_to_console("🪟 Window resize", &self.last_event);
            }
            window::Event::CloseRequested => {
                self.last_event = "Window close requested".to_string();
                log_to_console("🪟 Window close", "Close requested");
            }
            window::Event::Focused => {
                self.last_event = "Window focused".to_string();
                log_to_console("🪟 Window focus", "Gained focus");
            }
            _ => {}
        }
    }
}

fn log_to_console(event: &str, details: &str) {
    #[cfg(target_arch = "wasm32")]
    {
        web_sys::console::log_1(&format!("{}: {}", event, details).into());
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        println!("{}: {}", event, details);
    }
}
