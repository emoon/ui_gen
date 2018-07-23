///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////


struct KeyVal { int val, key; };
static std::map<int, int> s_meta_keys_lookup;
static std::map<int, int> s_keys_lookup;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

static void create_enum_mappings() {
    static KeyVal meta_keys_vals[] = {
        { (int)Qt::CTRL, 0 },
    };

    for (int i = 0; i < 1; ++i) {
        s_meta_keys_lookup[meta_keys_vals[i].key] = meta_keys_vals[i].val;
    };
    static KeyVal keys_vals[] = {
        { (int)Qt::Key_Escape, 0 },
        { (int)Qt::Key_Tab, 1 },
        { (int)Qt::Key_Backtab, 2 },
        { (int)Qt::Key_Backspace, 3 },
        { (int)Qt::Key_Return, 4 },
        { (int)Qt::Key_Enter, 5 },
        { (int)Qt::Key_Insert, 6 },
        { (int)Qt::Key_Delete, 7 },
        { (int)Qt::Key_Pause, 8 },
        { (int)Qt::Key_Print, 9 },
        { (int)Qt::Key_SysReq, 10 },
        { (int)Qt::Key_Clear, 11 },
        { (int)Qt::Key_Home, 12 },
        { (int)Qt::Key_End, 13 },
        { (int)Qt::Key_Left, 14 },
        { (int)Qt::Key_Up, 15 },
        { (int)Qt::Key_Right, 16 },
        { (int)Qt::Key_Down, 17 },
        { (int)Qt::Key_PageUp, 18 },
        { (int)Qt::Key_PageDown, 19 },
        { (int)Qt::Key_Shift, 20 },
        { (int)Qt::Key_Control, 21 },
        { (int)Qt::Key_Meta, 22 },
        { (int)Qt::Key_Alt, 23 },
        { (int)Qt::Key_CapsLock, 24 },
        { (int)Qt::Key_NumLock, 25 },
        { (int)Qt::Key_ScrollLock, 26 },
        { (int)Qt::Key_F1, 27 },
        { (int)Qt::Key_F2, 28 },
        { (int)Qt::Key_F3, 29 },
        { (int)Qt::Key_F4, 30 },
        { (int)Qt::Key_F5, 31 },
        { (int)Qt::Key_F6, 32 },
        { (int)Qt::Key_F7, 33 },
        { (int)Qt::Key_F8, 34 },
        { (int)Qt::Key_F9, 35 },
        { (int)Qt::Key_F10, 36 },
        { (int)Qt::Key_F11, 37 },
        { (int)Qt::Key_F12, 38 },
        { (int)Qt::Key_F13, 39 },
        { (int)Qt::Key_F14, 40 },
        { (int)Qt::Key_F15, 41 },
        { (int)Qt::Key_F16, 42 },
        { (int)Qt::Key_F17, 43 },
        { (int)Qt::Key_F18, 44 },
        { (int)Qt::Key_F19, 45 },
        { (int)Qt::Key_F20, 46 },
        { (int)Qt::Key_F21, 47 },
        { (int)Qt::Key_F22, 48 },
        { (int)Qt::Key_F23, 49 },
        { (int)Qt::Key_F24, 50 },
        { (int)Qt::Key_F25, 51 },
        { (int)Qt::Key_F26, 52 },
        { (int)Qt::Key_F27, 53 },
        { (int)Qt::Key_F28, 54 },
        { (int)Qt::Key_F29, 55 },
        { (int)Qt::Key_F30, 56 },
        { (int)Qt::Key_F31, 57 },
        { (int)Qt::Key_F32, 58 },
        { (int)Qt::Key_F33, 59 },
        { (int)Qt::Key_F34, 60 },
        { (int)Qt::Key_F35, 61 },
        { (int)Qt::Key_Super_L, 62 },
        { (int)Qt::Key_Super_R, 63 },
        { (int)Qt::Key_Menu, 64 },
        { (int)Qt::Key_Hyper_L, 65 },
        { (int)Qt::Key_Hyper_R, 66 },
        { (int)Qt::Key_Help, 67 },
        { (int)Qt::Key_Direction_L, 68 },
        { (int)Qt::Key_Direction_R, 69 },
        { (int)Qt::Key_Space, 70 },
        { (int)Qt::Key_Any, 71 },
        { (int)Qt::Key_Exclam, 72 },
        { (int)Qt::Key_QuoteDbl, 73 },
        { (int)Qt::Key_NumberSign, 74 },
        { (int)Qt::Key_Dollar, 75 },
        { (int)Qt::Key_Percent, 76 },
        { (int)Qt::Key_Ampersand, 77 },
        { (int)Qt::Key_Apostrophe, 78 },
        { (int)Qt::Key_ParenLeft, 79 },
        { (int)Qt::Key_ParenRight, 80 },
        { (int)Qt::Key_Asterisk, 81 },
        { (int)Qt::Key_Plus, 82 },
        { (int)Qt::Key_Comma, 83 },
        { (int)Qt::Key_Minus, 84 },
        { (int)Qt::Key_Period, 85 },
        { (int)Qt::Key_Slash, 86 },
        { (int)Qt::Key_0, 87 },
        { (int)Qt::Key_1, 88 },
        { (int)Qt::Key_2, 89 },
        { (int)Qt::Key_3, 90 },
        { (int)Qt::Key_4, 91 },
        { (int)Qt::Key_5, 92 },
        { (int)Qt::Key_6, 93 },
        { (int)Qt::Key_7, 94 },
        { (int)Qt::Key_8, 95 },
        { (int)Qt::Key_9, 96 },
        { (int)Qt::Key_Colon, 97 },
        { (int)Qt::Key_Semicolon, 98 },
        { (int)Qt::Key_Less, 99 },
        { (int)Qt::Key_Equal, 100 },
        { (int)Qt::Key_Greater, 101 },
        { (int)Qt::Key_Question, 102 },
        { (int)Qt::Key_At, 103 },
        { (int)Qt::Key_A, 104 },
        { (int)Qt::Key_B, 105 },
        { (int)Qt::Key_C, 106 },
        { (int)Qt::Key_D, 107 },
        { (int)Qt::Key_E, 108 },
        { (int)Qt::Key_F, 109 },
        { (int)Qt::Key_G, 110 },
        { (int)Qt::Key_H, 111 },
        { (int)Qt::Key_I, 112 },
        { (int)Qt::Key_J, 113 },
        { (int)Qt::Key_K, 114 },
        { (int)Qt::Key_L, 115 },
        { (int)Qt::Key_M, 116 },
        { (int)Qt::Key_N, 117 },
        { (int)Qt::Key_O, 118 },
        { (int)Qt::Key_P, 119 },
        { (int)Qt::Key_Q, 120 },
        { (int)Qt::Key_R, 121 },
        { (int)Qt::Key_S, 122 },
        { (int)Qt::Key_T, 123 },
        { (int)Qt::Key_U, 124 },
        { (int)Qt::Key_V, 125 },
        { (int)Qt::Key_W, 126 },
        { (int)Qt::Key_X, 127 },
        { (int)Qt::Key_Y, 128 },
        { (int)Qt::Key_Z, 129 },
        { (int)Qt::Key_BracketLeft, 130 },
        { (int)Qt::Key_Backslash, 131 },
        { (int)Qt::Key_BracketRight, 132 },
        { (int)Qt::Key_AsciiCircum, 133 },
        { (int)Qt::Key_Underscore, 134 },
        { (int)Qt::Key_QuoteLeft, 135 },
        { (int)Qt::Key_BraceLeft, 136 },
        { (int)Qt::Key_Bar, 137 },
        { (int)Qt::Key_BraceRight, 138 },
        { (int)Qt::Key_AsciiTilde, 139 },
        { (int)Qt::Key_Back, 140 },
        { (int)Qt::Key_Forward, 141 },
        { (int)Qt::Key_Stop, 142 },
        { (int)Qt::Key_Refresh, 143 },
        { (int)Qt::Key_VolumeDown, 144 },
        { (int)Qt::Key_VolumeMute, 145 },
        { (int)Qt::Key_VolumeUp, 146 },
        { (int)Qt::Key_BassBoost, 147 },
        { (int)Qt::Key_BassUp, 148 },
        { (int)Qt::Key_BassDown, 149 },
        { (int)Qt::Key_TrebleUp, 150 },
        { (int)Qt::Key_TrebleDown, 151 },
        { (int)Qt::Key_MediaPlay, 152 },
        { (int)Qt::Key_MediaStop, 153 },
        { (int)Qt::Key_MediaPrevious, 154 },
        { (int)Qt::Key_MediaNext, 155 },
        { (int)Qt::Key_MediaRecord, 156 },
        { (int)Qt::Key_MediaPause, 157 },
        { (int)Qt::Key_MediaTogglePlayPause, 158 },
        { (int)Qt::Key_HomePage, 159 },
        { (int)Qt::Key_Favorites, 160 },
        { (int)Qt::Key_Search, 161 },
        { (int)Qt::Key_Standby, 162 },
        { (int)Qt::Key_OpenUrl, 163 },
        { (int)Qt::Key_LaunchMail, 164 },
        { (int)Qt::Key_LaunchMedia, 165 },
        { (int)Qt::Key_Launch0, 166 },
        { (int)Qt::Key_Launch1, 167 },
        { (int)Qt::Key_Launch2, 168 },
        { (int)Qt::Key_Launch3, 169 },
        { (int)Qt::Key_Launch4, 170 },
        { (int)Qt::Key_Launch5, 171 },
        { (int)Qt::Key_Launch6, 172 },
        { (int)Qt::Key_Launch7, 173 },
        { (int)Qt::Key_Launch8, 174 },
        { (int)Qt::Key_Launch9, 175 },
        { (int)Qt::Key_LaunchA, 176 },
        { (int)Qt::Key_LaunchB, 177 },
        { (int)Qt::Key_LaunchC, 178 },
        { (int)Qt::Key_LaunchD, 179 },
        { (int)Qt::Key_LaunchE, 180 },
        { (int)Qt::Key_LaunchF, 181 },
        { (int)Qt::Key_MonBrightnessUp, 182 },
        { (int)Qt::Key_MonBrightnessDown, 183 },
        { (int)Qt::Key_KeyboardLightOnOff, 184 },
        { (int)Qt::Key_KeyboardBrightnessUp, 185 },
        { (int)Qt::Key_KeyboardBrightnessDown, 186 },
        { (int)Qt::Key_PowerOff, 187 },
        { (int)Qt::Key_WakeUp, 188 },
        { (int)Qt::Key_Eject, 189 },
        { (int)Qt::Key_ScreenSaver, 190 },
        { (int)Qt::Key_WWW, 191 },
        { (int)Qt::Key_Memo, 192 },
        { (int)Qt::Key_LightBulb, 193 },
        { (int)Qt::Key_Shop, 194 },
        { (int)Qt::Key_History, 195 },
        { (int)Qt::Key_AddFavorite, 196 },
        { (int)Qt::Key_HotLinks, 197 },
        { (int)Qt::Key_BrightnessAdjust, 198 },
        { (int)Qt::Key_Finance, 199 },
        { (int)Qt::Key_Community, 200 },
        { (int)Qt::Key_AudioRewind, 201 },
        { (int)Qt::Key_BackForward, 202 },
        { (int)Qt::Key_ApplicationLeft, 203 },
        { (int)Qt::Key_ApplicationRight, 204 },
        { (int)Qt::Key_Book, 205 },
        { (int)Qt::Key_CD, 206 },
        { (int)Qt::Key_Calculator, 207 },
        { (int)Qt::Key_ToDoList, 208 },
        { (int)Qt::Key_ClearGrab, 209 },
        { (int)Qt::Key_Close, 210 },
        { (int)Qt::Key_Copy, 211 },
        { (int)Qt::Key_Cut, 212 },
        { (int)Qt::Key_Display, 213 },
        { (int)Qt::Key_DOS, 214 },
        { (int)Qt::Key_Documents, 215 },
        { (int)Qt::Key_Excel, 216 },
        { (int)Qt::Key_Explorer, 217 },
        { (int)Qt::Key_Game, 218 },
        { (int)Qt::Key_Go, 219 },
        { (int)Qt::Key_iTouch, 220 },
        { (int)Qt::Key_LogOff, 221 },
        { (int)Qt::Key_Market, 222 },
        { (int)Qt::Key_Meeting, 223 },
        { (int)Qt::Key_MenuKB, 224 },
        { (int)Qt::Key_MenuPB, 225 },
        { (int)Qt::Key_MySites, 226 },
        { (int)Qt::Key_News, 227 },
        { (int)Qt::Key_OfficeHome, 228 },
        { (int)Qt::Key_Option, 229 },
        { (int)Qt::Key_Paste, 230 },
        { (int)Qt::Key_Phone, 231 },
        { (int)Qt::Key_Calendar, 232 },
        { (int)Qt::Key_Reply, 233 },
        { (int)Qt::Key_Reload, 234 },
        { (int)Qt::Key_RotateWindows, 235 },
        { (int)Qt::Key_RotationPB, 236 },
        { (int)Qt::Key_RotationKB, 237 },
        { (int)Qt::Key_Save, 238 },
        { (int)Qt::Key_Send, 239 },
        { (int)Qt::Key_Spell, 240 },
        { (int)Qt::Key_SplitScreen, 241 },
        { (int)Qt::Key_Support, 242 },
        { (int)Qt::Key_TaskPane, 243 },
        { (int)Qt::Key_Terminal, 244 },
        { (int)Qt::Key_Tools, 245 },
        { (int)Qt::Key_Travel, 246 },
        { (int)Qt::Key_Video, 247 },
        { (int)Qt::Key_Word, 248 },
        { (int)Qt::Key_Xfer, 249 },
        { (int)Qt::Key_ZoomIn, 250 },
        { (int)Qt::Key_ZoomOut, 251 },
        { (int)Qt::Key_Away, 252 },
        { (int)Qt::Key_Messenger, 253 },
        { (int)Qt::Key_WebCam, 254 },
        { (int)Qt::Key_MailForward, 255 },
        { (int)Qt::Key_Pictures, 256 },
        { (int)Qt::Key_Music, 257 },
        { (int)Qt::Key_Battery, 258 },
        { (int)Qt::Key_Bluetooth, 259 },
        { (int)Qt::Key_WLAN, 260 },
        { (int)Qt::Key_UWB, 261 },
        { (int)Qt::Key_AudioForward, 262 },
        { (int)Qt::Key_AudioRepeat, 263 },
        { (int)Qt::Key_AudioRandomPlay, 264 },
        { (int)Qt::Key_Subtitle, 265 },
        { (int)Qt::Key_AudioCycleTrack, 266 },
        { (int)Qt::Key_Time, 267 },
        { (int)Qt::Key_Hibernate, 268 },
        { (int)Qt::Key_View, 269 },
        { (int)Qt::Key_TopMenu, 270 },
        { (int)Qt::Key_PowerDown, 271 },
        { (int)Qt::Key_Suspend, 272 },
        { (int)Qt::Key_ContrastAdjust, 273 },
        { (int)Qt::Key_LaunchG, 274 },
        { (int)Qt::Key_LaunchH, 275 },
        { (int)Qt::Key_TouchpadToggle, 276 },
        { (int)Qt::Key_TouchpadOn, 277 },
        { (int)Qt::Key_TouchpadOff, 278 },
        { (int)Qt::Key_MicMute, 279 },
        { (int)Qt::Key_Red, 280 },
        { (int)Qt::Key_Green, 281 },
        { (int)Qt::Key_Yellow, 282 },
        { (int)Qt::Key_Blue, 283 },
        { (int)Qt::Key_ChannelUp, 284 },
        { (int)Qt::Key_ChannelDown, 285 },
        { (int)Qt::Key_Guide, 286 },
        { (int)Qt::Key_Info, 287 },
        { (int)Qt::Key_Settings, 288 },
        { (int)Qt::Key_MicVolumeUp, 289 },
        { (int)Qt::Key_MicVolumeDown, 290 },
        { (int)Qt::Key_New, 291 },
        { (int)Qt::Key_Open, 292 },
        { (int)Qt::Key_Find, 293 },
        { (int)Qt::Key_Undo, 294 },
        { (int)Qt::Key_Redo, 295 },
        { (int)Qt::Key_MediaLast, 296 },
        { (int)Qt::Key_Select, 297 },
        { (int)Qt::Key_Yes, 298 },
        { (int)Qt::Key_No, 299 },
        { (int)Qt::Key_Cancel, 300 },
        { (int)Qt::Key_Printer, 301 },
        { (int)Qt::Key_Execute, 302 },
        { (int)Qt::Key_Sleep, 303 },
        { (int)Qt::Key_Play, 304 },
        { (int)Qt::Key_Zoom, 305 },
        { (int)Qt::Key_Exit, 306 },
        { (int)Qt::Key_Context1, 307 },
        { (int)Qt::Key_Context2, 308 },
        { (int)Qt::Key_Context3, 309 },
        { (int)Qt::Key_Context4, 310 },
        { (int)Qt::Key_Call, 311 },
        { (int)Qt::Key_Hangup, 312 },
        { (int)Qt::Key_Flip, 313 },
        { (int)Qt::Key_ToggleCallHangup, 314 },
        { (int)Qt::Key_VoiceDial, 315 },
        { (int)Qt::Key_LastNumberRedial, 316 },
        { (int)Qt::Key_Camera, 317 },
        { (int)Qt::Key_CameraFocus, 318 },
    };

    for (int i = 0; i < 319; ++i) {
        s_keys_lookup[keys_vals[i].key] = keys_vals[i].val;
    };
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRWidget : public QWidget {
    Q_OBJECT
public:
    WRWidget(QWidget* widget) : QWidget(widget) { }
    virtual ~WRWidget() {}

public:
    virtual void paintEvent(QPaintEvent* event);
    void (*m_paint)(RUBase*, void*, struct RUPaintEvent event) = nullptr;
    void* m_paint_user_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRListWidget : public QListWidget {
    Q_OBJECT
public:
    WRListWidget(QWidget* widget) : QListWidget(widget) { }
    virtual ~WRListWidget() {}

public:
    virtual void paintEvent(QPaintEvent* event);
    void (*m_paint)(RUBase*, void*, struct RUPaintEvent event) = nullptr;
    void* m_paint_user_data = nullptr;
public:
    virtual void dragEnterEvent(QDragEnterEvent* event);
    void (*m_drag_enter)(RUBase*, void*, struct RUDragEnterEvent event) = nullptr;
    void* m_drag_enter_user_data = nullptr;
public:
    virtual void dropEvent(QDropEvent* event);
    void (*m_drop)(RUBase*, void*, struct RUDropEvent event) = nullptr;
    void* m_drop_user_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRMenu : public QMenu {
    Q_OBJECT
public:
    WRMenu(QWidget* widget) : QMenu(widget) { }
    virtual ~WRMenu() {}

public:
    virtual void paintEvent(QPaintEvent* event);
    void (*m_paint)(RUBase*, void*, struct RUPaintEvent event) = nullptr;
    void* m_paint_user_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRMenuBar : public QMenuBar {
    Q_OBJECT
public:
    WRMenuBar(QWidget* widget) : QMenuBar(widget) { }
    virtual ~WRMenuBar() {}

public:
    virtual void paintEvent(QPaintEvent* event);
    void (*m_paint)(RUBase*, void*, struct RUPaintEvent event) = nullptr;
    void* m_paint_user_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRLabel : public QLabel {
    Q_OBJECT
public:
    WRLabel(QWidget* widget) : QLabel(widget) { }
    virtual ~WRLabel() {}

public:
    virtual void paintEvent(QPaintEvent* event);
    void (*m_paint)(RUBase*, void*, struct RUPaintEvent event) = nullptr;
    void* m_paint_user_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRLineEdit : public QLineEdit {
    Q_OBJECT
public:
    WRLineEdit(QWidget* widget) : QLineEdit(widget) { }
    virtual ~WRLineEdit() {}

public:
    virtual void paintEvent(QPaintEvent* event);
    void (*m_paint)(RUBase*, void*, struct RUPaintEvent event) = nullptr;
    void* m_paint_user_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRPlainTextEdit : public QPlainTextEdit {
    Q_OBJECT
public:
    WRPlainTextEdit(QWidget* widget) : QPlainTextEdit(widget) { }
    virtual ~WRPlainTextEdit() {}

public:
    virtual void paintEvent(QPaintEvent* event);
    void (*m_paint)(RUBase*, void*, struct RUPaintEvent event) = nullptr;
    void* m_paint_user_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRSlider : public QSlider {
    Q_OBJECT
public:
    WRSlider(QWidget* widget) : QSlider(widget) { }
    virtual ~WRSlider() {}

public:
    virtual void paintEvent(QPaintEvent* event);
    void (*m_paint)(RUBase*, void*, struct RUPaintEvent event) = nullptr;
    void* m_paint_user_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRMainWindow : public QMainWindow {
    Q_OBJECT
public:
    WRMainWindow(QWidget* widget) : QMainWindow(widget) { }
    virtual ~WRMainWindow() {}

public:
    virtual void paintEvent(QPaintEvent* event);
    void (*m_paint)(RUBase*, void*, struct RUPaintEvent event) = nullptr;
    void* m_paint_user_data = nullptr;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRVBoxLayout : public QVBoxLayout {
    Q_OBJECT
public:
    WRVBoxLayout(QWidget* widget) : QVBoxLayout(widget) { }
    virtual ~WRVBoxLayout() {}

};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

class WRHBoxLayout : public QHBoxLayout {
    Q_OBJECT
public:
    WRHBoxLayout(QWidget* widget) : QHBoxLayout(widget) { }
    virtual ~WRHBoxLayout() {}

};

void WRWidget::paintEvent(QPaintEvent* event) {
    if (m_paint) {
        RUPaintEvent e;
        e.funcs = &s_paint_event_funcs;
        e.priv_data = (struct RUBase*)event;
        RUWidget w;
        w.funcs = &s_widget_funcs;
        w.priv_data = (struct RUBase*)this;
        m_paint((struct RUBase*)&w, m_paint_user_data, (struct RUBase*)&e);
    } else {
        QWidget::paintEvent(event);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void set_widget_paint_event(void* object, void* user_data, void (*event)(struct RUBase* widget, void* self_c, struct RUPaintEvent event)) {
    WRWidget* qt_object = (WRWidget*)object;
    qt_object->m_paint_user_data = user_data;
    qt_object->m_paint = event;
};

void WRListWidget::paintEvent(QPaintEvent* event) {
    if (m_paint) {
        RUPaintEvent e;
        e.funcs = &s_paint_event_funcs;
        e.priv_data = (struct RUBase*)event;
        RUListWidget w;
        w.funcs = &s_list_widget_funcs;
        w.priv_data = (struct RUBase*)this;
        m_paint((struct RUBase*)&w, m_paint_user_data, (struct RUBase*)&e);
    } else {
        QListWidget::paintEvent(event);
    }
}

void WRListWidget::dragEnterEvent(QDragEnterEvent* event) {
    if (m_drag_enter) {
        RUDragEnterEvent e;
        e.funcs = &s_drag_enter_event_funcs;
        e.priv_data = (struct RUBase*)event;
        RUListWidget w;
        w.funcs = &s_list_widget_funcs;
        w.priv_data = (struct RUBase*)this;
        m_drag_enter((struct RUBase*)&w, m_drag_enter_user_data, (struct RUBase*)&e);
    } else {
        QListWidget::dragEnterEvent(event);
    }
}

void WRListWidget::dropEvent(QDropEvent* event) {
    if (m_drop) {
        RUDropEvent e;
        e.funcs = &s_drop_event_funcs;
        e.priv_data = (struct RUBase*)event;
        RUListWidget w;
        w.funcs = &s_list_widget_funcs;
        w.priv_data = (struct RUBase*)this;
        m_drop((struct RUBase*)&w, m_drop_user_data, (struct RUBase*)&e);
    } else {
        QListWidget::dropEvent(event);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void set_list_widget_paint_event(void* object, void* user_data, void (*event)(struct RUBase* widget, void* self_c, struct RUPaintEvent event)) {
    WRListWidget* qt_object = (WRListWidget*)object;
    qt_object->m_paint_user_data = user_data;
    qt_object->m_paint = event;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void set_list_widget_drag_enter_event(void* object, void* user_data, void (*event)(struct RUBase* widget, void* self_c, struct RUDragEnterEvent event)) {
    WRListWidget* qt_object = (WRListWidget*)object;
    qt_object->m_drag_enter_user_data = user_data;
    qt_object->m_drag_enter = event;
};

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void set_list_widget_drop_event(void* object, void* user_data, void (*event)(struct RUBase* widget, void* self_c, struct RUDropEvent event)) {
    WRListWidget* qt_object = (WRListWidget*)object;
    qt_object->m_drop_user_data = user_data;
    qt_object->m_drop = event;
};

void WRMenu::paintEvent(QPaintEvent* event) {
    if (m_paint) {
        RUPaintEvent e;
        e.funcs = &s_paint_event_funcs;
        e.priv_data = (struct RUBase*)event;
        RUMenu w;
        w.funcs = &s_menu_funcs;
        w.priv_data = (struct RUBase*)this;
        m_paint((struct RUBase*)&w, m_paint_user_data, (struct RUBase*)&e);
    } else {
        QMenu::paintEvent(event);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void set_menu_paint_event(void* object, void* user_data, void (*event)(struct RUBase* widget, void* self_c, struct RUPaintEvent event)) {
    WRMenu* qt_object = (WRMenu*)object;
    qt_object->m_paint_user_data = user_data;
    qt_object->m_paint = event;
};

void WRMenuBar::paintEvent(QPaintEvent* event) {
    if (m_paint) {
        RUPaintEvent e;
        e.funcs = &s_paint_event_funcs;
        e.priv_data = (struct RUBase*)event;
        RUMenuBar w;
        w.funcs = &s_menu_bar_funcs;
        w.priv_data = (struct RUBase*)this;
        m_paint((struct RUBase*)&w, m_paint_user_data, (struct RUBase*)&e);
    } else {
        QMenuBar::paintEvent(event);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void set_menu_bar_paint_event(void* object, void* user_data, void (*event)(struct RUBase* widget, void* self_c, struct RUPaintEvent event)) {
    WRMenuBar* qt_object = (WRMenuBar*)object;
    qt_object->m_paint_user_data = user_data;
    qt_object->m_paint = event;
};

void WRLabel::paintEvent(QPaintEvent* event) {
    if (m_paint) {
        RUPaintEvent e;
        e.funcs = &s_paint_event_funcs;
        e.priv_data = (struct RUBase*)event;
        RULabel w;
        w.funcs = &s_label_funcs;
        w.priv_data = (struct RUBase*)this;
        m_paint((struct RUBase*)&w, m_paint_user_data, (struct RUBase*)&e);
    } else {
        QLabel::paintEvent(event);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void set_label_paint_event(void* object, void* user_data, void (*event)(struct RUBase* widget, void* self_c, struct RUPaintEvent event)) {
    WRLabel* qt_object = (WRLabel*)object;
    qt_object->m_paint_user_data = user_data;
    qt_object->m_paint = event;
};

void WRLineEdit::paintEvent(QPaintEvent* event) {
    if (m_paint) {
        RUPaintEvent e;
        e.funcs = &s_paint_event_funcs;
        e.priv_data = (struct RUBase*)event;
        RULineEdit w;
        w.funcs = &s_line_edit_funcs;
        w.priv_data = (struct RUBase*)this;
        m_paint((struct RUBase*)&w, m_paint_user_data, (struct RUBase*)&e);
    } else {
        QLineEdit::paintEvent(event);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void set_line_edit_paint_event(void* object, void* user_data, void (*event)(struct RUBase* widget, void* self_c, struct RUPaintEvent event)) {
    WRLineEdit* qt_object = (WRLineEdit*)object;
    qt_object->m_paint_user_data = user_data;
    qt_object->m_paint = event;
};

void WRPlainTextEdit::paintEvent(QPaintEvent* event) {
    if (m_paint) {
        RUPaintEvent e;
        e.funcs = &s_paint_event_funcs;
        e.priv_data = (struct RUBase*)event;
        RUPlainTextEdit w;
        w.funcs = &s_plain_text_edit_funcs;
        w.priv_data = (struct RUBase*)this;
        m_paint((struct RUBase*)&w, m_paint_user_data, (struct RUBase*)&e);
    } else {
        QPlainTextEdit::paintEvent(event);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void set_plain_text_edit_paint_event(void* object, void* user_data, void (*event)(struct RUBase* widget, void* self_c, struct RUPaintEvent event)) {
    WRPlainTextEdit* qt_object = (WRPlainTextEdit*)object;
    qt_object->m_paint_user_data = user_data;
    qt_object->m_paint = event;
};

void WRSlider::paintEvent(QPaintEvent* event) {
    if (m_paint) {
        RUPaintEvent e;
        e.funcs = &s_paint_event_funcs;
        e.priv_data = (struct RUBase*)event;
        RUSlider w;
        w.funcs = &s_slider_funcs;
        w.priv_data = (struct RUBase*)this;
        m_paint((struct RUBase*)&w, m_paint_user_data, (struct RUBase*)&e);
    } else {
        QSlider::paintEvent(event);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void set_slider_paint_event(void* object, void* user_data, void (*event)(struct RUBase* widget, void* self_c, struct RUPaintEvent event)) {
    WRSlider* qt_object = (WRSlider*)object;
    qt_object->m_paint_user_data = user_data;
    qt_object->m_paint = event;
};

void WRMainWindow::paintEvent(QPaintEvent* event) {
    if (m_paint) {
        RUPaintEvent e;
        e.funcs = &s_paint_event_funcs;
        e.priv_data = (struct RUBase*)event;
        RUMainWindow w;
        w.funcs = &s_main_window_funcs;
        w.priv_data = (struct RUBase*)this;
        m_paint((struct RUBase*)&w, m_paint_user_data, (struct RUBase*)&e);
    } else {
        QMainWindow::paintEvent(event);
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

void set_main_window_paint_event(void* object, void* user_data, void (*event)(struct RUBase* widget, void* self_c, struct RUPaintEvent event)) {
    WRMainWindow* qt_object = (WRMainWindow*)object;
    qt_object->m_paint_user_data = user_data;
    qt_object->m_paint = event;
};

static struct RUAction create_action(struct RUBase* priv_data) {
    return create_generic_func<struct RUAction, struct RUActionFuncs, QAction>(&s_action_funcs, priv_data);
}

static void destroy_action(struct RUBase* priv_data) {
    destroy_generic<QAction>(priv_data);
}

static struct RUWidget create_widget(struct RUBase* priv_data) {
    return create_widget_func<struct RUWidget, struct RUWidgetFuncs, WRWidget>(&s_widget_funcs, priv_data);
}

static void destroy_widget(struct RUBase* priv_data) {
    destroy_generic<WRWidget>(priv_data);
}

static struct RUListWidgetItem create_list_widget_item(struct RUBase* priv_data) {
    return create_generic_func<struct RUListWidgetItem, struct RUListWidgetItemFuncs, QListWidgetItem>(&s_list_widget_item_funcs, priv_data);
}

static void destroy_list_widget_item(struct RUBase* priv_data) {
    destroy_generic<QListWidgetItem>(priv_data);
}

static struct RUListWidget create_list_widget(struct RUBase* priv_data) {
    return create_widget_func<struct RUListWidget, struct RUListWidgetFuncs, WRListWidget>(&s_list_widget_funcs, priv_data);
}

static void destroy_list_widget(struct RUBase* priv_data) {
    destroy_generic<WRListWidget>(priv_data);
}

static struct RUTimer create_timer(struct RUBase* priv_data) {
    return create_generic_func<struct RUTimer, struct RUTimerFuncs, QTimer>(&s_timer_funcs, priv_data);
}

static void destroy_timer(struct RUBase* priv_data) {
    destroy_generic<QTimer>(priv_data);
}

static struct RUIcon create_icon(struct RUBase* priv_data) {
    return create_generic_func<struct RUIcon, struct RUIconFuncs, QIcon>(&s_icon_funcs, priv_data);
}

static void destroy_icon(struct RUBase* priv_data) {
    destroy_generic<QIcon>(priv_data);
}

static struct RUMenu create_menu(struct RUBase* priv_data) {
    return create_widget_func<struct RUMenu, struct RUMenuFuncs, WRMenu>(&s_menu_funcs, priv_data);
}

static void destroy_menu(struct RUBase* priv_data) {
    destroy_generic<WRMenu>(priv_data);
}

static struct RUMenuBar create_menu_bar(struct RUBase* priv_data) {
    return create_widget_func<struct RUMenuBar, struct RUMenuBarFuncs, WRMenuBar>(&s_menu_bar_funcs, priv_data);
}

static void destroy_menu_bar(struct RUBase* priv_data) {
    destroy_generic<WRMenuBar>(priv_data);
}

static struct RULabel create_label(struct RUBase* priv_data) {
    return create_widget_func<struct RULabel, struct RULabelFuncs, WRLabel>(&s_label_funcs, priv_data);
}

static void destroy_label(struct RUBase* priv_data) {
    destroy_generic<WRLabel>(priv_data);
}

static struct RULineEdit create_line_edit(struct RUBase* priv_data) {
    return create_widget_func<struct RULineEdit, struct RULineEditFuncs, WRLineEdit>(&s_line_edit_funcs, priv_data);
}

static void destroy_line_edit(struct RUBase* priv_data) {
    destroy_generic<WRLineEdit>(priv_data);
}

static struct RUPlainTextEdit create_plain_text_edit(struct RUBase* priv_data) {
    return create_widget_func<struct RUPlainTextEdit, struct RUPlainTextEditFuncs, WRPlainTextEdit>(&s_plain_text_edit_funcs, priv_data);
}

static void destroy_plain_text_edit(struct RUBase* priv_data) {
    destroy_generic<WRPlainTextEdit>(priv_data);
}

static struct RUSlider create_slider(struct RUBase* priv_data) {
    return create_widget_func<struct RUSlider, struct RUSliderFuncs, WRSlider>(&s_slider_funcs, priv_data);
}

static void destroy_slider(struct RUBase* priv_data) {
    destroy_generic<WRSlider>(priv_data);
}

static struct RUMainWindow create_main_window(struct RUBase* priv_data) {
    return create_widget_func<struct RUMainWindow, struct RUMainWindowFuncs, WRMainWindow>(&s_main_window_funcs, priv_data);
}

static void destroy_main_window(struct RUBase* priv_data) {
    destroy_generic<WRMainWindow>(priv_data);
}

static struct RUVBoxLayout create_v_box_layout(struct RUBase* priv_data) {
    return create_widget_func<struct RUVBoxLayout, struct RUVBoxLayoutFuncs, WRVBoxLayout>(&s_v_box_layout_funcs, priv_data);
}

static void destroy_v_box_layout(struct RUBase* priv_data) {
    destroy_generic<WRVBoxLayout>(priv_data);
}

static struct RUHBoxLayout create_h_box_layout(struct RUBase* priv_data) {
    return create_widget_func<struct RUHBoxLayout, struct RUHBoxLayoutFuncs, WRHBoxLayout>(&s_h_box_layout_funcs, priv_data);
}

static void destroy_h_box_layout(struct RUBase* priv_data) {
    destroy_generic<WRHBoxLayout>(priv_data);
}

