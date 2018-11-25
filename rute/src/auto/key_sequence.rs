// This file is auto-generated by rute_gen. DO NOT EDIT.
use std::cell::Cell;
use std::rc::Rc;

#[allow(unused_imports)]
use std::marker::PhantomData;

#[allow(unused_imports)]
use std::os::raw::c_void;

#[allow(unused_imports)]
use std::mem::transmute;

#[allow(unused_imports)]
use std::ffi::{CStr, CString};

use rute_ffi_base::*;

#[allow(unused_imports)]
use auto::*;

///
/// In its most common form, a key sequence describes a combination of
/// keys that must be used together to perform some action. Key sequences
/// are used with QAction objects to specify which keyboard shortcuts can
/// be used to trigger actions.
///
/// Key sequences can be constructed for use as keyboard shortcuts in
/// three different ways:
///
/// * For standard shortcuts, a [standard key](QKeySequence::StandardKey)
/// can be used to request the platform-specific key sequence associated with each shortcut.
/// * For custom shortcuts, human-readable strings such as "Ctrl+X" can be used, and these can be translated into the appropriate shortcuts for users of different languages. Translations are made in the "QShortcut" context.
/// * For hard-coded shortcuts, integer key codes can be specified with a combination of values defined by the Qt::Key and Qt::Modifier enum values. Each key code consists of a single Qt::Key value and zero or more modifiers, such as Qt::SHIFT, Qt::CTRL, Qt::ALT and Qt::META.
///
/// For example, **{Ctrl** P} might be a sequence used as a shortcut for
/// printing a document, and can be specified in any of the following
/// ways:
///
/// Note that, for letters, the case used in the specification string
/// does not matter. In the above examples, the user does not need to
/// hold down the **{Shift}** key to activate a shortcut specified
/// with . However, for other keys, the use of **{Shift}** as
/// an unspecified extra modifier key can lead to confusion for users
/// of an application whose keyboards have different layouts to those
/// used by the developers. See the [Keyboard Layout Issues](Keyboard%20Layout%20Issues)
/// section
/// below for more details.
///
/// It is preferable to use standard shortcuts where possible.
/// When creating key sequences for non-standard shortcuts, you should use
/// human-readable strings in preference to hard-coded integer values.
///
/// QKeySequence objects can be cast to a QString to obtain a human-readable
/// translated version of the sequence. Similarly, the toString() function
/// produces human-readable strings for use in menus. On MacOS , the
/// appropriate symbols are used to describe keyboard shortcuts using special
/// keys on the Macintosh keyboard.
///
/// An alternative way to specify hard-coded key codes is to use the Unicode
/// code point of the character; for example, 'A' gives the same key sequence
/// as Qt::Key_A.
///
/// **Note**: On MacOS , references to , Qt::CTRL, Qt::Key_Control
/// and Qt::ControlModifier correspond to the **Command** keys on the
/// Macintosh keyboard, and references to , Qt::META, Qt::Key_Meta and
/// Qt::MetaModifier correspond to the **Control** keys. Developers on
/// MacOS can use the same shortcut descriptions across all platforms,
/// and their applications will automatically work as expected on MacOS .
///
/// # Standard Shortcuts
///
/// QKeySequence defines many [standard
/// keyboard shortcuts](QKeySequence::StandardKey)
/// to reduce the amount of effort required when
/// setting up actions in a typical application. The table below shows
/// some common key sequences that are often used for these standard
/// shortcuts by applications on four widely-used platforms. Note
/// that on MacOS , the **Ctrl** value corresponds to the **Command** keys on the Macintosh keyboard, and the **Meta** value
/// corresponds to the **Control** keys.
///
/// * StandardKey
/// * Windows
/// * MacOS
/// * KDE Plasma
/// * GNOME
/// * HelpContents
/// * F1
/// * Ctrl+?
/// * F1
/// * WhatsThis
/// * Shift+F1
/// * Open
/// * Ctrl+O
/// * Close
/// * Ctrl+F4, Ctrl+W
/// * Ctrl+W, Ctrl+F4
/// * Ctrl+W
/// * Save
/// * Ctrl+S
/// * Quit
///
/// * Ctrl+Q
/// * SaveAs
///
/// * Ctrl+Shift+S
///
/// * Ctrl+Shift+S
/// * New
/// * Ctrl+N
/// * Delete
/// * Del
/// * Del, Meta+D
/// * Del, Ctrl+D
/// * Cut
/// * Ctrl+X, Shift+Del
/// * Ctrl+X, Meta+K
/// * Ctrl+X, F20, Shift+Del
/// * Copy
/// * Ctrl+C, Ctrl+Ins
/// * Ctrl+C
/// * Ctrl+C, F16, Ctrl+Ins
/// * Paste
/// * Ctrl+V, Shift+Ins
/// * Ctrl+V, Meta+Y
/// * Ctrl+V, F18, Shift+Ins
/// * Preferences
///
/// * Ctrl+,
///
/// * Undo
/// * Ctrl+Z, Alt+Backspace
/// * Ctrl+Z
/// * Ctrl+Z, F14
/// * Redo
/// * Ctrl+Y, Shift+Ctrl+Z, Alt+Shift+Backspace
/// * Ctrl+Shift+Z
/// * Back
/// * Alt+Left, Backspace
/// * Ctrl+[
/// * Alt+Left
/// * Forward
/// * Alt+Right, Shift+Backspace
/// * Ctrl+]
/// * Alt+Right
/// * Refresh
/// * F5
/// * Ctrl+R, F5
/// * ZoomIn
/// * Ctrl+Plus
/// * ZoomOut
/// * Ctrl+Minus
/// * FullScreen
/// * F11, Alt+Enter
/// * Ctrl+Meta+F
/// * F11, Ctrl+Shift+F
/// * Ctrl+F11
/// * Print
/// * Ctrl+P
/// * AddTab
/// * Ctrl+T
/// * Ctrl+Shift+N, Ctrl+T
/// * Ctrl+T
/// * NextChild
/// * Ctrl+Tab, Forward, Ctrl+F6
/// * Ctrl+}, Forward, Ctrl+Tab
/// * Ctrl+Tab, Forward, Ctrl+Comma
/// * Ctrl+Tab, Forward
/// * PreviousChild
/// * Ctrl+Shift+Tab, Back, Ctrl+Shift+F6
/// * Ctrl+{, Back, Ctrl+Shift+Tab
/// * Ctrl+Shift+Tab, Back, Ctrl+Period
/// * Ctrl+Shift+Tab, Back
/// * Find
/// * Ctrl+F
/// * FindNext
/// * F3, Ctrl+G
/// * Ctrl+G
/// * F3
/// * Ctrl+G, F3
/// * FindPrevious
/// * Shift+F3, Ctrl+Shift+G
/// * Ctrl+Shift+G
/// * Shift+F3
/// * Ctrl+Shift+G, Shift+F3
/// * Replace
/// * Ctrl+H
/// * (none)
/// * Ctrl+R
/// * Ctrl+H
/// * SelectAll
/// * Ctrl+A
/// * Deselect
///
/// * Ctrl+Shift+A
/// * Bold
/// * Ctrl+B
/// * Italic
/// * Ctrl+I
/// * Underline
/// * Ctrl+U
/// * MoveToNextChar
/// * Right
/// * Right, Meta+F
/// * Right
/// * MoveToPreviousChar
/// * Left
/// * Left, Meta+B
/// * Left
/// * MoveToNextWord
/// * Ctrl+Right
/// * Alt+Right
/// * Ctrl+Right
/// * MoveToPreviousWord
/// * Ctrl+Left
/// * Alt+Left
/// * Ctrl+Left
/// * MoveToNextLine
/// * Down
/// * Down, Meta+N
/// * Down
/// * MoveToPreviousLine
/// * Up
/// * Up, Meta+P
/// * Up
/// * MoveToNextPage
/// * PgDown
/// * PgDown, Alt+PgDown, Meta+Down, Meta+PgDown, Meta+V
/// * PgDown
/// * MoveToPreviousPage
/// * PgUp
/// * PgUp, Alt+PgUp, Meta+Up, Meta+PgUp
/// * PgUp
/// * MoveToStartOfLine
/// * Home
/// * Ctrl+Left, Meta+Left
/// * Home
/// * MoveToEndOfLine
/// * End
/// * Ctrl+Right, Meta+Right
/// * End, Ctrl+E
/// * MoveToStartOfBlock
/// * (none)
/// * Alt+Up, Meta+A
/// * (none)
/// * MoveToEndOfBlock
/// * (none)
/// * Alt+Down, Meta+E
/// * (none)
/// * MoveToStartOfDocument
/// * Ctrl+Home
/// * Ctrl+Up, Home
/// * Ctrl+Home
/// * MoveToEndOfDocument
/// * Ctrl+End
/// * Ctrl+Down, End
/// * Ctrl+End
/// * SelectNextChar
/// * Shift+Right
/// * SelectPreviousChar
/// * Shift+Left
/// * SelectNextWord
/// * Ctrl+Shift+Right
/// * Alt+Shift+Right
/// * Ctrl+Shift+Right
/// * SelectPreviousWord
/// * Ctrl+Shift+Left
/// * Alt+Shift+Left
/// * Ctrl+Shift+Left
/// * SelectNextLine
/// * Shift+Down
/// * SelectPreviousLine
/// * Shift+Up
/// * SelectNextPage
/// * Shift+PgDown
/// * SelectPreviousPage
/// * Shift+PgUp
/// * SelectStartOfLine
/// * Shift+Home
/// * Ctrl+Shift+Left
/// * Shift+Home
/// * SelectEndOfLine
/// * Shift+End
/// * Ctrl+Shift+Right
/// * Shift+End
/// * SelectStartOfBlock
/// * (none)
/// * Alt+Shift+Up, Meta+Shift+A
/// * (none)
/// * SelectEndOfBlock
/// * (none)
/// * Alt+Shift+Down, Meta+Shift+E
/// * (none)
/// * SelectStartOfDocument
/// * Ctrl+Shift+Home
/// * Ctrl+Shift+Up, Shift+Home
/// * Ctrl+Shift+Home
/// * SelectEndOfDocument
/// * Ctrl+Shift+End
/// * Ctrl+Shift+Down, Shift+End
/// * Ctrl+Shift+End
/// * DeleteStartOfWord
/// * Ctrl+Backspace
/// * Alt+Backspace
/// * Ctrl+Backspace
/// * DeleteEndOfWord
/// * Ctrl+Del
/// * (none)
/// * Ctrl+Del
/// * DeleteEndOfLine
/// * (none)
/// * Ctrl+K
/// * DeleteCompleteLine
/// * (none)
/// * Ctrl+U
/// * InsertParagraphSeparator
/// * Enter
/// * InsertLineSeparator
/// * Shift+Enter
/// * Meta+Enter, Meta+O
/// * Shift+Enter
/// * Backspace
/// * (none)
/// * Meta+H
/// * (none)
/// * Cancel
/// * Escape
/// * Escape, Ctrl+.
/// * Escape
///
/// Note that, since the key sequences used for the standard shortcuts differ
/// between platforms, you still need to test your shortcuts on each platform
/// to ensure that you do not unintentionally assign the same key sequence to
/// many actions.
///
/// # Keyboard Layout Issues
///
/// Many key sequence specifications are chosen by developers based on the
/// layout of certain types of keyboard, rather than choosing keys that
/// represent the first letter of an action's name, such as **{Ctrl** S}
/// ("Ctrl+S") or **{Ctrl** C} ("Ctrl+C").
/// Additionally, because certain symbols can only be entered with the
/// help of modifier keys on certain keyboard layouts, key sequences intended
/// for use with one keyboard layout may map to a different key, map to no
/// keys at all, or require an additional modifier key to be used on
/// different keyboard layouts.
///
/// For example, the shortcuts, **{Ctrl** plus} and **{Ctrl** minus}, are often
/// used as shortcuts for zoom operations in graphics applications, and these
/// may be specified as and respectively. However, the way
/// these shortcuts are specified and interpreted depends on the keyboard layout.
/// Users of Norwegian keyboards will note that the **{+}** and **{-}** keys
/// are not adjacent on the keyboard, but will still be able to activate both
/// shortcuts without needing to press the **{Shift}** key. However, users
/// with British keyboards will need to hold down the **{Shift}** key
/// to enter the **{+}** symbol, making the shortcut effectively the same as
///
///
/// Although some developers might resort to fully specifying all the modifiers
/// they use on their keyboards to activate a shortcut, this will also result
/// in unexpected behavior for users of different keyboard layouts.
///
/// For example, a developer using a British keyboard may decide to specify
/// as the key sequence in order to create a shortcut that
/// coincidentally behaves in the same way as **{Ctrl** plus}. However, the
/// **{=}** key needs to be accessed using the **{Shift}** key on Norwegian
/// keyboard, making the required shortcut effectively **{Ctrl** Shift Shift =}
/// (an impossible key combination).
///
/// As a result, both human-readable strings and hard-coded key codes
/// can both be problematic to use when specifying a key sequence that
/// can be used on a variety of different keyboard layouts. Only the
/// use of [standard shortcuts](QKeySequence::StandardKey)
///
/// guarantees that the user will be able to use the shortcuts that
/// the developer intended.
///
/// Despite this, we can address this issue by ensuring that human-readable
/// strings are used, making it possible for translations of key sequences to
/// be made for users of different languages. This approach will be successful
/// for users whose keyboards have the most typical layout for the language
/// they are using.
///
/// # GNU Emacs Style Key Sequences
///
/// Key sequences similar to those used in [GNU Emacs](http://www.gnu.org/software/emacs/)
/// , allowing up to four
/// key codes, can be created by using the multiple argument constructor,
/// or by passing a human-readable string of comma-separated key sequences.
///
/// For example, the key sequence, **{Ctrl** X} followed by **{Ctrl** C}, can
/// be specified using either of the following ways:
///
/// **Warning**: A QApplication instance must have been constructed before a
/// QKeySequence is created; otherwise, your application may crash.
///
/// **See also:** [`Shortcut`]
/// # Licence
///
/// The documentation is an adoption of the original [Qt Documentation](http://doc.qt.io/) and provided herein is licensed under the terms of the [GNU Free Documentation License version 1.3](http://www.gnu.org/licenses/fdl.html) as published by the Free Software Foundation.
#[derive(Clone)]
pub struct KeySequence<'a> {
    #[doc(hidden)]
    pub data: Rc<Cell<Option<*const RUBase>>>,
    #[doc(hidden)]
    pub all_funcs: *const RUKeySequenceAllFuncs,
    #[doc(hidden)]
    pub owned: bool,
    #[doc(hidden)]
    pub _marker: PhantomData<::std::cell::Cell<&'a ()>>,
}

