use std::vec;

pub struct ChatState {
    pub name: String,
    pub address: String,
    pub current_channel: String,
    available_channels: Vec<String>,
}

impl ChatState {
    pub fn new(name: String, address: String) -> ChatState {
        ChatState {
            name,
            address,
            current_channel: String::from("general"),
            available_channels: vec![String::from("general")],
        }
    }

    pub fn filter_msg(&self, line: &String) -> bool {
        let segments = line.split("/").collect::<Vec<&str>>();
        let message_channel = segments[0];

        // if self
        //     .available_channels
        //     .iter()
        //     .any(|e| message_channel.contains(e))
        // {
        //     return true;
        // }
        if self.current_channel.contains(message_channel) {
            return true;
        } else if self.name.contains(message_channel) {
            return true;
        }

        false
    }

    pub fn process(&mut self, line: &String) -> bool {
        if line.contains("/c") {
            println!("Contains /c");
            let segments = line.split(" ").collect::<Vec<&str>>();
            let new_channel = segments[1].replace("\n", "");
            self.current_channel = String::from(new_channel);

            return false;
        }
        return true;
    }

    pub fn create_out(&self, user_line: &String) -> String {
        let line = format!("{}/{}> {}", self.current_channel, self.name, user_line);
        line
    }
}
