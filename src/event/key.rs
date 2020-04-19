#[derive(Debug, Hash, Eq, PartialEq)]
pub enum KeyCode {
    Backspace,
    Tab,
    Enter,
    Shift,
    Ctrl,
    Alt,
    Escape,
    Space,
    Pgup,
    Pgdown,
    End,
    Home,
    Left,
    Up,
    Right,
    Down,
    Insert,
    Delete,
    QwertyEquals,
    QwertyMinus,
    QwertyTilde,
    QwertyBracketLeft,
    QwertyBracketRight,
    QwertySemicolon,
    QwertyQuote,
    QwertyBackslash,
    QwertyComma,
    QwertyPeriod,
    QwertySlash,
    IntlBackslash,
    LeftWindowKey,
    RightWindowKey,
    ContextMenu,
    PrintScreen,
    PauseBreak,
    CapsLock,
    NumLock,
    ScrollLock,
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
    A,
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
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    NumpadMult,
    NumpadAdd,
    NumpadEnter,
    NumpadSub,
    NumpadDot,
    NumpadDiv,
    MouseLeft,
    MouseRight,
    MouseMiddle,
    MouseBack,
    MouseForward,
    MouseWheelUp,
    MouseWheelDown,
    LocLeft,
    LocRight,
    Unknown,
}

