use std::str::FromStr;

pub enum Command {
    GetName,
    GetStatusText,
    GetStatus,
    GetPowerConsumption,
    TurnOn,
    TurnOff,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(input: &str) -> Result<Command, Self::Err> {
        match input {
            "get_name" => Ok(Command::GetName),
            "get_status_text" => Ok(Command::GetStatusText),
            "get_status" => Ok(Command::GetStatus),
            "get_power_consumption" => Ok(Command::GetPowerConsumption),
            "turn_on" => Ok(Command::TurnOn),
            "turn_off" => Ok(Command::TurnOff),
            _ => Err(()),
        }
    }
}
