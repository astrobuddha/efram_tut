use egui::Widgets;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum RunMode {
    /// This is the default for the demo.
    ///
    /// If this is selected, egui is only updated if are input events
    /// (like mouse movements) or there are some animations in the GUI.
    ///
    /// Reactive mode saves CPU.
    Ractive,

    /// This will call `egui::Context::request_repaint()` at the end of each frame
    /// to request the backend to repaint as soon as possible.
    ///
    /// On most platforms this will mean that egui will run at the display refresh rate of e.g. 60 Hz.
    ///
    /// For this demo it is not any reason to do so except to
    /// demonstrate how quickly egui runs.
    ///
    /// For games or other interactive apps, this is probably what you want to do.
    /// It will guarantee that egui is always up-to-date.
    Continuous,
}

impl Default for RunMode {
    fn default() -> Self {
        RunMode::Reactive
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct BackendPanel {
    pub open: bool,

    #[cfg_attr(feature = "serde", serde(skip))]
    run_mode: RunMode,

    #[cfg_attr(feature = "serde", serde(skip))]
    repaint_after_seconds: f32,

    #[cfg_attr(feature = "serde", serde(skip))]
    pixels_per_point: Option<f32>,

    #[cfg_attr(feature = "serde", serde(skip))]
    frame_history: crate::frame_history::frame_history,

    egui_windows: EguiWindows,
}

impl Default for BackendPanel {
    fn default() -> Self {
        Self {
            open: false,
            run_mode: Default::default(),
            repaint_after_seconds: 1.0,
            pixels_per_point: None,
            frame_history: Default::default(),
            egui_windows: Default::default(),
        }
    }
}

// todo start implemented the backend panel for the side panel
impl BackendPanel {
    // pub fn update()
}