impl<'a> KeySequence<'a> {
    pub fn new() -> KeySequence<'a> {
        let data = Rc::new(Cell::new(None));

        let ffi_data = unsafe {
            ((*rute_ffi_get()).create_key_sequence)(
                ::std::ptr::null(),
                transmute(rute_object_delete_callback as usize),
                Rc::into_raw(data.clone()) as *const c_void,
            )
        };

        data.set(Some(ffi_data.qt_data));

        KeySequence {
            data,
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }
    #[allow(dead_code)]
    pub(crate) fn new_from_rc(ffi_data: RUKeySequence) -> KeySequence<'a> {
        KeySequence {
            data: unsafe { Rc::from_raw(ffi_data.host_data as *const Cell<Option<*const RUBase>>) },
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_owned(ffi_data: RUKeySequence) -> KeySequence<'a> {
        KeySequence {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: true,
            _marker: PhantomData,
        }
    }

    #[allow(dead_code)]
    pub(crate) fn new_from_temporary(ffi_data: RUKeySequence) -> KeySequence<'a> {
        KeySequence {
            data: Rc::new(Cell::new(Some(ffi_data.qt_data as *const RUBase))),
            all_funcs: ffi_data.all_funcs,
            owned: false,
            _marker: PhantomData,
        }
    }
    ///
    /// Returns the number of keys in the key sequence.
    /// The maximum is 4.
    pub fn count(&self) -> i32 {
        let (obj_data, funcs) = self.get_key_sequence_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).count)(obj_data);
            ret_val
        }
    }
    ///
    /// Returns `true` if the key sequence is empty; otherwise returns
    /// false.
    pub fn is_empty(&self) -> bool {
        let (obj_data, funcs) = self.get_key_sequence_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_empty)(obj_data);
            ret_val
        }
    }
    ///
    /// Matches the sequence with *seq.* Returns ExactMatch if
    /// successful, PartialMatch if *seq* matches incompletely,
    /// and NoMatch if the sequences have nothing in common.
    /// Returns NoMatch if *seq* is shorter.
    pub fn matches<K: KeySequenceTrait<'a>>(&self, seq: &K) -> SequenceMatch {
        let (obj_seq_1, _funcs) = seq.get_key_sequence_obj_funcs();

        let (obj_data, funcs) = self.get_key_sequence_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).matches)(obj_data, obj_seq_1);
            let ret_val = { transmute::<i32, SequenceMatch>(ret_val) };
            ret_val
        }
    }
    ///
    /// Returns the shortcut key sequence for the mnemonic in *text,*
    /// or an empty key sequence if no mnemonics are found.
    ///
    /// For example, mnemonic("E&xit") returns `Qt::ALT+Qt::Key_X` ,
    /// mnemonic("&Quit") returns `ALT+Key_Q` , and mnemonic("Quit")
    /// returns an empty QKeySequence.
    ///
    /// We provide a [list of common mnemonics](accelerators.html)
    ///
    /// in English. At the time of writing, Microsoft and Open Group do
    /// not appear to have issued equivalent recommendations for other
    /// languages.
    pub fn mnemonic(text: &str) -> KeySequence<'a> {
        let str_in_text_1 = CString::new(text).unwrap();

        let (obj_data, funcs) = unsafe {
            (
                ::std::ptr::null(),
                (*((*rute_ffi_get()).get_key_sequence)(::std::ptr::null()).all_funcs)
                    .key_sequence_funcs,
            )
        };
        unsafe {
            let ret_val = ((*funcs).mnemonic)(obj_data, str_in_text_1.as_ptr());
            let t = ret_val;
            let ret_val;
            if t.host_data != ::std::ptr::null() {
                ret_val = KeySequence::new_from_rc(t);
            } else {
                ret_val = KeySequence::new_from_owned(t);
            }
            ret_val
        }
    }
    ///
    /// Returns a list of key bindings for the given *key.*
    /// The result of calling this function will vary based on the target platform.
    /// The first element of the list indicates the primary shortcut for the given platform.
    /// If the result contains more than one result, these can
    /// be considered alternative shortcuts on the same platform for the given *key.*
    ///
    /// Swaps key sequence *other* with this key sequence. This operation is very
    /// fast and never fails.
    pub fn swap<K: KeySequenceTrait<'a>>(&self, other: &K) -> &Self {
        let (obj_other_1, _funcs) = other.get_key_sequence_obj_funcs();

        let (obj_data, funcs) = self.get_key_sequence_obj_funcs();
        unsafe {
            ((*funcs).swap)(obj_data, obj_other_1);
        }
        self
    }
    pub fn is_detached(&self) -> bool {
        let (obj_data, funcs) = self.get_key_sequence_obj_funcs();
        unsafe {
            let ret_val = ((*funcs).is_detached)(obj_data);
            ret_val
        }
    }
}
pub trait KeySequenceTrait<'a> {
    #[inline]
    #[doc(hidden)]
    fn get_key_sequence_obj_funcs(&self) -> (*const RUBase, *const RUKeySequenceFuncs);
}

