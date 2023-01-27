use std::convert::{TryFrom, TryInto};

use crate::{cli::project::run::CliRunCommand, CliError};

mod command;
mod flag;
pub mod output;
mod pipe;
mod shell;

use command::Command;
use output::Output;
use pipe::Pipe;

use super::BENCHER_CMD;

#[derive(Debug)]
pub enum Runner {
    Pipe(Pipe),
    Command(Command),
}

impl TryFrom<CliRunCommand> for Runner {
    type Error = CliError;

    fn try_from(command: CliRunCommand) -> Result<Self, Self::Error> {
        if let Some(cmd) = command.cmd {
            Ok(Self::Command(Command::try_from((command.shell, cmd))?))
        } else if let Ok(cmd) = std::env::var(BENCHER_CMD) {
            Ok(Self::Command(Command::try_from((command.shell, cmd))?))
        } else if let Some(pipe) = Pipe::new() {
            Ok(Self::Pipe(pipe))
        } else {
            Err(CliError::NoCommand)
        }
    }
}

impl Runner {
    pub fn run(&self) -> Result<Output, CliError> {
        Ok(match self {
            Self::Pipe(pipe) => pipe.output(),
            Self::Command(command) => command.try_into()?,
        })
    }
}
