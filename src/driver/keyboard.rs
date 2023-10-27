use crate::keyboard::Key;
use enigo::{Enigo, KeyboardControllable};

pub type EnigoKey = enigo::Key;

// TODO: find a way yo replace this horrible enum to same enum conversion
impl From<Key> for EnigoKey {
    fn from(value: Key) -> Self {
        match value {
            #[cfg(target_os = "windows")]
            Key::Num0 => EnigoKey::Num0,
            #[cfg(target_os = "windows")]
            Key::Num1 => EnigoKey::Num1,
            #[cfg(target_os = "windows")]
            Key::Num2 => EnigoKey::Num2,
            #[cfg(target_os = "windows")]
            Key::Num3 => EnigoKey::Num3,
            #[cfg(target_os = "windows")]
            Key::Num4 => EnigoKey::Num4,
            #[cfg(target_os = "windows")]
            Key::Num5 => EnigoKey::Num5,
            #[cfg(target_os = "windows")]
            Key::Num6 => EnigoKey::Num6,
            #[cfg(target_os = "windows")]
            Key::Num7 => EnigoKey::Num7,
            #[cfg(target_os = "windows")]
            Key::Num8 => EnigoKey::Num8,
            #[cfg(target_os = "windows")]
            Key::Num9 => EnigoKey::Num9,
            #[cfg(target_os = "windows")]
            Key::A => EnigoKey::A,
            #[cfg(target_os = "windows")]
            Key::B => EnigoKey::B,
            #[cfg(target_os = "windows")]
            Key::C => EnigoKey::C,
            #[cfg(target_os = "windows")]
            Key::D => EnigoKey::D,
            #[cfg(target_os = "windows")]
            Key::E => EnigoKey::E,
            #[cfg(target_os = "windows")]
            Key::F => EnigoKey::F,
            #[cfg(target_os = "windows")]
            Key::G => EnigoKey::G,
            #[cfg(target_os = "windows")]
            Key::H => EnigoKey::H,
            #[cfg(target_os = "windows")]
            Key::I => EnigoKey::I,
            #[cfg(target_os = "windows")]
            Key::J => EnigoKey::J,
            #[cfg(target_os = "windows")]
            Key::K => EnigoKey::K,
            #[cfg(target_os = "windows")]
            Key::L => EnigoKey::L,
            #[cfg(target_os = "windows")]
            Key::M => EnigoKey::M,
            #[cfg(target_os = "windows")]
            Key::N => EnigoKey::N,
            #[cfg(target_os = "windows")]
            Key::O => EnigoKey::O,
            #[cfg(target_os = "windows")]
            Key::P => EnigoKey::P,
            #[cfg(target_os = "windows")]
            Key::Q => EnigoKey::Q,
            #[cfg(target_os = "windows")]
            Key::R => EnigoKey::R,
            #[cfg(target_os = "windows")]
            Key::S => EnigoKey::S,
            #[cfg(target_os = "windows")]
            Key::T => EnigoKey::T,
            #[cfg(target_os = "windows")]
            Key::U => EnigoKey::U,
            #[cfg(target_os = "windows")]
            Key::V => EnigoKey::V,
            #[cfg(target_os = "windows")]
            Key::W => EnigoKey::W,
            #[cfg(target_os = "windows")]
            Key::X => EnigoKey::X,
            #[cfg(target_os = "windows")]
            Key::Y => EnigoKey::Y,
            #[cfg(target_os = "windows")]
            Key::Z => EnigoKey::Z,
            #[cfg(target_os = "windows")]
            Key::AbntC1 => EnigoKey::AbntC1,
            #[cfg(target_os = "windows")]
            Key::AbntC2 => EnigoKey::AbntC2,
            #[cfg(target_os = "windows")]
            Key::Accept => EnigoKey::Accept,
            #[cfg(target_os = "windows")]
            Key::Add => EnigoKey::Add,
            /// alt key on Linux and Windows (option key on macOS)
            Key::Alt => EnigoKey::Alt,
            #[cfg(target_os = "windows")]
            Key::Apps => EnigoKey::Apps,
            #[cfg(target_os = "windows")]
            Key::Attn => EnigoKey::Attn,
            /// backspace key
            Key::Backspace => EnigoKey::Backspace,
            #[cfg(target_os = "linux")]
            Key::Begin => EnigoKey::Begin,
            #[cfg(target_os = "linux")]
            Key::Break => EnigoKey::Break,
            #[cfg(target_os = "windows")]
            Key::BrowserBack => EnigoKey::BrowserBack,
            #[cfg(target_os = "windows")]
            Key::BrowserFavorites => EnigoKey::BrowserFavorites,
            #[cfg(target_os = "windows")]
            Key::BrowserForward => EnigoKey::BrowserForward,
            #[cfg(target_os = "windows")]
            Key::BrowserHome => EnigoKey::BrowserHome,
            #[cfg(target_os = "windows")]
            Key::BrowserRefresh => EnigoKey::BrowserRefresh,
            #[cfg(target_os = "windows")]
            Key::BrowserSearch => EnigoKey::BrowserSearch,
            #[cfg(target_os = "windows")]
            Key::BrowserStop => EnigoKey::BrowserStop,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::Cancel => EnigoKey::Cancel,
            /// caps lock key
            Key::CapsLock => EnigoKey::CapsLock,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::Clear => EnigoKey::Clear,
            #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
            /// command key on macOS (super key on Key::Linux => EnigoKey::Linux, windows key on Windows)
            Key::Command => EnigoKey::Command,
            /// control key
            Key::Control => EnigoKey::Control,
            #[cfg(target_os = "windows")]
            Key::Convert => EnigoKey::Convert,
            #[cfg(target_os = "windows")]
            Key::Crsel => EnigoKey::Crsel,
            #[cfg(target_os = "windows")]
            Key::DBEAlphanumeric => EnigoKey::DBEAlphanumeric,
            #[cfg(target_os = "windows")]
            Key::DBECodeinput => EnigoKey::DBECodeinput,
            #[cfg(target_os = "windows")]
            Key::DBEDetermineString => EnigoKey::DBEDetermineString,
            #[cfg(target_os = "windows")]
            Key::DBEEnterDLGConversionMode => EnigoKey::DBEEnterDLGConversionMode,
            #[cfg(target_os = "windows")]
            Key::DBEEnterIMEConfigMode => EnigoKey::DBEEnterIMEConfigMode,
            #[cfg(target_os = "windows")]
            Key::DBEEnterWordRegisterMode => EnigoKey::DBEEnterWordRegisterMode,
            #[cfg(target_os = "windows")]
            Key::DBEFlushString => EnigoKey::DBEFlushString,
            #[cfg(target_os = "windows")]
            Key::DBEHiragana => EnigoKey::DBEHiragana,
            #[cfg(target_os = "windows")]
            Key::DBEKatakana => EnigoKey::DBEKatakana,
            #[cfg(target_os = "windows")]
            Key::DBENoCodepoint => EnigoKey::DBENoCodepoint,
            #[cfg(target_os = "windows")]
            Key::DBENoRoman => EnigoKey::DBENoRoman,
            #[cfg(target_os = "windows")]
            Key::DBERoman => EnigoKey::DBERoman,
            #[cfg(target_os = "windows")]
            Key::DBESBCSChar => EnigoKey::DBESBCSChar,
            #[cfg(target_os = "windows")]
            Key::DBESChar => EnigoKey::DBESChar,
            #[cfg(target_os = "windows")]
            Key::Decimal => EnigoKey::Decimal,
            /// delete key
            Key::Delete => EnigoKey::Delete,
            #[cfg(target_os = "windows")]
            Key::Divide => EnigoKey::Divide,
            /// down arrow key
            Key::DownArrow => EnigoKey::DownArrow,
            /// end key
            Key::End => EnigoKey::End,
            #[cfg(target_os = "windows")]
            Key::Ereof => EnigoKey::Ereof,
            /// escape key (esc)
            Key::Escape => EnigoKey::Escape,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::Execute => EnigoKey::Execute,
            #[cfg(target_os = "windows")]
            Key::Exsel => EnigoKey::Exsel,
            /// F1 key
            Key::F1 => EnigoKey::F1,
            /// F2 key
            Key::F2 => EnigoKey::F2,
            /// F3 key
            Key::F3 => EnigoKey::F3,
            /// F4 key
            Key::F4 => EnigoKey::F4,
            /// F5 key
            Key::F5 => EnigoKey::F5,
            /// F6 key
            Key::F6 => EnigoKey::F6,
            /// F7 key
            Key::F7 => EnigoKey::F7,
            /// F8 key
            Key::F8 => EnigoKey::F8,
            /// F9 key
            Key::F9 => EnigoKey::F9,
            /// F10 key
            Key::F10 => EnigoKey::F10,
            /// F11 key
            Key::F11 => EnigoKey::F11,
            /// F12 key
            Key::F12 => EnigoKey::F12,
            /// F13 key
            Key::F13 => EnigoKey::F13,
            /// F14 key
            Key::F14 => EnigoKey::F14,
            /// F15 key
            Key::F15 => EnigoKey::F15,
            /// F16 key
            Key::F16 => EnigoKey::F16,
            /// F17 key
            Key::F17 => EnigoKey::F17,
            /// F18 key
            Key::F18 => EnigoKey::F18,
            /// F19 key
            Key::F19 => EnigoKey::F19,
            /// F20 key
            Key::F20 => EnigoKey::F20,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            /// F21 key
            Key::F21 => EnigoKey::F21,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            /// F22 key
            Key::F22 => EnigoKey::F22,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            /// F23 key
            Key::F23 => EnigoKey::F23,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            /// F24 key
            Key::F24 => EnigoKey::F24,
            #[cfg(target_os = "linux")]
            Key::F25 => EnigoKey::F25,
            #[cfg(target_os = "linux")]
            Key::F26 => EnigoKey::F26,
            #[cfg(target_os = "linux")]
            Key::F27 => EnigoKey::F27,
            #[cfg(target_os = "linux")]
            Key::F28 => EnigoKey::F28,
            #[cfg(target_os = "linux")]
            Key::F29 => EnigoKey::F29,
            #[cfg(target_os = "linux")]
            Key::F30 => EnigoKey::F30,
            #[cfg(target_os = "linux")]
            Key::F31 => EnigoKey::F31,
            #[cfg(target_os = "linux")]
            Key::F32 => EnigoKey::F32,
            #[cfg(target_os = "linux")]
            Key::F33 => EnigoKey::F33,
            #[cfg(target_os = "linux")]
            Key::F34 => EnigoKey::F34,
            #[cfg(target_os = "linux")]
            Key::F35 => EnigoKey::F35,
            #[cfg(target_os = "macos")]
            Key::Function => EnigoKey::Function,
            #[cfg(target_os = "windows")]
            Key::Final => EnigoKey::Final,
            #[cfg(target_os = "linux")]
            Key::Find => EnigoKey::Find,
            #[cfg(target_os = "windows")]
            Key::GamepadA => EnigoKey::GamepadA,
            #[cfg(target_os = "windows")]
            Key::GamepadB => EnigoKey::GamepadB,
            #[cfg(target_os = "windows")]
            Key::GamepadDPadDown => EnigoKey::GamepadDPadDown,
            #[cfg(target_os = "windows")]
            Key::GamepadDPadLeft => EnigoKey::GamepadDPadLeft,
            #[cfg(target_os = "windows")]
            Key::GamepadDPadRight => EnigoKey::GamepadDPadRight,
            #[cfg(target_os = "windows")]
            Key::GamepadDPadUp => EnigoKey::GamepadDPadUp,
            #[cfg(target_os = "windows")]
            Key::GamepadLeftShoulder => EnigoKey::GamepadLeftShoulder,
            #[cfg(target_os = "windows")]
            Key::GamepadLeftThumbstickButton => EnigoKey::GamepadLeftThumbstickButton,
            #[cfg(target_os = "windows")]
            Key::GamepadLeftThumbstickDown => EnigoKey::GamepadLeftThumbstickDown,
            #[cfg(target_os = "windows")]
            Key::GamepadLeftThumbstickLeft => EnigoKey::GamepadLeftThumbstickLeft,
            #[cfg(target_os = "windows")]
            Key::GamepadLeftThumbstickRight => EnigoKey::GamepadLeftThumbstickRight,
            #[cfg(target_os = "windows")]
            Key::GamepadLeftThumbstickUp => EnigoKey::GamepadLeftThumbstickUp,
            #[cfg(target_os = "windows")]
            Key::GamepadLeftTrigger => EnigoKey::GamepadLeftTrigger,
            #[cfg(target_os = "windows")]
            Key::GamepadMenu => EnigoKey::GamepadMenu,
            #[cfg(target_os = "windows")]
            Key::GamepadRightShoulder => EnigoKey::GamepadRightShoulder,
            #[cfg(target_os = "windows")]
            Key::GamepadRightThumbstickButton => EnigoKey::GamepadRightThumbstickButton,
            #[cfg(target_os = "windows")]
            Key::GamepadRightThumbstickDown => EnigoKey::GamepadRightThumbstickDown,
            #[cfg(target_os = "windows")]
            Key::GamepadRightThumbstickLeft => EnigoKey::GamepadRightThumbstickLeft,
            #[cfg(target_os = "windows")]
            Key::GamepadRightThumbstickRight => EnigoKey::GamepadRightThumbstickRight,
            #[cfg(target_os = "windows")]
            Key::GamepadRightThumbstickUp => EnigoKey::GamepadRightThumbstickUp,
            #[cfg(target_os = "windows")]
            Key::GamepadRightTrigger => EnigoKey::GamepadRightTrigger,
            #[cfg(target_os = "windows")]
            Key::GamepadView => EnigoKey::GamepadView,
            #[cfg(target_os = "windows")]
            Key::GamepadX => EnigoKey::GamepadX,
            #[cfg(target_os = "windows")]
            Key::GamepadY => EnigoKey::GamepadY,
            #[cfg(target_os = "windows")]
            Key::Hangeul => EnigoKey::Hangeul,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::Hangul => EnigoKey::Hangul,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::Hanja => EnigoKey::Hanja,
            Key::Help => EnigoKey::Help,
            /// home key
            Key::Home => EnigoKey::Home,
            #[cfg(target_os = "windows")]
            Key::Ico00 => EnigoKey::Ico00,
            #[cfg(target_os = "windows")]
            Key::IcoClear => EnigoKey::IcoClear,
            #[cfg(target_os = "windows")]
            Key::IcoHelp => EnigoKey::IcoHelp,
            #[cfg(target_os = "windows")]
            Key::IMEOff => EnigoKey::IMEOff,
            #[cfg(target_os = "windows")]
            Key::IMEOn => EnigoKey::IMEOn,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::Insert => EnigoKey::Insert,
            #[cfg(target_os = "windows")]
            Key::Junja => EnigoKey::Junja,
            #[cfg(target_os = "windows")]
            Key::Kana => EnigoKey::Kana,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::Kanji => EnigoKey::Kanji,
            #[cfg(target_os = "windows")]
            Key::LaunchApp1 => EnigoKey::LaunchApp1,
            #[cfg(target_os = "windows")]
            Key::LaunchApp2 => EnigoKey::LaunchApp2,
            #[cfg(target_os = "windows")]
            Key::LaunchMail => EnigoKey::LaunchMail,
            #[cfg(target_os = "windows")]
            Key::LaunchMediaSelect => EnigoKey::LaunchMediaSelect,
            #[cfg(target_os = "macos")]
            /// Opens launchpad
            Key::Launchpad => EnigoKey::Launchpad,
            #[cfg(target_os = "windows")]
            Key::LButton => EnigoKey::LButton,
            Key::LControl => EnigoKey::LControl,
            /// left arrow key
            Key::LeftArrow => EnigoKey::LeftArrow,
            #[cfg(target_os = "linux")]
            Key::Linefeed => EnigoKey::Linefeed,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::LMenu => EnigoKey::LMenu,
            Key::LShift => EnigoKey::LShift,
            #[cfg(target_os = "windows")]
            Key::LWin => EnigoKey::LWin,
            #[cfg(target_os = "windows")]
            Key::MButton => EnigoKey::MButton,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::MediaNextTrack => EnigoKey::MediaNextTrack,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::MediaPlayPause => EnigoKey::MediaPlayPause,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::MediaPrevTrack => EnigoKey::MediaPrevTrack,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::MediaStop => EnigoKey::MediaStop,
            /// meta key (also known as "windows"Key::, => EnigoKey::" "super"Key::, => EnigoKey::" and "command")
            Key::Meta => EnigoKey::Meta,
            #[cfg(target_os = "macos")]
            /// Opens mission control
            Key::MissionControl => EnigoKey::MissionControl,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::ModeChange => EnigoKey::ModeChange,
            #[cfg(target_os = "windows")]
            Key::Multiply => EnigoKey::Multiply,
            #[cfg(target_os = "windows")]
            Key::NavigationAccept => EnigoKey::NavigationAccept,
            #[cfg(target_os = "windows")]
            Key::NavigationCancel => EnigoKey::NavigationCancel,
            #[cfg(target_os = "windows")]
            Key::NavigationDown => EnigoKey::NavigationDown,
            #[cfg(target_os = "windows")]
            Key::NavigationLeft => EnigoKey::NavigationLeft,
            #[cfg(target_os = "windows")]
            Key::NavigationMenu => EnigoKey::NavigationMenu,
            #[cfg(target_os = "windows")]
            Key::NavigationRight => EnigoKey::NavigationRight,
            #[cfg(target_os = "windows")]
            Key::NavigationUp => EnigoKey::NavigationUp,
            #[cfg(target_os = "windows")]
            Key::NavigationView => EnigoKey::NavigationView,
            #[cfg(target_os = "windows")]
            Key::NoName => EnigoKey::NoName,
            #[cfg(target_os = "windows")]
            Key::NonConvert => EnigoKey::NonConvert,
            #[cfg(target_os = "windows")]
            Key::None => EnigoKey::None,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::Numlock => EnigoKey::Numlock,
            #[cfg(target_os = "windows")]
            Key::Numpad0 => EnigoKey::Numpad0,
            #[cfg(target_os = "windows")]
            Key::Numpad1 => EnigoKey::Numpad1,
            #[cfg(target_os = "windows")]
            Key::Numpad2 => EnigoKey::Numpad2,
            #[cfg(target_os = "windows")]
            Key::Numpad3 => EnigoKey::Numpad3,
            #[cfg(target_os = "windows")]
            Key::Numpad4 => EnigoKey::Numpad4,
            #[cfg(target_os = "windows")]
            Key::Numpad5 => EnigoKey::Numpad5,
            #[cfg(target_os = "windows")]
            Key::Numpad6 => EnigoKey::Numpad6,
            #[cfg(target_os = "windows")]
            Key::Numpad7 => EnigoKey::Numpad7,
            #[cfg(target_os = "windows")]
            Key::Numpad8 => EnigoKey::Numpad8,
            #[cfg(target_os = "windows")]
            Key::Numpad9 => EnigoKey::Numpad9,
            #[cfg(target_os = "windows")]
            Key::OEM1 => EnigoKey::OEM1,
            #[cfg(target_os = "windows")]
            Key::OEM102 => EnigoKey::OEM102,
            #[cfg(target_os = "windows")]
            Key::OEM2 => EnigoKey::OEM2,
            #[cfg(target_os = "windows")]
            Key::OEM3 => EnigoKey::OEM3,
            #[cfg(target_os = "windows")]
            Key::OEM4 => EnigoKey::OEM4,
            #[cfg(target_os = "windows")]
            Key::OEM5 => EnigoKey::OEM5,
            #[cfg(target_os = "windows")]
            Key::OEM6 => EnigoKey::OEM6,
            #[cfg(target_os = "windows")]
            Key::OEM7 => EnigoKey::OEM7,
            #[cfg(target_os = "windows")]
            Key::OEM8 => EnigoKey::OEM8,
            #[cfg(target_os = "windows")]
            Key::OEMAttn => EnigoKey::OEMAttn,
            #[cfg(target_os = "windows")]
            Key::OEMAuto => EnigoKey::OEMAuto,
            #[cfg(target_os = "windows")]
            Key::OEMAx => EnigoKey::OEMAx,
            #[cfg(target_os = "windows")]
            Key::OEMBacktab => EnigoKey::OEMBacktab,
            #[cfg(target_os = "windows")]
            Key::OEMClear => EnigoKey::OEMClear,
            #[cfg(target_os = "windows")]
            Key::OEMComma => EnigoKey::OEMComma,
            #[cfg(target_os = "windows")]
            Key::OEMCopy => EnigoKey::OEMCopy,
            #[cfg(target_os = "windows")]
            Key::OEMCusel => EnigoKey::OEMCusel,
            #[cfg(target_os = "windows")]
            Key::OEMEnlw => EnigoKey::OEMEnlw,
            #[cfg(target_os = "windows")]
            Key::OEMFinish => EnigoKey::OEMFinish,
            #[cfg(target_os = "windows")]
            Key::OEMFJJisho => EnigoKey::OEMFJJisho,
            #[cfg(target_os = "windows")]
            Key::OEMFJLoya => EnigoKey::OEMFJLoya,
            #[cfg(target_os = "windows")]
            Key::OEMFJMasshou => EnigoKey::OEMFJMasshou,
            #[cfg(target_os = "windows")]
            Key::OEMFJRoya => EnigoKey::OEMFJRoya,
            #[cfg(target_os = "windows")]
            Key::OEMFJTouroku => EnigoKey::OEMFJTouroku,
            #[cfg(target_os = "windows")]
            Key::OEMJump => EnigoKey::OEMJump,
            #[cfg(target_os = "windows")]
            Key::OEMMinus => EnigoKey::OEMMinus,
            #[cfg(target_os = "windows")]
            Key::OEMNECEqual => EnigoKey::OEMNECEqual,
            #[cfg(target_os = "windows")]
            Key::OEMPA1 => EnigoKey::OEMPA1,
            #[cfg(target_os = "windows")]
            Key::OEMPA2 => EnigoKey::OEMPA2,
            #[cfg(target_os = "windows")]
            Key::OEMPA3 => EnigoKey::OEMPA3,
            #[cfg(target_os = "windows")]
            Key::OEMPeriod => EnigoKey::OEMPeriod,
            #[cfg(target_os = "windows")]
            Key::OEMPlus => EnigoKey::OEMPlus,
            #[cfg(target_os = "windows")]
            Key::OEMReset => EnigoKey::OEMReset,
            #[cfg(target_os = "windows")]
            Key::OEMWsctrl => EnigoKey::OEMWsctrl,
            /// option key on macOS (alt key on Linux and Windows)
            Key::Option => EnigoKey::Option,
            #[cfg(target_os = "windows")]
            Key::PA1 => EnigoKey::PA1,
            #[cfg(target_os = "windows")]
            Key::Packet => EnigoKey::Packet,
            /// page down key
            Key::PageDown => EnigoKey::PageDown,
            /// page up key
            Key::PageUp => EnigoKey::PageUp,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::Pause => EnigoKey::Pause,
            #[cfg(target_os = "windows")]
            Key::Play => EnigoKey::Play,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::Print => EnigoKey::Print,
            #[cfg(target_os = "windows")]
            Key::Processkey => EnigoKey::Processkey,
            #[cfg(target_os = "windows")]
            Key::RButton => EnigoKey::RButton,
            #[cfg(target_os = "macos")]
            Key::RCommand => EnigoKey::RCommand,
            Key::RControl => EnigoKey::RControl,
            #[cfg(target_os = "linux")]
            Key::Redo => EnigoKey::Redo,
            /// return key
            Key::Return => EnigoKey::Return,
            /// right arrow key
            Key::RightArrow => EnigoKey::RightArrow,
            #[cfg(target_os = "windows")]
            Key::RMenu => EnigoKey::RMenu,
            #[cfg(target_os = "macos")]
            Key::ROption => EnigoKey::ROption,
            Key::RShift => EnigoKey::RShift,
            #[cfg(target_os = "windows")]
            Key::RWin => EnigoKey::RWin,
            #[cfg(target_os = "windows")]
            Key::Scroll => EnigoKey::Scroll,
            #[cfg(target_os = "linux")]
            Key::ScrollLock => EnigoKey::ScrollLock,
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            Key::Select => EnigoKey::Select,
            #[cfg(target_os = "linux")]
            Key::ScriptSwitch => EnigoKey::ScriptSwitch,
            #[cfg(target_os = "windows")]
            Key::Separator => EnigoKey::Separator,
            /// shift key
            Key::Shift => EnigoKey::Shift,
            #[cfg(target_os = "linux")]
            /// Lock shift key
            Key::ShiftLock => EnigoKey::ShiftLock,
            #[cfg(target_os = "windows")]
            Key::Sleep => EnigoKey::Sleep,
            #[cfg(target_os = "windows")]
            Key::Snapshot => EnigoKey::Snapshot,
            /// space key
            Key::Space => EnigoKey::Space,
            #[cfg(target_os = "windows")]
            Key::Subtract => EnigoKey::Subtract,
            #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
            /// super key on linux (command key on Key::macOS => EnigoKey::macOS, windows key on Windows)
            Key::Super => EnigoKey::Super,
            #[cfg(target_os = "linux")]
            Key::SysReq => EnigoKey::SysReq,
            /// tab key (tabulator)
            Key::Tab => EnigoKey::Tab,
            #[cfg(target_os = "linux")]
            Key::Undo => EnigoKey::Undo,
            /// up arrow key
            Key::UpArrow => EnigoKey::UpArrow,
            Key::VolumeDown => EnigoKey::VolumeDown,
            Key::VolumeMute => EnigoKey::VolumeMute,
            Key::VolumeUp => EnigoKey::VolumeUp,
            #[cfg(target_os = "windows")]
            Key::XButton1 => EnigoKey::XButton1,
            #[cfg(target_os = "windows")]
            Key::XButton2 => EnigoKey::XButton2,
            #[cfg(target_os = "windows")]
            Key::Zoom => EnigoKey::Zoom,
            /// keyboard layout dependent key
            Key::Layout(char) => EnigoKey::Layout(char),
            /// raw keycode eg 0x38
            Key::Raw(u16) => EnigoKey::Raw(u16),
        }
    }
}

pub trait KeyboardAdapter {
    fn key_sequence(&mut self, sequence: &str);
    fn key_click(&mut self, key: Key);
    fn key_sequence_parse(&mut self, sequence: &str);
    // fn key_down(&mut self, key: EnigoKey);
    // fn key_up(&mut self, key: EnigoKey);
}

// TODO: study each fn works in Enigo
impl KeyboardAdapter for Enigo {
    fn key_sequence(&mut self, sequence: &str) {
        KeyboardControllable::key_sequence(self, sequence)
    }
    fn key_click(&mut self, key: Key) {
        KeyboardControllable::key_click(self, key.into())
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