impl<'a> KeySequenceTrait<'a> for KeySequence<'a> {
    #[doc(hidden)]
    fn get_key_sequence_obj_funcs(&self) -> (*const RUBase, *const RUKeySequenceFuncs) {
        let obj = self.data.get().unwrap();
        unsafe { (obj, (*self.all_funcs).key_sequence_funcs) }
    }
}
#[repr(u32)]
pub enum StandardKey {
    UnknownKey,
    HelpContents,
    WhatsThis,
    Open,
    Close,
    Save,
    New,
    Delete,
    Cut,
    Copy,
    Paste,
    Undo,
    Redo,
    Back,
    Forward,
    Refresh,
    ZoomIn,
    ZoomOut,
    Print,
    AddTab,
    NextChild,
    PreviousChild,
    Find,
    FindNext,
    FindPrevious,
    Replace,
    SelectAll,
    Bold,
    Italic,
    Underline,
    MoveToNextChar,
    MoveToPreviousChar,
    MoveToNextWord,
    MoveToPreviousWord,
    MoveToNextLine,
    MoveToPreviousLine,
    MoveToNextPage,
    MoveToPreviousPage,
    MoveToStartOfLine,
    MoveToEndOfLine,
    MoveToStartOfBlock,
    MoveToEndOfBlock,
    MoveToStartOfDocument,
    MoveToEndOfDocument,
    SelectNextChar,
    SelectPreviousChar,
    SelectNextWord,
    SelectPreviousWord,
    SelectNextLine,
    SelectPreviousLine,
    SelectNextPage,
    SelectPreviousPage,
    SelectStartOfLine,
    SelectEndOfLine,
    SelectStartOfBlock,
    SelectEndOfBlock,
    SelectStartOfDocument,
    SelectEndOfDocument,
    DeleteStartOfWord,
    DeleteEndOfWord,
    DeleteEndOfLine,
    InsertParagraphSeparator,
    InsertLineSeparator,
    SaveAs,
    Preferences,
    Quit,
    FullScreen,
    Deselect,
    DeleteCompleteLine,
    Backspace,
    Cancel,
}

#[repr(u32)]
pub enum SequenceFormat {
    NativeText,
    PortableText,
}

#[repr(u32)]
pub enum SequenceMatch {
    NoMatch,
    PartialMatch,
    ExactMatch,
}
