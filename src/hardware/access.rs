//! **feature:access** - System Accessibility (*WIP*)
//!
//! This API allows you to create a GUI that automatically adapts to the
//! specific user's needs.  Blind users will get a text interface that
//! can be used with either a screen-reader or braille-display.  The interface
//! will be usable without a mouse (keyboard only) for motion-impaired, blind,
//! and power-users who can't be bothered to touch the mouse.  Localization and
//! adapting to mobile devices will also be handled.

use std::ops::Range;

use pix::{rgb::SRgba8, Raster};

/// Possible Events For Text Input
pub enum TextEv {
    /// Escape Key, Back: To Selection Mode
    Back,
    /// Autocompletion, Indentation, Or Next One-Line-Input
    Tab,
    /// De-Indentation, Or Previous One-Line-Input
    UnTab,
    /// Ctr-X
    Cut,
    /// Alt-X
    Swap,
    /// Ctr-C
    Copy,
    /// Alt-C
    Cancel,
    /// Ctr-V
    Paste,
    /// Alt-V
    PasteUnformat,
    /// Insert
    Clipboard,
    /// Backspace
    Backspace,
    /// Shift+Backspace,Delete
    Delete,
    /// Ctr+Insert
    Emoji,
    /// Alt+Insert
    Compose,
    ///
    Text(char),
}

/// Possible commands to interface with the accessibility system.
pub enum TextCmd {
    /// Turn composing on or off.
    Compose(bool),
}

/// Text Input Field User Interface.
pub trait TextInput {
    /// Event handler.
    fn event(&mut self, event: TextEv);
}

/// User Interface Builder.
// FIXME: Rename Ui
#[derive(Default)]
pub struct UiBuilder {
    text_input: Option<Box<dyn TextInput>>,
}

impl UiBuilder {
    /// Create a new User Interface Builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Add state for editable text fields.
    pub fn text<Text>(mut self, text: Text) -> Self
    where
        Text: 'static + TextInput,
    {
        self.text_input = Some(Box::new(text));
        self
    }

    /// Set input rules by communicating with the system.
    pub fn text_cmd(&mut self, event: TextCmd) {}
}

/// An activatable action.
pub struct Action<T>
where
    T: 'static,
{
    /// Label for the action (only used in overflow hamburger menu).
    label: Option<&'static str>,
    /// Icon image coordinates for the action (used in window header if space is
    /// available).
    icon: Option<(f32, f32, f32, f32)>,
    /// Action on button / menu item press.
    ///
    /// `None` to change nesting level.  If `label` and `icon` are `None`, then
    /// decrease nesting level, otherwise increase.
    action: Option<T>,
}

impl<T> Action<T> {
    /// Create an action to open a branch.
    pub const fn branch(
        label: &'static str,
        icon: Option<(f32, f32, f32, f32)>,
    ) -> Self {
        Self {
            label: Some(label),
            icon,
            action: None,
        }
    }

    /// Create an action to close a branch.
    pub const fn close() -> Self {
        Self {
            label: None,
            icon: None,
            action: None,
        }
    }

    /// Create an action with a label.
    pub const fn label(label: &'static str, action: T) -> Self {
        Self {
            label: Some(label),
            icon: None,
            action: Some(action),
        }
    }

    /// Create an action with an icon.
    pub const fn icon(icon: (f32, f32, f32, f32), action: T) -> Self {
        Self {
            label: None,
            icon: Some(icon),
            action: Some(action),
        }
    }

    /// Create an action with a label and icon.
    pub const fn new(
        label: &'static str,
        icon: (f32, f32, f32, f32),
        action: T,
    ) -> Self {
        Self {
            label: Some(label),
            icon: Some(icon),
            action: Some(action),
        }
    }
}

/// Navigation icon in the upper-left corner of the screen.
enum Nav<T>
where
    T: 'static,
{
    /// Back button.
    Back(T),
    /// Drawer containing a dynamic long tab tree (unlimited).
    Drawer(Vec<Action<T>>),
}

/// A page of an app.  This structure contains the information needed to render
/// the page.
struct Page<T>
where
    T: 'static,
{
    /// Label of the page.
    pub label: Option<String>,
    /// Static buttons available in the window header.  If there is not enough
    /// room, overflow into the pop-up hamburger menu in the upper-right corner
    /// of the screen.
    pub actions: &'static [Action<T>],
    /// The navigation element in the upper left corner of the screen.
    pub nav: Nav<T>,
    /// Static short tab list (up to 5).
    pub pages: &'static [Action<T>],
    /// Status bar (up to 24 monospace-latin-width-characters).
    pub status: Option<String>,
    /// Multimedia document if Some, None is custom canvas.
    pub document: Option<Document>,
}

