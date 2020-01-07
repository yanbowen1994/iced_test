use iced::{button, Align, Application, Checkbox, Column, Command, Container, Element, Length, Row, Settings, Subscription, Text, Image, HorizontalAlignment, Background, Color, Button, Sandbox, VerticalAlignment};
use crate::message::Message;
use crate::my_button;

#[derive(Clone, Debug, Default)]
struct Group {
    name: String,
    script: String,
    route_online: u32,
    route_offline: u32,
    pc_online: u32,
    pc_offline: u32,
    watching: button::State
}

impl Group {
    pub fn view(&mut self) -> Element<Message> {
        let top = Row::new()
            .align_items(Align::Center)
            .spacing(20)
            .padding(4)
            .push(Image::new("./resources/logo.png")
                .height(Length::Units(32))
                .width(Length::Units(32))
            )
            .push(
                Button::new(&mut self.watching,
                            Text::new(&self.name).size(24))
                    .on_press(Message::SelectGroup(self.name.clone()))
                    .padding(5)
            );

        let info = Row::new()
            .spacing(1)
            .push(
                Image::new("./resources/router.png")
                    .height(Length::Units(24))
                    .width(Length::Units(24))
            )
            .push(
                Text::new(format!("online:{} offline:{}",
                                    &self.route_online,
                                    &self.route_offline)
                )
                    .size(16)
//                    .horizontal_alignment(HorizontalAlignment::Center)
//                    .vertical_alignment(VerticalAlignment::Bottom)
            )
            .push(
                Image::new("./resources/device.png")
                    .height(Length::Units(20))
                    .width(Length::Units(20))
            )
            .push(
                Text::new(format!("online:{} offline:{}",
                        &self.pc_online,
                        &self.pc_offline)
                )
                    .size(16)
//                    .horizontal_alignment(HorizontalAlignment::Center)
            );

        let window = Row::new()
            .spacing(0)
            .push(top)
            .push(info)
            .into();
        window
    }
}

#[derive(Debug, Default)]
pub struct GroupList {
    refresh: button::State,
    groups: Vec<Group>,
}

impl GroupList {
    pub fn new() -> Self {
        let mut a = Self::default();
        add_test_group(&mut a);
        a
    }

    fn title(&self) -> String {
        String::from("Tinc-gui")
    }

    pub fn view(&mut self) -> Element<Message> {
        let title = Text::new("Group List")
            .size(50)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Left);

        let top = Row::new()
            .push(title)
            .push(
                my_button(&mut self.refresh, "Refresh")
                    .on_press(Message::None),
            )
            .align_items(Align::End);

        let group_list: Element<_> = self.groups
            .iter_mut()
            .enumerate()
            .fold(Column::new().spacing(30),
                |column, (i, group)| {
                    column.push(group.view())
                }
            )
            .into();

        let window = Column::new()
            .spacing(40)
            .padding(20)
            .push(top)
            .push(group_list);

        window.into()
    }
}

fn add_test_group(group_list: &mut GroupList) {
    let group1 = Group {
        name: "Huayi1".to_string(),
        script: "lalalal".to_string(),
        route_online: 1,
        route_offline: 2,
        pc_online: 3,
        pc_offline: 4,
        watching: button::State::new(),
    };

    let group2 = Group {
        name: "Ybw".to_string(),
        script: "sb".to_string(),
        route_online: 5,
        route_offline: 6,
        pc_online: 7,
        pc_offline: 8,
        watching: button::State::new(),
    };
    group_list.groups = vec![group1, group2];
}