impl From<crate::wasm::key::KeyCode> for KeyCode {
    fn from(key_code: crate::wasm::key::KeyCode) -> KeyCode {
        match key_code {
            crate::wasm::key::BACKSPACE => KeyCode::Backspace,
            crate::wasm::key::TAB => KeyCode::Tab,
            crate::wasm::key::ENTER => KeyCode::Enter,
            crate::wasm::key::SHIFT => KeyCode::Shift,
            crate::wasm::key::CTRL => KeyCode::Ctrl,
            crate::wasm::key::ALT => KeyCode::Alt,
            crate::wasm::key::ESCAPE => KeyCode::Escape,
            crate::wasm::key::SPACE => KeyCode::Space,
            crate::wasm::key::PGUP => KeyCode::Pgup,
            crate::wasm::key::PGDOWN => KeyCode::Pgdown,
            crate::wasm::key::END => KeyCode::End,
            crate::wasm::key::HOME => KeyCode::Home,
            crate::wasm::key::LEFT => KeyCode::Left,
            crate::wasm::key::UP => KeyCode::Up,
            crate::wasm::key::RIGHT => KeyCode::Right,
            crate::wasm::key::DOWN => KeyCode::Down,
            crate::wasm::key::INSERT => KeyCode::Insert,
            crate::wasm::key::DELETE => KeyCode::Delete,
            crate::wasm::key::QWERTY_EQUALS => KeyCode::QwertyEquals,
            crate::wasm::key::QWERTY_MINUS => KeyCode::QwertyMinus,
            crate::wasm::key::QWERTY_TILDE => KeyCode::QwertyTilde,
            crate::wasm::key::QWERTY_BRACKET_LEFT => KeyCode::QwertyBracketLeft,
            crate::wasm::key::QWERTY_BRACKET_RIGHT => KeyCode::QwertyBracketRight,
            crate::wasm::key::QWERTY_SEMICOLON => KeyCode::QwertySemicolon,
            crate::wasm::key::QWERTY_QUOTE => KeyCode::QwertyQuote,
            crate::wasm::key::QWERTY_BACKSLASH => KeyCode::QwertyBackslash,
            crate::wasm::key::QWERTY_COMMA => KeyCode::QwertyComma,
            crate::wasm::key::QWERTY_PERIOD => KeyCode::QwertyPeriod,
            crate::wasm::key::QWERTY_SLASH => KeyCode::QwertySlash,
            crate::wasm::key::INTL_BACKSLASH => KeyCode::IntlBackslash,
            crate::wasm::key::LEFT_WINDOW_KEY => KeyCode::LeftWindowKey,
            crate::wasm::key::RIGHT_WINDOW_KEY => KeyCode::RightWindowKey,
            crate::wasm::key::CONTEXT_MENU => KeyCode::ContextMenu,
            crate::wasm::key::PAUSE_BREAK => KeyCode::PauseBreak,
            crate::wasm::key::CAPS_LOCK => KeyCode::CapsLock,
            crate::wasm::key::NUM_LOCK => KeyCode::NumLock,
            crate::wasm::key::SCROLL_LOCK => KeyCode::ScrollLock,
            crate::wasm::key::NUMPAD_0 => KeyCode::Numpad0,
            crate::wasm::key::NUMPAD_1 => KeyCode::Numpad1,
            crate::wasm::key::NUMPAD_2 => KeyCode::Numpad2,
            crate::wasm::key::NUMPAD_3 => KeyCode::Numpad3,
            crate::wasm::key::NUMPAD_4 => KeyCode::Numpad4,
            crate::wasm::key::NUMPAD_5 => KeyCode::Numpad5,
            crate::wasm::key::NUMPAD_6 => KeyCode::Numpad6,
            crate::wasm::key::NUMPAD_7 => KeyCode::Numpad7,
            crate::wasm::key::NUMPAD_8 => KeyCode::Numpad8,
            crate::wasm::key::NUMPAD_9 => KeyCode::Numpad9,
            crate::wasm::key::A => KeyCode::A,
            crate::wasm::key::B => KeyCode::B,
            crate::wasm::key::C => KeyCode::C,
            crate::wasm::key::D => KeyCode::D,
            crate::wasm::key::E => KeyCode::E,
            crate::wasm::key::F => KeyCode::F,
            crate::wasm::key::G => KeyCode::G,
            crate::wasm::key::H => KeyCode::H,
            crate::wasm::key::I => KeyCode::I,
            crate::wasm::key::J => KeyCode::J,
            crate::wasm::key::K => KeyCode::K,
            crate::wasm::key::L => KeyCode::L,
            crate::wasm::key::M => KeyCode::M,
            crate::wasm::key::N => KeyCode::N,
            crate::wasm::key::O => KeyCode::O,
            crate::wasm::key::P => KeyCode::P,
            crate::wasm::key::Q => KeyCode::Q,
            crate::wasm::key::R => KeyCode::R,
            crate::wasm::key::S => KeyCode::S,
            crate::wasm::key::T => KeyCode::T,
            crate::wasm::key::U => KeyCode::U,
            crate::wasm::key::V => KeyCode::V,
            crate::wasm::key::W => KeyCode::W,
            crate::wasm::key::X => KeyCode::X,
            crate::wasm::key::Y => KeyCode::Y,
            crate::wasm::key::Z => KeyCode::Z,
            crate::wasm::key::F1 => KeyCode::F1,
            crate::wasm::key::F2 => KeyCode::F2,
            crate::wasm::key::F3 => KeyCode::F3,
            crate::wasm::key::F4 => KeyCode::F4,
            crate::wasm::key::F5 => KeyCode::F5,
            crate::wasm::key::F6 => KeyCode::F6,
            crate::wasm::key::F7 => KeyCode::F7,
            crate::wasm::key::F8 => KeyCode::F8,
            crate::wasm::key::F9 => KeyCode::F9,
            crate::wasm::key::F10 => KeyCode::F10,
            crate::wasm::key::F11 => KeyCode::F11,
            crate::wasm::key::F12 => KeyCode::F12,
            crate::wasm::key::F13 => KeyCode::F13,
            crate::wasm::key::F14 => KeyCode::F14,
            crate::wasm::key::F15 => KeyCode::F15,
            crate::wasm::key::F16 => KeyCode::F16,
            crate::wasm::key::F17 => KeyCode::F17,
            crate::wasm::key::F18 => KeyCode::F18,
            crate::wasm::key::F19 => KeyCode::F19,
            crate::wasm::key::F20 => KeyCode::F20,
            crate::wasm::key::F21 => KeyCode::F21,
            crate::wasm::key::F22 => KeyCode::F22,
            crate::wasm::key::F23 => KeyCode::F23,
            crate::wasm::key::F24 => KeyCode::F24,
            crate::wasm::key::NUMPAD_MULT => KeyCode::NumpadMult,
            crate::wasm::key::NUMPAD_ADD => KeyCode::NumpadAdd,
            crate::wasm::key::NUMPAD_ENTER => KeyCode::NumpadEnter,
            crate::wasm::key::NUMPAD_SUB => KeyCode::NumpadSub,
            crate::wasm::key::NUMPAD_DOT => KeyCode::NumpadDot,
            crate::wasm::key::NUMPAD_DIV => KeyCode::NumpadDiv,
            crate::wasm::key::MOUSE_LEFT => KeyCode::MouseLeft,
            crate::wasm::key::MOUSE_RIGHT => KeyCode::MouseRight,
            crate::wasm::key::MOUSE_MIDDLE => KeyCode::MouseMiddle,
            crate::wasm::key::MOUSE_BACK => KeyCode::MouseBack,
            crate::wasm::key::MOUSE_FORWARD => KeyCode::MouseForward,
            crate::wasm::key::MOUSE_WHEEL_UP => KeyCode::MouseWheelUp,
            crate::wasm::key::MOUSE_WHEEL_DOWN => KeyCode::MouseWheelDown,
            crate::wasm::key::LOC_LEFT => KeyCode::LocLeft,
            crate::wasm::key::LOC_RIGHT => KeyCode::LocRight,
            _ => KeyCode::Unknown,
        }
    }
}
