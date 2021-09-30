use crate::{Input, Output, PlaybackStatus};
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
    SendPlaybackStatus(std::sync::mpsc::SendError<PlaybackStatus>),
    Simple(&'static str),
}

impl std::error::Error for SpotnixError {}
