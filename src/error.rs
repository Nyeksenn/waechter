/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use smol::channel::{RecvError, SendError};
use smol::io;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum CommError {
    IoError(io::Error),
    SendError(SendError<String>),
    RecvError(RecvError),
}

impl fmt::Display for CommError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Send of the read path failed!")
    }
}

impl error::Error for CommError {}

impl From<io::Error> for CommError {
    fn from(err: io::Error) -> Self {
        CommError::IoError(err)
    }
}

impl From<SendError<String>> for CommError {
    fn from(err: SendError<String>) -> Self {
        CommError::SendError(err)
    }
}

impl From<RecvError> for CommError {
    fn from(err: RecvError) -> Self {
        CommError::RecvError(err)
    }
}
