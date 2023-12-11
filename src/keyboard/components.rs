use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct KeyBoardButton(pub char);

#[derive(Component, Debug)]
pub struct Capitalizable;

#[derive(Component, PartialEq, Clone)]
pub enum KeyType {
    Letter,
    Function,
    Number,
}

#[derive(Component, Debug)]
//pub struct KeyBoardButton(char);
pub struct KeyBoard;

#[derive(Component, Debug)]
//pub struct KeyBoardButton(char);
pub struct KeyboardNode;
