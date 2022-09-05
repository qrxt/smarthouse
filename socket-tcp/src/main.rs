use iced::text_input::{self};
use iced::toggler::Toggler;
use iced::{
    alignment, button, window, Alignment, Application, Button, Color, Column, Container, Element,
    Length, Row, Settings, Text, TextInput,
};
use iced::{executor, Command};
use socket_tcp::client::{Client, ConnectionError};

async fn get_status(address: String) -> Result<String, ConnectionError> {
    let client = Client::new(&address);

    client.get_status().await
}

async fn get_name(address: String) -> Result<String, ConnectionError> {
    let client = Client::new(&address);

    client.get_name().await
}

async fn change_socket_state(current: bool, address: String) -> Result<String, ConnectionError> {
    let client = Client::new(&address);

    match current {
        true => client.turn_on().await,
        false => client.turn_off().await,
    }
}

async fn get_power_consumption(address: String) -> Result<String, ConnectionError> {
    let client = Client::new(&address);

    client.get_power_consumption().await
}

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            size: (300, 225),
            resizable: false,
            decorations: true,
            ..Default::default()
        },
        ..Default::default()
    };

    Model::run(settings)
}

#[derive(Debug, PartialEq, Eq)]
enum ApplicationState {
    Connected,
    NotConnected,
    FailedToConnect,
}

struct Model {
    state: ApplicationState,

    name: String,
    status: bool,
    power_consumption: String,

    ip_address_button_state: button::State,
    ip_address_input_state: text_input::State,
    ip_address_input: String,
}

#[derive(Debug, Clone)]
enum Message {
    StatusReceived(Result<String, ConnectionError>),
    NameReceived(Result<String, ConnectionError>),
    PowerConsumptionReceived(Result<String, ConnectionError>),
    ChangeSocketState(bool),

    IpAddressChanged(String),
    IpAddressSubmit(String),
}

impl Application for Model {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_: ()) -> (Self, Command<Self::Message>) {
        (
            Self {
                state: ApplicationState::NotConnected,

                status: false,
                name: "Smart socket".to_string(),
                power_consumption: "0.0".to_string(),

                ip_address_button_state: button::State::default(),
                ip_address_input: "".to_string(),
                ip_address_input_state: text_input::State::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Smart Socket client".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::StatusReceived(result) => {
                println!("status received: {:?}", result);

                match result {
                    Ok(status) => {
                        self.status = matches!(status.as_str(), "on");
                        self.state = ApplicationState::Connected;

                        println!("$$$OK");

                        Command::none()
                    }
                    Err(e) => {
                        eprintln!("Failed to get socket status: {}", e);
                        self.status = false;
                        self.state = ApplicationState::FailedToConnect;

                        Command::none()
                    }
                }
            }

            Message::ChangeSocketState(new_state) => Command::batch(vec![
                Command::perform(
                    change_socket_state(new_state, self.ip_address_input.to_string()),
                    Message::StatusReceived,
                ),
                Command::perform(
                    get_power_consumption(self.ip_address_input.to_string()),
                    Message::PowerConsumptionReceived,
                ),
            ]),
            Message::NameReceived(result) => match result {
                Ok(name) => {
                    self.name = name;

                    Command::none()
                }
                Err(e) => {
                    eprintln!("Failed to get socket name: {}", e);
                    self.name = "Unknown".to_string();

                    Command::none()
                }
            },
            Message::PowerConsumptionReceived(result) => match result {
                Ok(power_consumption) => {
                    self.power_consumption = power_consumption;

                    Command::none()
                }
                Err(e) => {
                    eprintln!("Failed to get socket power consumption: {}", e);
                    self.power_consumption = "0.0".to_string();

                    Command::none()
                }
            },
            Message::IpAddressChanged(new_input_state) => {
                println!("Changing text input state: {}", new_input_state);

                self.ip_address_input = new_input_state;

                Command::none()
            }
            Message::IpAddressSubmit(ip_address) => Command::batch(vec![
                Command::perform(get_status(ip_address.to_string()), Message::StatusReceived),
                Command::perform(get_name(ip_address.to_string()), Message::NameReceived),
                Command::perform(
                    get_power_consumption(ip_address),
                    Message::PowerConsumptionReceived,
                ),
            ]),
        }
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        println!("app state: {:?}", &self.state);

        match self.state {
            ApplicationState::NotConnected | ApplicationState::FailedToConnect => {
                let connection_error_text = match self.state {
                    ApplicationState::FailedToConnect => {
                        "Failed to connect: address is not available".to_string()
                    }
                    _ => "".to_string(),
                };
                let connection_error_text_component =
                    Text::new(connection_error_text).color(Color::from_rgb(1., 0.2, 0.2));

                let connect_button: Button<Message> = Button::new(
                    &mut self.ip_address_button_state,
                    Row::new()
                        .spacing(10)
                        .align_items(Alignment::Center)
                        .push(Text::new("Connect")),
                )
                .on_press(Message::IpAddressSubmit(self.ip_address_input.to_string()));

                let input_ip_address = TextInput::new(
                    &mut self.ip_address_input_state,
                    "IP address",
                    &self.ip_address_input,
                    Message::IpAddressChanged,
                )
                .on_submit(Message::IpAddressSubmit(self.ip_address_input.to_string()));

                let inputs_row = Row::new()
                    .spacing(10)
                    .align_items(Alignment::Center)
                    .push(input_ip_address)
                    .push(connect_button);

                Container::new(
                    Column::new()
                        .padding(10)
                        .push(connection_error_text_component)
                        .push(inputs_row)
                        .align_items(Alignment::Center),
                )
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(alignment::Horizontal::Center)
                .align_y(alignment::Vertical::Center)
                .into()
            }

            ApplicationState::Connected => {
                let name = Text::new(self.name.to_string())
                    .size(24)
                    .horizontal_alignment(alignment::Horizontal::Left);
                let power_consumption =
                    Text::new(format!("Power consumption: {}W", self.power_consumption)).size(16);

                let toggle_label = match &self.status {
                    true => "Turn off",
                    false => "Turn on",
                };
                let toggle = Toggler::new(
                    self.status,
                    toggle_label.to_string(),
                    Message::ChangeSocketState,
                );

                let column = Column::new()
                    .padding(10)
                    .push(name)
                    .push(toggle)
                    .push(power_consumption)
                    .width(Length::Units(200))
                    .height(Length::Fill)
                    .align_items(Alignment::Center);

                Column::new()
                    .push(
                        Container::new(column)
                            .width(Length::Units(400))
                            .height(Length::Units(400))
                            .align_x(alignment::Horizontal::Center)
                            .align_y(alignment::Vertical::Center)
                            .padding(20)
                            .style(style::Container),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_items(Alignment::Center)
                    .spacing(50)
                    .padding(30)
                    .into()
            }
        }
    }

    fn background_color(&self) -> Color {
        Color::from_rgb(0.78, 0.84, 0.92)
    }
}

mod style {
    use iced::container;
    use iced::{Background, Color};

    pub struct Container;
    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                text_color: Some(Color::from_rgb(0., 0., 0.)),
                background: Some(Background::Color(Color::from_rgb(1., 1., 1.))),
                border_radius: 20.,
                border_width: 1.,
                border_color: Color::from_rgb(1., 1., 1.),
            }
        }
    }
}
