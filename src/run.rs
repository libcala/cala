/// Cala program control flow loop.
pub enum Loop<T> {
    /// Exit the program.
    ///
    /// # Usage
    /// ```
    /// // Set the home loop to `do_nothing()`.
    /// cala::init!(do_nothing, ());
    ///  
    /// // This program just quits right away.
    /// pub fn do_nothing(_: &mut ()) -> cala::Loop<()> {
    ///     // Exit.
    ///     cala::Exit
    /// }
    /// ```
    Exit,
    /// Keep the program running.
    ///
    /// # Usage
    /// ```
    /// // Set the home loop to `infinite_loop()`.
    /// cala::init!(infinite_loop, ());
    ///  
    /// // This program runs until the user interupts it.
    /// pub fn infinite_loop(_: &mut ()) -> cala::Loop<()> {
    ///     // Run this function again.
    ///     cala::Continue
    /// }
    /// ```
    Continue,
    /// Go back one activity.
    ///
    /// # Usage
    /// ```
    /// // Set the home loop to `home()`.
    /// cala::init!(home, ());
    ///  
    /// // This program creates 2 new activities, which both go back right away.
    /// pub fn home(_: &mut ()) -> cala::Loop<()> {
    ///     // `activity_a` is first, because it will be on the top of the activity stack.
    ///     cala::ReplaceWithBack(activity_a, activity_b)
    /// }
    ///
    /// pub fn activity_a(_: &mut ()) -> cala::Loop<()> {
    ///     println!("First");
    ///     // Go to activity_b
    ///     cala::Back
    /// }
    ///
    /// pub fn activity_b(_: &mut ()) -> cala::Loop<()> {
    ///     println!("Last");
    ///     // Quit
    ///     cala::Back
    /// }
    /// ```
    Back,
    /// Add new activities to the activity stack (new activity loop).
    ///
    /// # Usage
    /// ```
    /// // Set the home loop to `home()`.
    /// cala::init!(home, ());
    ///  
    /// // Infinite loop.
    /// pub fn home(_: &mut ()) -> cala::Loop<()> {
    ///     // `activity_a` is first, because it will be on the top of the activity stack.
    ///     cala::Append(activity_a)
    /// }
    ///
    /// pub fn activity_a(_: &mut ()) -> cala::Loop<()> {
    ///     // Go to activity `home()`
    ///     cala::Back
    /// }
    /// ```
    Append(fn(&mut T) -> Loop<T>),
    /// Add a new activity to the activity stack (new activity loop), throwing away the
    /// one on top (the current activity).
    ///
    /// # Usage
    /// ```
    /// // Set the home loop to `home()`.
    /// cala::init!(home, ());
    ///  
    /// // This program creates 1 new activity, which quits right away.
    /// pub fn home(_: &mut ()) -> cala::Loop<()> {
    ///     // Replace our `home()` activity with `activity_a()`.
    ///     cala::Replace(activity_a)
    /// }
    ///
    /// pub fn activity_a(_: &mut ()) -> cala::Loop<()> {
    ///     // Quit
    ///     cala::Exit
    /// }
    /// ```
    Replace(fn(&mut T) -> Loop<T>),
    /// Add 2 new activities to the activity stack (new activity loop), throwing away the
    /// one on top (the current activity).  First parameter is the activity that the user
    /// will enter, and the second is the one they will enter when they go back.
    ///
    /// # Usage
    /// ```
    /// // Set the home loop to `home()`.
    /// cala::init!(home, ());
    ///  
    /// // This program creates 2 new activities, which both go back right away.
    /// pub fn home(_: &mut ()) -> cala::Loop<()> {
    ///     // `activity_a` is first, because it will be on the top of the activity stack.
    ///     cala::ReplaceWithBack(activity_a, activity_b)
    /// }
    ///
    /// pub fn activity_a(_: &mut ()) -> cala::Loop<()> {
    ///     println!("First");
    ///     // Go to activity_b
    ///     cala::Back
    /// }
    ///
    /// pub fn activity_b(_: &mut ()) -> cala::Loop<()> {
    ///     println!("Last");
    ///     // Quit
    ///     cala::Back
    /// }
    /// ```
    ReplaceWithBack(fn(&mut T) -> Loop<T>, fn(&mut T) -> Loop<T>),
}
