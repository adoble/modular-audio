use source::Source;

pub struct SourceChannelMap {
    mappings: [(Source, u8); 5],
}

impl SourceChannelMap {
    pub fn new() -> Self {
        Self {
            mappings: [(None, 0); 5],
        }
    }

    pub fn add_mapping(&mut self, source: Source, channel: u8) {
        for (src, chan) in self.mappings.iter_mut() {
            if *src == source {
                *chan = channel;
                return;
            }
        }
    }

    pub fn get_channel(&self, source: Source) -> Option<u8> {
        for (src, chan) in self.mappings.iter() {
            if *src == source {
                return Some(*chan);
            }
        }
        None
    }
}
