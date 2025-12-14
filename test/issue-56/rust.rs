#[derive(serde_repr::Serialize_repr)]
#[repr(u16)]
#[tsync]
pub enum KeyCode {
    Space = 32,
    Apostrophe = 39,
    Comma = 44,
    Minus = 45,
}
