// A data structure that represents the last 256 keys pressed
#[derive(Debug, Clone, Copy)]
pub struct KeyQueue {
    pub queue: [Option<KeyCode>; 256],
    pub index: usize,
}