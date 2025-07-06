use crate::{command::Command, tv::TV};

// TODO: Define the TVChangeChannelCommand struct and implement Command for it
// - It should store the target channel and previous channel
// - The execute method should change the TV channel
// - The undo method should revert the TV to the previous channel and print "Undo: TV channel changed back to {previous_channel}."
pub struct TVChangeChannelCommand {
    tv: TV,
    target_channel: u32,
    previous_channel: Option<u32>,
}

impl TVChangeChannelCommand {
    pub fn new(channel: u32) -> Self {
        TVChangeChannelCommand {
            tv: TV::new(),
            target_channel: channel,
            previous_channel: None,
        }
    }
}

impl Command for TVChangeChannelCommand {
    fn execute(&mut self, tv: &mut TV) {
        self.previous_channel = Some(tv.get_current_channel());
        tv.change_channel(self.target_channel);
    }
    fn undo(&mut self, tv: &mut TV) {
        if let Some(prev_channel) = self.previous_channel {
            tv.change_channel(prev_channel);
            println!("Undo: TV channel changed back to {prev_channel}.");
        }
    }
}
