use std::sync::mpsc::Receiver;

use midir::*;

#[repr(u8)]
#[derive(Debug, Clone, Copy)]
pub enum Channel {
    Ch1 = 0,
    Ch2 = 1,
    Ch3 = 2,
    Ch4 = 3,
    Ch5 = 4,
    Ch6 = 5,
    Ch7 = 6,
    Ch8 = 7,
    Ch9 = 8,
    Ch10 = 9,
    Ch11 = 10,
    Ch12 = 11,
    Ch13 = 12,
    Ch14 = 13,
    Ch15 = 14,
    Ch16 = 15,
}

impl Channel {
    fn from_u8(ch: u8) -> Result<Self, ()> {
        match ch {
            0 => Ok(Channel::Ch1),
            1 => Ok(Channel::Ch2),
            2 => Ok(Channel::Ch3),
            3 => Ok(Channel::Ch4),
            4 => Ok(Channel::Ch5),
            5 => Ok(Channel::Ch6),
            6 => Ok(Channel::Ch7),
            7 => Ok(Channel::Ch8),
            8 => Ok(Channel::Ch9),
            9 => Ok(Channel::Ch10),
            10 => Ok(Channel::Ch11),
            11 => Ok(Channel::Ch12),
            12 => Ok(Channel::Ch13),
            13 => Ok(Channel::Ch14),
            14 => Ok(Channel::Ch15),
            15 => Ok(Channel::Ch16),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum MidiMessage {
    SysEx,
    Note(Channel, u8, u8),
    ControlChange(Channel, u8, u8),
}

impl MidiMessage {
    fn parse(data: &[u8]) -> Result<Self, ()> {
        match data {
            [op @ 0b10010000..=0b10011111, a, b] => {
                Ok(Self::Note(Channel::from_u8(*op & 0xf)?, *a, *b))
            }
            [op @ 0b10110000..=0b10111111, a, b] => {
                Ok(Self::ControlChange(Channel::from_u8(*op & 0xf)?, *a, *b))
            }
            [240, .., 247] => Ok(Self::SysEx),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Button {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy)]
pub enum AxiomMessage {
    Piano(u8, u8),
    // first value in range 1..=8
    Pad(u8, u8),
    // first value in range 1..=8
    Knob(u8, u8),

    Sustain(Button),
    Center(Button),
    Left(Button),
    Right(Button),
    Up(Button),
    Down(Button),
    Stop(Button),
    Play(Button),
    Record(Button),
}

impl AxiomMessage {
    fn parse(port: usize, msg: MidiMessage) -> Result<Self, ()> {
        Ok(match (port, msg) {
            (1, MidiMessage::Note(Channel::Ch1, k, v)) => Self::Piano(k, v),
            (1, MidiMessage::ControlChange(Channel::Ch1, 64, 127)) => {
                Self::Sustain(Button::Pressed)
            }
            (1, MidiMessage::ControlChange(Channel::Ch1, 64, 0)) => {
                Self::Sustain(Button::Released)
            }
            (2, MidiMessage::ControlChange(Channel::Ch16, k, 127)) => match k {
                98 => Self::Center(Button::Pressed),
                99 => Self::Left(Button::Pressed),
                100 => Self::Right(Button::Pressed),
                101 => Self::Up(Button::Pressed),
                102 => Self::Down(Button::Pressed),
                116 => Self::Stop(Button::Pressed),
                117 => Self::Play(Button::Pressed),
                118 => Self::Record(Button::Pressed),
                _ => Err(())?,
            },
            (2, MidiMessage::ControlChange(Channel::Ch16, k, 0)) => match k {
                98 => Self::Center(Button::Released),
                99 => Self::Left(Button::Released),
                100 => Self::Right(Button::Released),
                101 => Self::Up(Button::Released),
                102 => Self::Down(Button::Released),
                116 => Self::Stop(Button::Released),
                117 => Self::Play(Button::Released),
                118 => Self::Record(Button::Released),
                _ => Err(())?,
            },
            (2, MidiMessage::Note(Channel::Ch16, k @ 81..=88, v)) => {
                Self::Pad(k - 80, v)
            }
            (2, MidiMessage::ControlChange(Channel::Ch16, k @ 33..=40, v)) => {
                Self::Knob(k - 32, v)
            }
            _ => Err(())?,
        })
    }
}

fn get_input_port<'a>(
    midi_in: &MidiInput,
    device: &str,
) -> Result<MidiInputPort, PortInfoError> {
    for p in midi_in.ports() {
        if midi_in.port_name(&p)? == device {
            return Ok(p);
        }
    }
    Err(PortInfoError::InvalidPort)
}

fn get_output_port<'a>(
    midi_out: &MidiOutput,
    device: &str,
) -> Result<MidiOutputPort, PortInfoError> {
    for p in midi_out.ports() {
        if midi_out.port_name(&p)? == device {
            return Ok(p);
        }
    }
    Err(PortInfoError::InvalidPort)
}

fn connect_input<F>(
    name: &str,
    device: &str,
    callback: F,
) -> Result<MidiInputConnection<()>, Box<dyn std::error::Error>>
where
    F: FnMut(u64, &[u8], &mut ()) + Send + 'static,
{
    let mut midi_in = MidiInput::new("test")?;
    midi_in.ignore(Ignore::None);
    let port = get_input_port(&midi_in, device)?;
    Ok(midi_in.connect(&port, name, callback, ())?)
}

pub struct AxiomAirController {
    conn_out: MidiOutputConnection,
    _midi1: MidiInputConnection<()>,
    _midi2: MidiInputConnection<()>,
}

impl AxiomAirController {
    pub fn new(
    ) -> Result<(Self, Receiver<AxiomMessage>), Box<dyn std::error::Error>>
    {
        let midi_out = MidiOutput::new("ksp")?;
        let out_port =
            get_output_port(&midi_out, "Axiom AIR Mini 32 HyperCtrl")?;
        let mut conn_out = midi_out.connect(&out_port, "midir-write-output")?;
        // Enable HyperCtrl
        // Switches pads and other controls to port 2, channel 16
        conn_out
            .send(&[0xf0, 0x00, 0x01, 0x05, 0x20, 0x7f, 0x20, 0x3c, 0xf7])?;

        let (sender, recv) = std::sync::mpsc::channel();
        let sender2 = sender.clone();
        let midi1 = connect_input(
            "ksp1",
            "Axiom AIR Mini 32 MIDI In",
            move |_, message, _| {
                if let Ok(message) = MidiMessage::parse(message) {
                    if let Ok(message) = AxiomMessage::parse(1, message) {
                        let _ = sender.send(message);
                    }
                }
            },
        )?;
        let midi2 = connect_input(
            "ksp2",
            "Axiom AIR Mini 32 HyperCtrl",
            move |_, message, _| {
                if let Ok(message) = MidiMessage::parse(message) {
                    if let Ok(message) = AxiomMessage::parse(2, message) {
                        let _ = sender2.send(message);
                    }
                }
            },
        )?;
        Ok((
            Self {
                conn_out,
                _midi1: midi1,
                _midi2: midi2,
            },
            recv,
        ))
    }

    // Disable HyperCtrl
    // Resets pad (and others)'s port and channel
    fn _close(&mut self) -> Result<(), SendError> {
        self.conn_out
            .send(&[0xf0, 0x00, 0x01, 0x05, 0x20, 0x7f, 0x20, 0x00, 0xf7])
    }

    pub fn close(mut self) -> Result<(), SendError> {
        self._close()
    }
}

impl Drop for AxiomAirController {
    fn drop(&mut self) {
        let _ = self._close();
    }
}
