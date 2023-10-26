use enigo::{Enigo, KeyboardControllable};

pub type EnigoKey = enigo::Key;

pub trait KeyboardAdapter {
    fn key_sequence(&mut self, sequence: &str);
    fn key_click(&mut self, key: EnigoKey);
    fn key_sequence_parse(&mut self, sequence: &str);
    // fn key_down(&mut self, key: EnigoKey);
    // fn key_up(&mut self, key: EnigoKey);
}

// TODO: study each fn works in Enigo
impl KeyboardAdapter for Enigo {
    fn key_sequence(&mut self, sequence: &str) {
        KeyboardControllable::key_sequence(self, sequence)
    }
    fn key_click(&mut self, key: EnigoKey) {
        KeyboardControllable::key_click(self, key)
    }
    fn key_sequence_parse(&mut self, sequence: &str) {
        KeyboardControllable::key_sequence_parse(self, sequence)
    }
    // fn key_down(&mut self, key: EnigoKey) {
    //     todo!()
    // }
    // fn key_up(&mut self, key: EnigoKey) {
    //     todo!()
    // }
}
