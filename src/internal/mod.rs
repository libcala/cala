static START: std::sync::Once = std::sync::ONCE_INIT;

// // // // // //

/// Initialize Cala.
pub fn init() {
    START.call_once(|| {
        #[cfg(feature = "user")]
        {
            // Initialize user data.
            crate::user::initialize_user_io();
        }
        #[cfg(feature = "controller")]
        {
            // Initialize controller port data.
            crate::controller::initialize_controller_io();
        }
        #[cfg(feature = "audio")]
        {
            // Intialize audio interface.
            crate::audio::initialize_audio_io();
        }
    });
}
