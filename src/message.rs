use crate::device_list::{Error, DeviceList};

#[derive(Debug, Clone)]
pub enum Message {
    None,
    Refresh,
    BackGroupList,
    Disconnect(String),
    SelectGroup(String),
    DeviceFound(Result<DeviceList, Error>),
}