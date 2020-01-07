use iced::{button, Align, Application, Checkbox, Column, Command, Container, Element, Length, Row, Sandbox, Settings, Subscription, Text, Image, HorizontalAlignment, Background, Color, Button};
use crate::message::Message;
use crate::my_button;

#[derive(Clone, Debug, Default)]
pub struct Device {
    ip: String,
    name: String,
    status: bool,
}

impl Device {
    pub fn view(&self) -> Element<Message> {
        let status =
            if self.status {
                "connect".to_string()
            }
            else {
                "disconnect".to_string()
            };
        Row::new()
            .spacing(20)
            .align_items(Align::Center)
            .push(Text::new(self.ip.clone()))
            .push(Text::new(self.name.clone()))
            .push(Text::new(status))
            .into()
    }
}

#[derive(Clone, Debug, Default)]
pub struct DeviceList {
    pub group_name: String,
    devices: Vec<Device>,
    button_refresh: button::State,
    button_back: button::State,
    button_disconnect: button::State,
}

impl Sandbox for DeviceList {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Tinc-gui")
    }

    fn update(&mut self, message: Message) {
//        match message {
//        }
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("Device List")
            .size(50)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Left);

        let top = Row::new()
            .align_items(Align::End)
            .spacing(10)
            .push(title)
            .push(
                my_button(&mut self.button_back, "back")
                    .on_press(Message::BackGroupList)
            )
            .push(
                my_button(&mut self.button_refresh, "Refresh")
                    .on_press(Message::Refresh)
            )
            .push(
                my_button(&mut self.button_disconnect, "Disconnect")
                    .on_press(Message::Disconnect(self.group_name.clone()))
            );

        let device_list: Element<_> = self.devices
            .iter_mut()
            .enumerate()
            .fold(Column::new().spacing(30),
                  |column, (i, devices)| {
                      column.push(devices.view())
                  }
            )
            .into();

        let under = Column::new()
            .spacing(10)
            .push(
                Row::new()
                    .spacing(20)
                    .align_items(Align::Center)
                    .push(Text::new("ip"))
                    .push(Text::new("name"))
                    .push(Text::new("status"))
            )
            .push(
                device_list
            );

        let window = Column::new()
            .spacing(40)
            .padding(20)
            .push(top)
            .push(under);

        window.into()
    }
}

#[derive(Debug, Clone)]
pub enum Error {
    Unknown,
}

pub async fn find_device(group_name: String) -> Result<DeviceList, Error> {
    let device = Device {
        ip: "10.1.1.1".to_string(),
        name: "no.1".to_string(),
        status: true,
    };

    let device_list = DeviceList {
        devices: vec![device],
        group_name,
        button_refresh: button::State::new(),
        button_back: button::State::new(),
        button_disconnect: button::State::new(),
    };
    Ok(device_list)
}