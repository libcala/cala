/// The Application State.
pub struct App<T> {
    // Store
    file: T,
    changed: bool,
}

impl<T> App<T> {
    /// Create a new Application.
    pub fn new(file: T) -> App<T> {
        App {
            file,
            changed: false,
        }
    }

    /// Get the file data.
    pub fn file(&mut self) -> &mut T {
        &mut self.file
    }

    /// Fetch a resource.
    pub fn fetch<U>(&mut self, res: &str) -> Option<U>
    where
        for<'de> U: serde::Deserialize<'de>,
    {
        stronghold::fetch(res)
    }

    /// Load file `res` from `zip`.
    pub fn open(&mut self, zip: &str, res: &str) -> Option<()>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        self.file = stronghold::load(zip, res)?;
        Some(())
    }

    /// File data has changed.  Next call to `sync()` will save the file.
    pub fn edit(&mut self) {
        self.changed = true;
    }

    /// Save file `res` in `zip` only if `edit()` has been called since last change.
    pub fn sync(&mut self, zip: &str, res: &str)
    where
        T: serde::Serialize,
    {
        if self.changed {
            stronghold::save(zip, res, &self.file);
            self.changed = false;
        }
    }
}
