mod texture;

use iced::widget::{button, column, container, text, image as iced_image};
use iced::{Center, Element, Length, Subscription, Task, Font};
use iced::event::{self, Event};
use iced::mouse;
use iced::window;
use texture::TextureLoader;

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
    texture_loader: TextureLoader,
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
    Decrement,
    EventOccurred(Event),
    TextureLoaded(Vec<u8>),
    LoadTexture,
    Ktx2TextureLoaded(Vec<u8>),
    LoadKtx2Texture,
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
            Message::TextureLoaded(data) => {
                if let Err(e) = self.texture_loader.load_from_png_bytes(&data) {
                    log_to_console("Texture load error", &e);
                } else {
                    log_to_console("Texture loaded",
                        &format!("Size: {:?}", self.texture_loader.dimensions()));
                }
            }
            Message::LoadTexture => {
                // 在 WASM 环境中，使用 JavaScript 加载纹理
                #[cfg(target_arch = "wasm32")]
                {
                    return load_texture_from_js();
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    log_to_console("Load texture", "Not supported on native");
                }
            }
            Message::LoadKtx2Texture => {
                // 在 WASM 环境中，使用 JavaScript 加载 KTX2 纹理
                #[cfg(target_arch = "wasm32")]
                {
                    return load_ktx2_from_js();
                }
                #[cfg(not(target_arch = "wasm32"))]
                {
                    // 在原生环境中，从文件系统加载
                    match std::fs::read("public/1.ktx2") {
                        Ok(data) => {
                            return Task::perform(async move { data }, Message::Ktx2TextureLoaded);
                        }
                        Err(e) => {
                            log_to_console("Load KTX2 error", &format!("无法读取文件: {}", e));
                        }
                    }
                }
            }
            Message::Ktx2TextureLoaded(data) => {
                if let Err(e) = self.texture_loader.load_from_ktx2_bytes(&data) {
                    log_to_console("KTX2 load error", &e);
                } else {
                    log_to_console("KTX2 texture loaded",
                        &format!("Size: {:?}", self.texture_loader.dimensions()));
                }
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

        // 创建图像 widget
        let texture_view: Element<'_, Message> = if let Some(handle) = self.texture_loader.as_iced_handle() {
            container(iced_image(handle).width(Length::Fixed(300.0))).into()
        } else {
            container(text("No texture loaded")
                .size(14)
                .font(DEFAULT_FONT))
            .width(Length::Fixed(300.0))
            .height(Length::Fixed(200.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
        };

        container(
            column![
                text("🖱️ Mouse Event & Texture Demo")
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
                button("📷 Load Texture (1.png)")
                    .on_press(Message::LoadTexture),
                button("🎨 Load KTX2 Texture (1.ktx2)")
                    .on_press(Message::LoadKtx2Texture),
                text("Texture Preview:")
                    .size(18)
                    .font(DEFAULT_FONT),
                texture_view,
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

/// 从 JavaScript 加载纹理（仅 WASM）
#[cfg(target_arch = "wasm32")]
fn load_texture_from_js() -> Task<Message> {
    use iced::Task;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    Task::perform(
        async {
            let window = web_sys::window().expect("no global `window` exists");

            // 使用 fetch API
            let promise = window.fetch_with_str("1.png");
            let response_value = match JsFuture::from(promise).await {
                Ok(val) => val,
                Err(e) => {
                    log_to_console("Fetch error", &format!("{:?}", e));
                    return vec![];
                }
            };

            let response: web_sys::Response = response_value.dyn_into().expect("response not valid");

            // 获取 array buffer
            let array_buffer_promise = response.array_buffer().expect("failed to get array buffer");
            let array_buffer = match JsFuture::from(array_buffer_promise).await {
                Ok(buf) => buf,
                Err(e) => {
                    log_to_console("ArrayBuffer error", &format!("{:?}", e));
                    return vec![];
                }
            };

            let u8_array: js_sys::Uint8Array = js_sys::Uint8Array::new(&array_buffer);
            let mut vec = vec![0; u8_array.length() as usize];
            u8_array.copy_to(&mut vec);

            log_to_console("Texture fetch", &format!("Loaded {} bytes", vec.len()));
            vec
        },
        Message::TextureLoaded,
    )
}

/// 占位函数（非 WASM 平台）
#[cfg(not(target_arch = "wasm32"))]
fn load_texture_from_js() -> Task<Message> {
    Task::none()
}

/// 从 JavaScript 加载 KTX2 纹理（仅 WASM）
#[cfg(target_arch = "wasm32")]
fn load_ktx2_from_js() -> Task<Message> {
    use iced::Task;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    Task::perform(
        async {
            let window = web_sys::window().expect("no global `window` exists");

            // 使用 fetch API 加载 KTX2 文件
            let promise = window.fetch_with_str("1.ktx2");
            let response_value = match JsFuture::from(promise).await {
                Ok(val) => val,
                Err(e) => {
                    log_to_console("KTX2 Fetch error", &format!("{:?}", e));
                    return vec![];
                }
            };

            let response: web_sys::Response = response_value.dyn_into().expect("response not valid");

            // 获取 array buffer
            let array_buffer_promise = response.array_buffer().expect("failed to get array buffer");
            let array_buffer = match JsFuture::from(array_buffer_promise).await {
                Ok(buf) => buf,
                Err(e) => {
                    log_to_console("KTX2 ArrayBuffer error", &format!("{:?}", e));
                    return vec![];
                }
            };

            let u8_array: js_sys::Uint8Array = js_sys::Uint8Array::new(&array_buffer);
            let mut vec = vec![0; u8_array.length() as usize];
            u8_array.copy_to(&mut vec);

            log_to_console("KTX2 fetch", &format!("Loaded {} bytes", vec.len()));
            vec
        },
        Message::Ktx2TextureLoaded,
    )
}

/// 占位函数（非 WASM 平台）
#[cfg(not(target_arch = "wasm32"))]
fn load_ktx2_from_js() -> Task<Message> {
    Task::none()
}
