//! Configure your application.
use crate::window;

/// The settings of an application.
#[derive(Debug, Clone)]
pub struct Settings<Flags> {
    /// The window settings.
    ///
    /// They will be ignored on the Web.
    pub window: window::Settings,

    /// The data needed to initialize an [`Application`].
    ///
    /// [`Application`]: crate::Application
    pub flags: Flags,

    /// Renderer specific settings
    pub renderer: crate::renderer::Settings,

}

impl<Flags> Settings<Flags> {
    /// Initialize [`Application`] settings using the given data.
    ///
    /// [`Application`]: crate::Application
    pub fn with_flags(flags: Flags) -> Self {
        let default_settings = Settings::<()>::default();

        Self {
            flags,
            window: default_settings.window,
            renderer: default_settings.renderer,
        }
    }
}

impl<Flags> Default for Settings<Flags>
    where
        Flags: Default,
{
    fn default() -> Self {
        Self {
            flags: Default::default(),
            window: Default::default(),
            renderer: Default::default(),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<Flags> From<Settings<Flags>> for iced_winit::Settings<Flags> {
    fn from(settings: Settings<Flags>) -> iced_winit::Settings<Flags> {
        iced_winit::Settings {
            window: settings.window.into(),
            flags: settings.flags,
        }
    }
}
