use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct KeyBoardButton(pub char, pub char);

#[derive(Component, Debug)]
pub struct Changeable;

// #[derive(Component, PartialEq, Clone)]
// pub enum KeyType {
//     Letter,
//     Function,
//     Number,
// }

// #[derive(Component, Debug)]
// pub struct KeyBoardKey(pub String, pub String);

#[derive(Component, Debug)]
//pub struct KeyBoardButton(char);
pub struct KeyBoard;

#[derive(Component, Debug)]
//pub struct KeyBoardButton(char);
pub struct KeyboardNode;

#[derive(Component, Debug)]
pub struct NumberKeyboardNode;
