use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardButton {
    Sequence(String),
    SequenceDsl(String),
    Key(Key),
}

// TODO: this enum lives in enigo::keycodes, the problem is that jojo-common is used in esp32 and windows targets
// TODO: this means that enigo cannot be build for esp32 and we need to encapsulate all enigo dependencies in the driver module behind a feature flag
// TODO: we need to find a way to use a file in a dependency instead of all the project, with this we can erase this enum
#[derive(Debug, Copy, Deserialize, Serialize, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    #[cfg(feature = "windows")]
    Num0,
    #[cfg(feature = "windows")]
    Num1,
    #[cfg(feature = "windows")]
    Num2,
    #[cfg(feature = "windows")]
    Num3,
    #[cfg(feature = "windows")]
    Num4,
    #[cfg(feature = "windows")]
    Num5,
    #[cfg(feature = "windows")]
    Num6,
    #[cfg(feature = "windows")]
    Num7,
    #[cfg(feature = "windows")]
    Num8,
    #[cfg(feature = "windows")]
    Num9,
    #[cfg(feature = "windows")]
    A,
    #[cfg(feature = "windows")]
    B,
    #[cfg(feature = "windows")]
    C,
    #[cfg(feature = "windows")]
    D,
    #[cfg(feature = "windows")]
    E,
    #[cfg(feature = "windows")]
    F,
    #[cfg(feature = "windows")]
    G,
    #[cfg(feature = "windows")]
    H,
    #[cfg(feature = "windows")]
    I,
    #[cfg(feature = "windows")]
    J,
    #[cfg(feature = "windows")]
    K,
    #[cfg(feature = "windows")]
    L,
    #[cfg(feature = "windows")]
    M,
    #[cfg(feature = "windows")]
    N,
    #[cfg(feature = "windows")]
    O,
    #[cfg(feature = "windows")]
    P,
    #[cfg(feature = "windows")]
    Q,
    #[cfg(feature = "windows")]
    R,
    #[cfg(feature = "windows")]
    S,
    #[cfg(feature = "windows")]
    T,
    #[cfg(feature = "windows")]
    U,
    #[cfg(feature = "windows")]
    V,
    #[cfg(feature = "windows")]
    W,
    #[cfg(feature = "windows")]
    X,
    #[cfg(feature = "windows")]
    Y,
    #[cfg(feature = "windows")]
    Z,
    #[cfg(feature = "windows")]
    AbntC1,
    #[cfg(feature = "windows")]
    AbntC2,
    #[cfg(feature = "windows")]
    Accept,
    #[cfg(feature = "windows")]
    Add,
    /// alt key on Linux and Windows (option key on macOS)
    Alt,
    #[cfg(feature = "windows")]
    Apps,
    #[cfg(feature = "windows")]
    Attn,
    /// backspace key
    Backspace,
    #[cfg(feature = "linux")]
    Begin,
    #[cfg(feature = "linux")]
    Break,
    #[cfg(feature = "windows")]
    BrowserBack,
    #[cfg(feature = "windows")]
    BrowserFavorites,
    #[cfg(feature = "windows")]
    BrowserForward,
    #[cfg(feature = "windows")]
    BrowserHome,
    #[cfg(feature = "windows")]
    BrowserRefresh,
    #[cfg(feature = "windows")]
    BrowserSearch,
    #[cfg(feature = "windows")]
    BrowserStop,
    #[cfg(any(feature = "windows", feature = "linux"))]
    Cancel,
    /// caps lock key
    CapsLock,
    #[cfg(any(feature = "windows", feature = "linux"))]
    Clear,
    #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
    /// command key on macOS (super key on Linux, windows key on Windows)
    Command,
    /// control key
    Control,
    #[cfg(feature = "windows")]
    Convert,
    #[cfg(feature = "windows")]
    Crsel,
    #[cfg(feature = "windows")]
    DBEAlphanumeric,
    #[cfg(feature = "windows")]
    DBECodeinput,
    #[cfg(feature = "windows")]
    DBEDetermineString,
    #[cfg(feature = "windows")]
    DBEEnterDLGConversionMode,
    #[cfg(feature = "windows")]
    DBEEnterIMEConfigMode,
    #[cfg(feature = "windows")]
    DBEEnterWordRegisterMode,
    #[cfg(feature = "windows")]
    DBEFlushString,
    #[cfg(feature = "windows")]
    DBEHiragana,
    #[cfg(feature = "windows")]
    DBEKatakana,
    #[cfg(feature = "windows")]
    DBENoCodepoint,
    #[cfg(feature = "windows")]
    DBENoRoman,
    #[cfg(feature = "windows")]
    DBERoman,
    #[cfg(feature = "windows")]
    DBESBCSChar,
    #[cfg(feature = "windows")]
    DBESChar,
    #[cfg(feature = "windows")]
    Decimal,
    /// delete key
    Delete,
    #[cfg(feature = "windows")]
    Divide,
    /// down arrow key
    DownArrow,
    /// end key
    End,
    #[cfg(feature = "windows")]
    Ereof,
    /// escape key (esc)
    Escape,
    #[cfg(any(feature = "windows", feature = "linux"))]
    Execute,
    #[cfg(feature = "windows")]
    Exsel,
    /// F1 key
    F1,
    /// F2 key
    F2,
    /// F3 key
    F3,
    /// F4 key
    F4,
    /// F5 key
    F5,
    /// F6 key
    F6,
    /// F7 key
    F7,
    /// F8 key
    F8,
    /// F9 key
    F9,
    /// F10 key
    F10,
    /// F11 key
    F11,
    /// F12 key
    F12,
    /// F13 key
    F13,
    /// F14 key
    F14,
    /// F15 key
    F15,
    /// F16 key
    F16,
    /// F17 key
    F17,
    /// F18 key
    F18,
    /// F19 key
    F19,
    /// F20 key
    F20,
    #[cfg(any(feature = "windows", feature = "linux"))]
    /// F21 key
    F21,
    #[cfg(any(feature = "windows", feature = "linux"))]
    /// F22 key
    F22,
    #[cfg(any(feature = "windows", feature = "linux"))]
    /// F23 key
    F23,
    #[cfg(any(feature = "windows", feature = "linux"))]
    /// F24 key
    F24,
    #[cfg(feature = "linux")]
    F25,
    #[cfg(feature = "linux")]
    F26,
    #[cfg(feature = "linux")]
    F27,
    #[cfg(feature = "linux")]
    F28,
    #[cfg(feature = "linux")]
    F29,
    #[cfg(feature = "linux")]
    F30,
    #[cfg(feature = "linux")]
    F31,
    #[cfg(feature = "linux")]
    F32,
    #[cfg(feature = "linux")]
    F33,
    #[cfg(feature = "linux")]
    F34,
    #[cfg(feature = "linux")]
    F35,
    #[cfg(feature = "macos")]
    Function,
    #[cfg(feature = "windows")]
    Final,
    #[cfg(feature = "linux")]
    Find,
    #[cfg(feature = "windows")]
    GamepadA,
    #[cfg(feature = "windows")]
    GamepadB,
    #[cfg(feature = "windows")]
    GamepadDPadDown,
    #[cfg(feature = "windows")]
    GamepadDPadLeft,
    #[cfg(feature = "windows")]
    GamepadDPadRight,
    #[cfg(feature = "windows")]
    GamepadDPadUp,
    #[cfg(feature = "windows")]
    GamepadLeftShoulder,
    #[cfg(feature = "windows")]
    GamepadLeftThumbstickButton,
    #[cfg(feature = "windows")]
    GamepadLeftThumbstickDown,
    #[cfg(feature = "windows")]
    GamepadLeftThumbstickLeft,
    #[cfg(feature = "windows")]
    GamepadLeftThumbstickRight,
    #[cfg(feature = "windows")]
    GamepadLeftThumbstickUp,
    #[cfg(feature = "windows")]
    GamepadLeftTrigger,
    #[cfg(feature = "windows")]
    GamepadMenu,
    #[cfg(feature = "windows")]
    GamepadRightShoulder,
    #[cfg(feature = "windows")]
    GamepadRightThumbstickButton,
    #[cfg(feature = "windows")]
    GamepadRightThumbstickDown,
    #[cfg(feature = "windows")]
    GamepadRightThumbstickLeft,
    #[cfg(feature = "windows")]
    GamepadRightThumbstickRight,
    #[cfg(feature = "windows")]
    GamepadRightThumbstickUp,
    #[cfg(feature = "windows")]
    GamepadRightTrigger,
    #[cfg(feature = "windows")]
    GamepadView,
    #[cfg(feature = "windows")]
    GamepadX,
    #[cfg(feature = "windows")]
    GamepadY,
    #[cfg(feature = "windows")]
    Hangeul,
    #[cfg(any(feature = "windows", feature = "linux"))]
    Hangul,
    #[cfg(any(feature = "windows", feature = "linux"))]
    Hanja,
    Help,
    /// home key
    Home,
    #[cfg(feature = "windows")]
    Ico00,
    #[cfg(feature = "windows")]
    IcoClear,
    #[cfg(feature = "windows")]
    IcoHelp,
    #[cfg(feature = "windows")]
    IMEOff,
    #[cfg(feature = "windows")]
    IMEOn,
    #[cfg(any(feature = "windows", feature = "linux"))]
    Insert,
    #[cfg(feature = "windows")]
    Junja,
    #[cfg(feature = "windows")]
    Kana,
    #[cfg(any(feature = "windows", feature = "linux"))]
    Kanji,
    #[cfg(feature = "windows")]
    LaunchApp1,
    #[cfg(feature = "windows")]
    LaunchApp2,
    #[cfg(feature = "windows")]
    LaunchMail,
    #[cfg(feature = "windows")]
    LaunchMediaSelect,
    #[cfg(feature = "macos")]
    /// Opens launchpad
    Launchpad,
    #[cfg(feature = "windows")]
    LButton,
    LControl,
    /// left arrow key
    LeftArrow,
    #[cfg(feature = "linux")]
    Linefeed,
    #[cfg(any(feature = "windows", feature = "linux"))]
    LMenu,
    LShift,
    #[cfg(feature = "windows")]
    LWin,
    #[cfg(feature = "windows")]
    MButton,
    #[cfg(any(feature = "windows", feature = "linux"))]
    MediaNextTrack,
    #[cfg(any(feature = "windows", feature = "linux"))]
    MediaPlayPause,
    #[cfg(any(feature = "windows", feature = "linux"))]
    MediaPrevTrack,
    #[cfg(any(feature = "windows", feature = "linux"))]
    MediaStop,
    /// meta key (also known as "windows", "super", and "command")
    Meta,
    #[cfg(feature = "macos")]
    /// Opens mission control
    MissionControl,
    #[cfg(any(feature = "windows", feature = "linux"))]
    ModeChange,
    #[cfg(feature = "windows")]
    Multiply,
    #[cfg(feature = "windows")]
    NavigationAccept,
    #[cfg(feature = "windows")]
    NavigationCancel,
    #[cfg(feature = "windows")]
    NavigationDown,
    #[cfg(feature = "windows")]
    NavigationLeft,
    #[cfg(feature = "windows")]
    NavigationMenu,
    #[cfg(feature = "windows")]
    NavigationRight,
    #[cfg(feature = "windows")]
    NavigationUp,
    #[cfg(feature = "windows")]
    NavigationView,
    #[cfg(feature = "windows")]
    NoName,
    #[cfg(feature = "windows")]
    NonConvert,
    #[cfg(feature = "windows")]
    None,
    #[cfg(any(feature = "windows", feature = "linux"))]
    Numlock,
    #[cfg(feature = "windows")]
    Numpad0,
    #[cfg(feature = "windows")]
    Numpad1,
    #[cfg(feature = "windows")]
    Numpad2,
    #[cfg(feature = "windows")]
    Numpad3,
    #[cfg(feature = "windows")]
    Numpad4,
    #[cfg(feature = "windows")]
    Numpad5,
    #[cfg(feature = "windows")]
    Numpad6,
    #[cfg(feature = "windows")]
    Numpad7,
    #[cfg(feature = "windows")]
    Numpad8,
    #[cfg(feature = "windows")]
    Numpad9,
    #[cfg(feature = "windows")]
    OEM1,
    #[cfg(feature = "windows")]
    OEM102,
    #[cfg(feature = "windows")]
    OEM2,
    #[cfg(feature = "windows")]
    OEM3,
    #[cfg(feature = "windows")]
    OEM4,
    #[cfg(feature = "windows")]
    OEM5,
    #[cfg(feature = "windows")]
    OEM6,
    #[cfg(feature = "windows")]
    OEM7,
    #[cfg(feature = "windows")]
    OEM8,
    #[cfg(feature = "windows")]
    OEMAttn,
    #[cfg(feature = "windows")]
    OEMAuto,
    #[cfg(feature = "windows")]
    OEMAx,
    #[cfg(feature = "windows")]
    OEMBacktab,
    #[cfg(feature = "windows")]
    OEMClear,
    #[cfg(feature = "windows")]
    OEMComma,
    #[cfg(feature = "windows")]
    OEMCopy,
    #[cfg(feature = "windows")]
    OEMCusel,
    #[cfg(feature = "windows")]
    OEMEnlw,
    #[cfg(feature = "windows")]
    OEMFinish,
    #[cfg(feature = "windows")]
    OEMFJJisho,
    #[cfg(feature = "windows")]
    OEMFJLoya,
    #[cfg(feature = "windows")]
    OEMFJMasshou,
    #[cfg(feature = "windows")]
    OEMFJRoya,
    #[cfg(feature = "windows")]
    OEMFJTouroku,
    #[cfg(feature = "windows")]
    OEMJump,
    #[cfg(feature = "windows")]
    OEMMinus,
    #[cfg(feature = "windows")]
    OEMNECEqual,
    #[cfg(feature = "windows")]
    OEMPA1,
    #[cfg(feature = "windows")]
    OEMPA2,
    #[cfg(feature = "windows")]
    OEMPA3,
    #[cfg(feature = "windows")]
    OEMPeriod,
    #[cfg(feature = "windows")]
    OEMPlus,
    #[cfg(feature = "windows")]
    OEMReset,
    #[cfg(feature = "windows")]
    OEMWsctrl,
    /// option key on macOS (alt key on Linux and Windows)
    Option,
    #[cfg(feature = "windows")]
    PA1,
    #[cfg(feature = "windows")]
    Packet,
    /// page down key
    PageDown,
    /// page up key
    PageUp,
    #[cfg(any(feature = "windows", feature = "linux"))]
    Pause,
    #[cfg(feature = "windows")]
    Play,
    #[cfg(any(feature = "windows", feature = "linux"))]
    Print,
    #[cfg(feature = "windows")]
    Processkey,
    #[cfg(feature = "windows")]
    RButton,
    #[cfg(feature = "macos")]
    RCommand,
    RControl,
    #[cfg(feature = "linux")]
    Redo,
    /// return key
    Return,
    /// right arrow key
    RightArrow,
    #[cfg(feature = "windows")]
    RMenu,
    #[cfg(feature = "macos")]
    ROption,
    RShift,
    #[cfg(feature = "windows")]
    RWin,
    #[cfg(feature = "windows")]
    Scroll,
    #[cfg(feature = "linux")]
    ScrollLock,
    #[cfg(any(feature = "windows", feature = "linux"))]
    Select,
    #[cfg(feature = "linux")]
    ScriptSwitch,
    #[cfg(feature = "windows")]
    Separator,
    /// shift key
    Shift,
    #[cfg(feature = "linux")]
    /// Lock shift key
    ShiftLock,
    #[cfg(feature = "windows")]
    Sleep,
    #[cfg(feature = "windows")]
    Snapshot,
    /// space key
    Space,
    #[cfg(feature = "windows")]
    Subtract,
    #[deprecated(since = "0.0.12", note = "now renamed to Meta")]
    /// super key on linux (command key on macOS, windows key on Windows)
    Super,
    #[cfg(feature = "linux")]
    SysReq,
    /// tab key (tabulator)
    Tab,
    #[cfg(feature = "linux")]
    Undo,
    /// up arrow key
    UpArrow,
    VolumeDown,
    VolumeMute,
    VolumeUp,
    #[cfg(feature = "windows")]
    XButton1,
    #[cfg(feature = "windows")]
    XButton2,
    #[cfg(feature = "windows")]
    Zoom,
    /// keyboard layout dependent key
    Layout(char),
    /// raw keycode eg 0x38
    Raw(u16),
}
