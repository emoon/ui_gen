
// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::os::raw::c_void;

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct RUBase {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct RUArray {
    pub priv_data: *const c_void,
    pub elements: *const c_void,
    pub count: i32,
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct RULayoutType {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct RUPaintDevice {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct RUWidgetType {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum RUMetaKeys {
    Ctrl,
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub enum RUKeys {
    KeyEscape,
    KeyTab,
    KeyBacktab,
    KeyBackspace,
    KeyReturn,
    KeyEnter,
    KeyInsert,
    KeyDelete,
    KeyPause,
    KeyPrint,
    KeySysReq,
    KeyClear,
    KeyHome,
    KeyEnd,
    KeyLeft,
    KeyUp,
    KeyRight,
    KeyDown,
    KeyPageUp,
    KeyPageDown,
    KeyShift,
    KeyControl,
    KeyMeta,
    KeyAlt,
    KeyCapsLock,
    KeyNumLock,
    KeyScrollLock,
    KeyF1,
    KeyF2,
    KeyF3,
    KeyF4,
    KeyF5,
    KeyF6,
    KeyF7,
    KeyF8,
    KeyF9,
    KeyF10,
    KeyF11,
    KeyF12,
    KeyF13,
    KeyF14,
    KeyF15,
    KeyF16,
    KeyF17,
    KeyF18,
    KeyF19,
    KeyF20,
    KeyF21,
    KeyF22,
    KeyF23,
    KeyF24,
    KeyF25,
    KeyF26,
    KeyF27,
    KeyF28,
    KeyF29,
    KeyF30,
    KeyF31,
    KeyF32,
    KeyF33,
    KeyF34,
    KeyF35,
    KeySuperL,
    KeySuperR,
    KeyMenu,
    KeyHyperL,
    KeyHyperR,
    KeyHelp,
    KeyDirectionL,
    KeyDirectionR,
    KeySpace,
    KeyAny,
    KeyExclam,
    KeyQuoteDbl,
    KeyNumberSign,
    KeyDollar,
    KeyPercent,
    KeyAmpersand,
    KeyApostrophe,
    KeyParenLeft,
    KeyParenRight,
    KeyAsterisk,
    KeyPlus,
    KeyComma,
    KeyMinus,
    KeyPeriod,
    KeySlash,
    Key0,
    Key1,
    Key2,
    Key3,
    Key4,
    Key5,
    Key6,
    Key7,
    Key8,
    Key9,
    KeyColon,
    KeySemicolon,
    KeyLess,
    KeyEqual,
    KeyGreater,
    KeyQuestion,
    KeyAt,
    KeyA,
    KeyB,
    KeyC,
    KeyD,
    KeyE,
    KeyF,
    KeyG,
    KeyH,
    KeyI,
    KeyJ,
    KeyK,
    KeyL,
    KeyM,
    KeyN,
    KeyO,
    KeyP,
    KeyQ,
    KeyR,
    KeyS,
    KeyT,
    KeyU,
    KeyV,
    KeyW,
    KeyX,
    KeyY,
    KeyZ,
    KeyBracketLeft,
    KeyBackslash,
    KeyBracketRight,
    KeyAsciiCircum,
    KeyUnderscore,
    KeyQuoteLeft,
    KeyBraceLeft,
    KeyBar,
    KeyBraceRight,
    KeyAsciiTilde,
    KeyBack,
    KeyForward,
    KeyStop,
    KeyRefresh,
    KeyVolumeDown,
    KeyVolumeMute,
    KeyVolumeUp,
    KeyBassBoost,
    KeyBassUp,
    KeyBassDown,
    KeyTrebleUp,
    KeyTrebleDown,
    KeyMediaPlay,
    KeyMediaStop,
    KeyMediaPrevious,
    KeyMediaNext,
    KeyMediaRecord,
    KeyMediaPause,
    KeyMediaTogglePlayPause,
    KeyHomePage,
    KeyFavorites,
    KeySearch,
    KeyStandby,
    KeyOpenUrl,
    KeyLaunchMail,
    KeyLaunchMedia,
    KeyLaunch0,
    KeyLaunch1,
    KeyLaunch2,
    KeyLaunch3,
    KeyLaunch4,
    KeyLaunch5,
    KeyLaunch6,
    KeyLaunch7,
    KeyLaunch8,
    KeyLaunch9,
    KeyLaunchA,
    KeyLaunchB,
    KeyLaunchC,
    KeyLaunchD,
    KeyLaunchE,
    KeyLaunchF,
    KeyMonBrightnessUp,
    KeyMonBrightnessDown,
    KeyKeyboardLightOnOff,
    KeyKeyboardBrightnessUp,
    KeyKeyboardBrightnessDown,
    KeyPowerOff,
    KeyWakeUp,
    KeyEject,
    KeyScreenSaver,
    KeyWww,
    KeyMemo,
    KeyLightBulb,
    KeyShop,
    KeyHistory,
    KeyAddFavorite,
    KeyHotLinks,
    KeyBrightnessAdjust,
    KeyFinance,
    KeyCommunity,
    KeyAudioRewind,
    KeyBackForward,
    KeyApplicationLeft,
    KeyApplicationRight,
    KeyBook,
    KeyCd,
    KeyCalculator,
    KeyToDoList,
    KeyClearGrab,
    KeyClose,
    KeyCopy,
    KeyCut,
    KeyDisplay,
    KeyDos,
    KeyDocuments,
    KeyExcel,
    KeyExplorer,
    KeyGame,
    KeyGo,
    KeyITouch,
    KeyLogOff,
    KeyMarket,
    KeyMeeting,
    KeyMenuKb,
    KeyMenuPb,
    KeyMySites,
    KeyNews,
    KeyOfficeHome,
    KeyOption,
    KeyPaste,
    KeyPhone,
    KeyCalendar,
    KeyReply,
    KeyReload,
    KeyRotateWindows,
    KeyRotationPb,
    KeyRotationKb,
    KeySave,
    KeySend,
    KeySpell,
    KeySplitScreen,
    KeySupport,
    KeyTaskPane,
    KeyTerminal,
    KeyTools,
    KeyTravel,
    KeyVideo,
    KeyWord,
    KeyXfer,
    KeyZoomIn,
    KeyZoomOut,
    KeyAway,
    KeyMessenger,
    KeyWebCam,
    KeyMailForward,
    KeyPictures,
    KeyMusic,
    KeyBattery,
    KeyBluetooth,
    KeyWlan,
    KeyUwb,
    KeyAudioForward,
    KeyAudioRepeat,
    KeyAudioRandomPlay,
    KeySubtitle,
    KeyAudioCycleTrack,
    KeyTime,
    KeyHibernate,
    KeyView,
    KeyTopMenu,
    KeyPowerDown,
    KeySuspend,
    KeyContrastAdjust,
    KeyLaunchG,
    KeyLaunchH,
    KeyTouchpadToggle,
    KeyTouchpadOn,
    KeyTouchpadOff,
    KeyMicMute,
    KeyRed,
    KeyGreen,
    KeyYellow,
    KeyBlue,
    KeyChannelUp,
    KeyChannelDown,
    KeyGuide,
    KeyInfo,
    KeySettings,
    KeyMicVolumeUp,
    KeyMicVolumeDown,
    KeyNew,
    KeyOpen,
    KeyFind,
    KeyUndo,
    KeyRedo,
    KeyMediaLast,
    KeySelect,
    KeyYes,
    KeyNo,
    KeyCancel,
    KeyPrinter,
    KeyExecute,
    KeySleep,
    KeyPlay,
    KeyZoom,
    KeyExit,
    KeyContext1,
    KeyContext2,
    KeyContext3,
    KeyContext4,
    KeyCall,
    KeyHangup,
    KeyFlip,
    KeyToggleCallHangup,
    KeyVoiceDial,
    KeyLastNumberRedial,
    KeyCamera,
    KeyCameraFocus,
}

}

#[repr(C)]
pub struct RUApplicationFuncs {
    pub set_style: extern "C" fn(self_c: *const RUBase, style: *const ::std::os::raw::c_char),
    pub set_style_sheet: extern "C" fn(self_c: *const RUBase, filename: *const ::std::os::raw::c_char) -> i32,
    pub exec: extern "C" fn(self_c: *const RUBase),
    pub set_about_to_quit_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern "C" fn(self_c: *const c_void),
    pub get_files: extern "C" fn(self_c: *const RUBase) -> RUArray,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUApplication {
    pub application_funcs: *const RUApplicationFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUActionFuncs {
    pub is_enabled: extern "C" fn(self_c: *const RUBase) -> bool,
    pub set_text: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub text: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub set_shortcut: extern "C" fn(self_c: *const RUBase, key:  RUKeys),
    pub set_shortcut_mod: extern "C" fn(self_c: *const RUBase, key:  RUKeys, modifier:  RUMetaKeys),
    pub set_triggered_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern "C" fn(self_c: *const c_void),
    pub set_int_data: extern "C" fn(self_c: *const RUBase, data: i32),
    pub get_int_data: extern "C" fn(self_c: *const RUBase) -> i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUAction {
    pub action_funcs: *const RUActionFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUWidgetFuncs {
    pub show: extern "C" fn(self_c: *const RUBase),
    pub set_persist_data: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub persist_data: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub set_fixed_height: extern "C" fn(self_c: *const RUBase, width: i32),
    pub set_fixed_width: extern "C" fn(self_c: *const RUBase, width: i32),
    pub resize: extern "C" fn(self_c: *const RUBase, width: i32, height: i32),
    pub set_parent: extern "C" fn(self_c: *const RUBase, widget: *const RUBase),
    pub set_layout: extern "C" fn(self_c: *const RUBase, layout: *const RUBase),
    pub update: extern "C" fn(self_c: *const RUBase),
    pub set_paint_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern "C" fn(widget: *const RUBase, self_c: *const c_void, event: *const RUBase)),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUWidget {
    pub widget_funcs: *const RUWidgetFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUListWidgetItemFuncs {
    pub set_text: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub text: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
    pub set_string_data: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub get_string_data: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUListWidgetItem {
    pub list_widget_item_funcs: *const RUListWidgetItemFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUListWidgetFuncs {
    pub clear: extern "C" fn(self_c: *const RUBase),
    pub add_item: extern "C" fn(self_c: *const RUBase, item: *const RUBase),
    pub add_text_item: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub current_item: extern "C" fn(self_c: *const RUBase) ->  RUListWidgetItem,
    pub current_row: extern "C" fn(self_c: *const RUBase) -> i32,
    pub selected_items: extern "C" fn(self_c: *const RUBase) -> RUArray,
    pub item: extern "C" fn(self_c: *const RUBase, index: i32) ->  RUListWidgetItem,
    pub set_current_row: extern "C" fn(self_c: *const RUBase, index: i32),
    pub count: extern "C" fn(self_c: *const RUBase) -> i32,
    pub set_drag_enabled: extern "C" fn(self_c: *const RUBase, state: bool),
    pub set_drop_indicator_shown: extern "C" fn(self_c: *const RUBase, state: bool),
    pub set_accept_drops: extern "C" fn(self_c: *const RUBase, state: bool),
    pub set_drag_enter_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern "C" fn(widget: *const RUBase, self_c: *const c_void, event: *const RUBase)),
    pub set_drop_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern "C" fn(widget: *const RUBase, self_c: *const c_void, event: *const RUBase)),
    pub add_widget_item: extern "C" fn(self_c: *const RUBase, item: *const RUBase),
    pub set_current_row_changed_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern "C" fn(self_c: *const c_void, row: i32),
    pub set_item_clicked_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern "C" fn(self_c: *const c_void, item: *const RUBase),
    pub set_item_double_clicked_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern "C" fn(self_c: *const c_void, item: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUListWidget {
    pub widget_funcs: *const RUWidgetFuncs,
    pub list_widget_funcs: *const RUListWidgetFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUUrlFuncs {
    pub is_local_file: extern "C" fn(self_c: *const RUBase) -> bool,
    pub to_local_file: extern "C" fn(self_c: *const RUBase) -> *const ::std::os::raw::c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUUrl {
    pub url_funcs: *const RUUrlFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUMimeDataFuncs {
    pub has_color: extern "C" fn(self_c: *const RUBase) -> bool,
    pub has_image: extern "C" fn(self_c: *const RUBase) -> bool,
    pub has_text: extern "C" fn(self_c: *const RUBase) -> bool,
    pub has_urls: extern "C" fn(self_c: *const RUBase) -> bool,
    pub urls: extern "C" fn(self_c: *const RUBase) -> RUArray,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUMimeData {
    pub mime_data_funcs: *const RUMimeDataFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUTimerFuncs {
    pub set_timeout_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern "C" fn(self_c: *const c_void),
    pub start: extern "C" fn(self_c: *const RUBase, time: i32),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUTimer {
    pub timer_funcs: *const RUTimerFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUIconFuncs {
    pub add_file: extern "C" fn(self_c: *const RUBase, filename: *const ::std::os::raw::c_char),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUIcon {
    pub icon_funcs: *const RUIconFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUMenuFuncs {
    pub add_action_text: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub set_triggered_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern "C" fn(self_c: *const c_void, action: *const RUBase),
    pub add_action: extern "C" fn(self_c: *const RUBase, action: *const RUBase),
    pub set_title: extern "C" fn(self_c: *const RUBase, title: *const ::std::os::raw::c_char),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUMenu {
    pub widget_funcs: *const RUWidgetFuncs,
    pub menu_funcs: *const RUMenuFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUMenuBarFuncs {
    pub add_menu: extern "C" fn(self_c: *const RUBase, menu: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUMenuBar {
    pub widget_funcs: *const RUWidgetFuncs,
    pub menu_bar_funcs: *const RUMenuBarFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RULabelFuncs {
    pub set_text: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RULabel {
    pub widget_funcs: *const RUWidgetFuncs,
    pub label_funcs: *const RULabelFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RULineEditFuncs {
    pub set_text: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub set_read_only: extern "C" fn(self_c: *const RUBase, value: bool),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RULineEdit {
    pub widget_funcs: *const RUWidgetFuncs,
    pub line_edit_funcs: *const RULineEditFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUPlainTextEditFuncs {
    pub clear: extern "C" fn(self_c: *const RUBase),
    pub set_plain_text: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub append_plain_text: extern "C" fn(self_c: *const RUBase, text: *const ::std::os::raw::c_char),
    pub set_read_only: extern "C" fn(self_c: *const RUBase, value: bool),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUPlainTextEdit {
    pub widget_funcs: *const RUWidgetFuncs,
    pub plain_text_edit_funcs: *const RUPlainTextEditFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUSliderFuncs {
    pub set_value_changed_event: extern "C" fn(object: *const RUBase, user_data: *const c_void,
                                            callback: extern "C" fn(self_c: *const c_void, value: i32),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUSlider {
    pub widget_funcs: *const RUWidgetFuncs,
    pub slider_funcs: *const RUSliderFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUMainWindowFuncs {
    pub is_animated: extern "C" fn(self_c: *const RUBase) -> bool,
    pub menu_bar: extern "C" fn(self_c: *const RUBase) ->  RUMenuBar,
    pub set_central_widget: extern "C" fn(self_c: *const RUBase, widget: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUMainWindow {
    pub widget_funcs: *const RUWidgetFuncs,
    pub main_window_funcs: *const RUMainWindowFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RULayoutFuncs {
    pub add_widget: extern "C" fn(self_c: *const RUBase, widget: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RULayout {
    pub layout_funcs: *const RULayoutFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUVBoxLayoutFuncs {
    pub add_layout: extern "C" fn(self_c: *const RUBase, layout: *const RUBase),
    pub update: extern "C" fn(self_c: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUVBoxLayout {
    pub layout_funcs: *const RULayoutFuncs,
    pub v_box_layout_funcs: *const RUVBoxLayoutFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RUHBoxLayoutFuncs {
    pub add_layout: extern "C" fn(self_c: *const RUBase, layout: *const RUBase),
    pub update: extern "C" fn(self_c: *const RUBase),
}

#[repr(C)]
#[derive(Copy, Clone)]
struct RUHBoxLayout {
    pub layout_funcs: *const RULayoutFuncs,
    pub h_box_layout_funcs: *const RUHBoxLayoutFuncs,
    pub privd: *const RUBase,
}

#[repr(C)]
pub struct RuteFFI {
    pub create_application: extern "C" fn(priv_data: *const RUBase) -> RUApplication,
    pub create_action: extern "C" fn(priv_data: *const RUBase) -> RUAction,
    pub create_widget: extern "C" fn(priv_data: *const RUBase) -> RUWidget,
    pub create_list_widget_item: extern "C" fn(priv_data: *const RUBase) -> RUListWidgetItem,
    pub create_list_widget: extern "C" fn(priv_data: *const RUBase) -> RUListWidget,
    pub create_timer: extern "C" fn(priv_data: *const RUBase) -> RUTimer,
    pub create_icon: extern "C" fn(priv_data: *const RUBase) -> RUIcon,
    pub create_menu: extern "C" fn(priv_data: *const RUBase) -> RUMenu,
    pub create_menu_bar: extern "C" fn(priv_data: *const RUBase) -> RUMenuBar,
    pub create_label: extern "C" fn(priv_data: *const RUBase) -> RULabel,
    pub create_line_edit: extern "C" fn(priv_data: *const RUBase) -> RULineEdit,
    pub create_plain_text_edit: extern "C" fn(priv_data: *const RUBase) -> RUPlainTextEdit,
    pub create_slider: extern "C" fn(priv_data: *const RUBase) -> RUSlider,
    pub create_main_window: extern "C" fn(priv_data: *const RUBase) -> RUMainWindow,
    pub create_v_box_layout: extern "C" fn(priv_data: *const RUBase) -> RUVBoxLayout,
    pub create_h_box_layout: extern "C" fn(priv_data: *const RUBase) -> RUHBoxLayout,
    pub privd: *const RUBase,
}

