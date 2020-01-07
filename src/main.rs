use std::rc::Rc;
use iced::{button, Align, Application, Color, Checkbox, Column, Command, Container, Element, Length, Settings, Subscription, Text, Button};

mod group_list;
use crate::group_list::GroupList;
mod device_list;
use crate::device_list::{DeviceList, find_device};
use crate::message::Message;

mod message;

pub fn main() {
    Windows::run(Settings::default())
}

#[derive(Debug)]
enum Windows {
    GroupList {
        group_list: GroupList,
        refresh: button::State,
    },
    DeviceList {
        device_list: DeviceList,
    },
}

impl Application for Windows {
    type Message = Message;

    fn new() -> (Windows, Command<Message>) {
        (
            Windows::GroupList {
                 group_list: GroupList::new(),
                 refresh: button::State::new()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        let subtitle = match self {
            Windows::GroupList {group_list, ..} => "group_list",
            Windows::DeviceList {device_list, ..} => &device_list.group_name,
        };

        format!("Tinc - {}", subtitle)
    }

    fn update(&mut self, message: Message) -> Command<Message> {

        match message {
            Message::BackGroupList => {
                *self = Windows::GroupList {
                    group_list: GroupList::new(),
                    refresh: button::State::new(),
                };
                Command::none()
            }

            Message::SelectGroup(group_name) => {
                Command::perform(find_device(group_name), Message::DeviceFound)
            }

            Message::DeviceFound(Ok(device_list)) => {
                *self = Windows::DeviceList {
                    device_list,
                };
                Command::none()
            }

            Message::DeviceFound(Err(e)) => {
                Command::none()
            }
            _ => Command::none(),
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = match self {
            Windows::GroupList {group_list, refresh} => Column::new()
                .max_width(1200)
                .spacing(20)
                .align_items(Align::Start)
                .push(group_list.view()),
            Windows::DeviceList {device_list} => Column::new()
                .max_width(1200)
                .spacing(20)
                .align_items(Align::Start)
                .push(device_list.view()),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
//            .center_y()
            .into()
    }
}

pub fn my_button<'a>(state: &'a mut button::State, text: &str) -> Button<'a, Message> {
    Button::new(state, Text::new(text).color(Color::WHITE))
        .background(Color::from_rgb(0.11, 0.42, 0.87))
        .border_radius(10)
        .padding(10)
}
