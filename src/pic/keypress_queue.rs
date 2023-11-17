// use pc_keyboard::DecodedKey;

// use super::keyboard::KEY_QUEUES;

// #[derive(Debug, Clone, Copy)]
// pub struct KeyQueue {
//     pub queue: [Option<DecodedKey>; 256],
//     pub index: usize,
// }

// impl KeyQueue {
//     pub fn new() -> KeyQueue {
//         // Create a new KeyQueue and add it to KEY_QUEUES
//         let mut key_queues = KEY_QUEUES.lock();
//         let key_queue = KeyQueue {
//             queue: [None; 256],
//             index: 0,
//         };

//         // Add the KeyQueue to KEY_QUEUES
//         key_queues.add(key_queue);
        
//         // Return a mutable reference to the KeyQueue
//         let index = key_queues.index - 1;
//         *key_queues.get_queue_mut(index)
//     }

//     fn shift_down(&mut self) {
//         // Shift all the keys down one
//         for i in 0..self.index - 1 {
//             self.queue[i] = self.queue[i + 1];
//         }

//         // Decrement the index
//         self.index -= 1;
//     }

//     pub fn is_empty(&self) -> bool {
//         self.index == 0
//     }

//     pub fn queue(&mut self, key: DecodedKey) {
//         // Check if the queue is full
//         if self.index == 256 {
//             self.shift_down();
//         }
        
//         // Push the key
//         self.queue[self.index] = Some(key);
//         self.index += 1;
//     }

//     pub fn dequeue(&mut self) -> Option<DecodedKey> {
//         let key = self.queue[0];
//         self.shift_down();
//         key
//     }
// }