mod keybind;
mod scratchpad;
mod workspace_config;

use crate::layouts::Layout;
pub use crate::models::{FocusBehaviour, Gutter, Margins, Size};
use crate::models::{LayoutMode, Manager};
use crate::state::State;
pub use keybind::Keybind;
pub use scratchpad::ScratchPad;
pub use workspace_config::Workspace;

pub trait Config {
    /// Returns a collection of bindings with the mod key mapped.
    fn mapped_bindings(&self) -> Vec<Keybind>;

    fn create_list_of_tag_labels(&self) -> Vec<String>;

    fn workspaces(&self) -> Option<Vec<Workspace>>;

    fn focus_behaviour(&self) -> FocusBehaviour;

    fn mousekey(&self) -> String;

    //of you are on tag "1" and you goto tag "1" this takes you to the previous tag
    fn disable_current_tag_swap(&self) -> bool;

    fn create_list_of_scratchpads(&self) -> Vec<ScratchPad>;

    fn layouts(&self) -> Vec<Layout>;

    fn layout_mode(&self) -> LayoutMode;

    fn focus_new_windows(&self) -> bool;

    fn command_handler<SERVER>(command: &str, manager: &mut Manager<Self, SERVER>) -> bool
    where
        Self: Sized;

    fn always_float(&self) -> bool;
    fn default_width(&self) -> i32;
    fn default_height(&self) -> i32;
    fn border_width(&self) -> i32;
    fn margin(&self) -> Margins;
    fn workspace_margin(&self) -> Option<Margins>;
    fn gutter(&self) -> Option<Vec<Gutter>>;
    fn default_border_color(&self) -> String;
    fn floating_border_color(&self) -> String;
    fn focused_border_color(&self) -> String;
    fn on_new_window_cmd(&self) -> Option<String>;
    fn get_list_of_gutters(&self) -> Vec<Gutter>;
    fn max_window_width(&self) -> Option<Size>;

    /// Attempt to write current state to a file.
    ///
    /// It will be used to restore the state after soft reload.
    ///
    /// **Note:** this function cannot fail.
    fn save_state(&self, state: &State);

    /// Load saved state if it exists.
    fn load_state(&self, state: &mut State);
}

#[cfg(test)]
#[allow(clippy::module_name_repetitions)]
pub struct TestConfig {
    pub tags: Vec<String>,
}

#[cfg(test)]
impl Config for TestConfig {
    fn mapped_bindings(&self) -> Vec<Keybind> {
        unimplemented!()
    }
    fn create_list_of_tag_labels(&self) -> Vec<String> {
        self.tags.clone()
    }
    fn workspaces(&self) -> Option<Vec<Workspace>> {
        unimplemented!()
    }
    fn focus_behaviour(&self) -> FocusBehaviour {
        FocusBehaviour::Sloppy
    }
    fn mousekey(&self) -> String {
        "Mod4".to_string()
    }
    fn disable_current_tag_swap(&self) -> bool {
        false
    }
    fn create_list_of_scratchpads(&self) -> Vec<ScratchPad> {
        vec![]
    }
    fn layouts(&self) -> Vec<Layout> {
        vec![]
    }
    fn layout_mode(&self) -> LayoutMode {
        LayoutMode::Workspace
    }
    fn focus_new_windows(&self) -> bool {
        false
    }
    fn command_handler<SERVER>(_command: &str, _manager: &mut Manager<Self, SERVER>) -> bool {
        unimplemented!()
    }
    fn always_float(&self) -> bool {
        false
    }
    fn default_width(&self) -> i32 {
        1000
    }
    fn default_height(&self) -> i32 {
        800
    }
    fn border_width(&self) -> i32 {
        0
    }
    fn margin(&self) -> Margins {
        Margins::new(0)
    }
    fn workspace_margin(&self) -> Option<Margins> {
        None
    }
    fn gutter(&self) -> Option<Vec<Gutter>> {
        unimplemented!()
    }
    fn default_border_color(&self) -> String {
        unimplemented!()
    }
    fn floating_border_color(&self) -> String {
        unimplemented!()
    }
    fn focused_border_color(&self) -> String {
        unimplemented!()
    }
    fn on_new_window_cmd(&self) -> Option<String> {
        None
    }
    fn get_list_of_gutters(&self) -> Vec<Gutter> {
        Default::default()
    }
    fn max_window_width(&self) -> Option<Size> {
        None
    }
    fn save_state(&self, _state: &State) {
        unimplemented!()
    }
    fn load_state(&self, _state: &mut State) {
        unimplemented!()
    }
}
