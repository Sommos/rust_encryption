use iced::widget;

struct CryptographyType {
    encryption_type: String,
    encryption_algorithm: String,
}

#[derive(Debug, Clone, Copy)]
enum Cryptography {
    Encrypt,
    Decrypt,
    Vigenere,
    Caesar,
}

impl CryptographyType {
    fn new() -> Self {
        Self { 
            encryption_type: String::from(""),
            encryption_algorithm: String::from(""),
        }
    }

    fn update(&mut self, message: Cryptography) -> iced::Task<Cryptography> {
        match message {
            Cryptography::Encrypt => self.encryption_type = String::from("Encrypt"),
            Cryptography::Decrypt => self.encryption_type = String::from("Decrypt"),
            Cryptography::Caesar => self.encryption_algorithm = String::from("Caesar"),
            Cryptography::Vigenere => self.encryption_algorithm = String::from("Vigenere"),
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Cryptography> {
        let buttons_row = widget::row![
            widget::button("Encrypt").on_press(Cryptography::Encrypt),
            widget::button("Decrypt").on_press(Cryptography::Decrypt),
        ]
        .spacing(10);
    
        let top_row = widget::container(buttons_row)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .align_x(iced::Alignment::Center);

        let buttons_row_2 = widget::row![
            widget::button("Caesar Cipher").on_press(Cryptography::Caesar),
            widget::button("Vigenere Cipher").on_press(Cryptography::Vigenere),
        ]
        .spacing(10);

        let second_row = widget::container(buttons_row_2)
            .width(iced::Length::Fill)
            .height(iced::Length::Shrink)
            .align_x(iced::Alignment::Center);

        let display_text = format!("{} // {}", self.encryption_type, self.encryption_algorithm);

        let centered_text = widget::text(display_text).size(50);
    
        let center_content = widget::container(centered_text)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center);

        let layout = widget::column![
            widget::container(widget::text(""))
            .height(iced::Length::Fixed(20.0)),
            top_row,
            widget::container(widget::text(""))
            .height(iced::Length::Fixed(20.0)),
            second_row,
            widget::container(widget::text(""))
            .height(iced::Length::Fixed(20.0)),
            center_content,
        ]
        .width(iced::Length::Fill)
        .height(iced::Length::Fill);
    
        widget::container(layout)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }
}

pub fn gui_init() -> Result<(), iced::Error> {
    iced::application(
        "Rust Encryption", 
        CryptographyType::update,
        CryptographyType::view,
    )
    .run_with(|| (CryptographyType::new(), iced::Task::none()))
}