/// A multimedia document (*.mdoc file).
///
/// Add multimedia document (plain text, pictures, music score, mathematical
/// expressions, multimedia player, multimedia track, buttons, table, list,
/// drop-down, radio select, date select, number select, code, map, canvas).
/// Each multi-media listed has an input field variant.
pub struct Document {
    buffer: Buffer,
    styles: Styles,
}

/// Buffer from a multimedia document.
pub struct Buffer {
    medias: Vec<Media>,
}

/// Document style (fonts, sizes, colors, etc.).
pub struct Styles {}

/// MultiMedia Type.  All of the different types of data that the user can input
/// must be covered by this enum.
pub enum Media {
    /// Single line of plain text, divided into 128-latin-column chunks.
    PlainText(Vec<String>),
    /// Text with headers, strong, emphasis, underline, strikethrough, links,
    /// custom styling.
    FormatText(Vec<String>),
    /// Signed integer.
    IntegerS(i128),
    /// Unsigned integer.
    IntegerU(u128),
    /// Signed Fixed-Point Decimal Value (38-digit precision).
    DecimalS(i128, u128),
    /// Unsigned Fixed-Point Decimal Value (38-digit precision).
    DecimalU(u128, u128),
    /// Image data (possibly animated).
    Image(Raster<SRgba8>),
    /// Audio buffer.
    Audio(),
    /// Video buffer.
    Video(),
    /// Music score.
    Music(),
    /// (Year, Month, DayOfMonth).
    Date(),
    /// (Hour, Minute, Second).
    Time(),
    /// (Year, Month, DayOfMonth, Hour, Minute, Second).
    DateTime(),
    /// Arbitrary duration of time.
    Duration(),
    /// (DayOfWeek)
    Day(),
    /// (WeekOfYear)
    Week(),
    /// Physical location (GPS coordinates)
    Location(),
    /// Geographical Map.
    Map(),
    /// A 3-dimensional model.
    Model(),
}

/// Accessibile User Interface.
pub struct Ui<'a, T>
where
    T: 'static,
{
    state: Page<T>,
    transition: &'a mut dyn FnMut(&mut Ui<T>, T),
}

impl<'a, T> Ui<'a, T> {
    /// Create a new user interface.
    pub fn new(transition: &'a mut dyn FnMut(&mut Ui<T>, T)) -> Self {
        Self {
            state: Page {
                label: Some(String::new()),
                actions: &[],
                nav: Nav::Drawer(Vec::new()),
                pages: &[],
                status: None,
                document: Some(Document {
                    buffer: Buffer { medias: Vec::new() },
                    styles: Styles {},
                }),
            },
            transition,
        }
    }

    /// Transition navigator widget to drawer.
    pub fn drawer<I: Iterator<Item = Action<T>>>(&mut self, tree: I) {
        use Nav::*;
        match self.state.nav {
            Back(_) => {
                self.state.nav = Nav::Drawer(tree.collect());
            }
            Drawer(ref mut drawer) => {
                drawer.clear();
                drawer.extend(tree);
            }
        }
    }

    /// Transition navigator widget to back button.
    pub fn back(&mut self, action: T) {
        self.state.nav = Nav::Back(action);
    }

    /// Disable document to render pixels directly.
    pub fn canvas(&mut self) {
        self.state.document = None;
    }

    /// Change entire document.
    pub fn document(&mut self, document: Document) {
        self.state.document = Some(document);
    }

    /// Replace part of the document with buffer data.
    ///
    /// The first element of the tuple is the index of the media widget.  The
    /// second element is the line, measure number, track index, or X pixel
    /// position.  The third element is the index within the line, measure
    /// number and part, frame index, or Y position.
    pub fn replace(&mut self, range: Range<(u32, u32, u32)>, buffer: Buffer) {}

    /// Transition which action list to use.
    pub fn actions(&mut self, actions: &'static [Action<T>]) {
        self.state.actions = actions;
    }

    /// Static short tab list (up to 5).
    ///
    /// # Panics
    /// If `actions.len()` is greater than 5.
    pub fn pages(&mut self, actions: &'static [Action<T>]) {
        assert!(actions.len() <= 5);
        self.state.pages = actions;
    }

    /// Transition to showing status text.
    pub fn show(&mut self) -> &mut String {
        // FIXME: Animate
        if self.state.status.is_none() {
            self.state.status = Some(String::new());
        }
        self.state.status.as_mut().unwrap()
    }

    /// Transition to hiding status text.
    pub fn hide(&mut self) {
        // FIXME: Animate
        self.state.status = None;
    }
}
