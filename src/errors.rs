use crate::{Event, Input, Output};
use derive_more::{Display, From};
use std::str::ParseBoolError;

#[derive(Debug, Display, From)]
pub enum SpotnixError {
    Parse(String),
    ParseBool(ParseBoolError),
    IO(std::io::Error),
    Fail(failure::Error),
    SendOutput(std::sync::mpsc::SendError<Output>),
    SendInput(std::sync::mpsc::SendError<Input>),
    SendEvent(std::sync::mpsc::SendError<Event>),
    Simple(&'static str),
}

impl std::error::Error for SpotnixError {}
