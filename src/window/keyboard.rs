/*!
* Keyboard inputs
* 
* Give acces to real-time keyboard input.
*
*/

use core::libc::{c_int};

/// Key codes
pub enum Key { 
    Unknown = -1, 
    A = 0, 
    B, 
    C, 
    D, 
    E, 
    F, 
    G, 
    H, 
    I, 
    J, 
    K, 
    L, 
    M, 
    N, 
    O, 
    P, 
    Q, 
    R, 
    S, 
    T, 
    U, 
    V, 
    W, 
    X, 
    Y, 
    Z, 
    Num0, 
    Num1, 
    Num2, 
    Num3, 
    Num4, 
    Num5, 
    Num6, 
    Num7, 
    Num8, 
    Num9, 
    Escape, 
    LControl, 
    LShift, 
    LAlt, 
    LSystem, 
    RControl, 
    RShift, 
    RAlt, 
    RSystem, 
    Menu, 
    LBracket, 
    RBracket, 
    SemiColon, 
    Comma, 
    Period, 
    Quote, 
    Slash, 
    BackSlash, 
    Tilde, 
    Equal, 
    Dash, 
    Space, 
    Return, 
    BackSpace, 
    Tab, 
    PageUp, 
    PageDown, 
    End, 
    Home, 
    Insert, 
    Delete, 
    Add, 
    Subtract, 
    Multiply, 
    Divide, 
    Left, 
    Right, 
    Up, 
    Down, 
    Numpad0, 
    Numpad1, 
    Numpad2, 
    Numpad3, 
    Numpad4, 
    Numpad5, 
    Numpad6, 
    Numpad7, 
    Numpad8, 
    Numpad9, 
    F1, 
    F2, 
    F3, 
    F4, 
    F5, 
    F6, 
    F7, 
    F8, 
    F9, 
    F10, 
    F11, 
    F12, 
    F13, 
    F14, 
    F15, 
    Pause, 
    KeyCount 
}

#[doc(hidden)]
pub mod csfml {
    
    pub use core::libc::{c_int};
    use rsfml::sfTypes::{sfBool};

    pub extern "C" {
        fn sfKeyboard_isKeyPressed(key : c_int) -> sfBool;
    }
}

/**
 * Check if a key is pressed.
 *
 * Return true if the key is pressed, false otherwise.
 */
pub fn keyboard_is_key_pressed(key : Key) -> bool {
    unsafe {
        match csfml::sfKeyboard_isKeyPressed(key as c_int) {
            0   => false,
            _   => true
        }
    }
}