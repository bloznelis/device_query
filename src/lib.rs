pub mod keymap;

#[cfg(target_os = "linux")]
pub mod linux;
#[cfg(target_os = "linux")]
pub use linux::MouseCoords;

#[cfg(target_os = "windows")]
pub mod windows;
#[cfg(target_os = "windows")]
pub use windows::MouseCoords;

pub trait MouseQuery {
    fn get_coords(&self) -> (i32, i32);
    fn get_keys(&self) -> ();
}

impl MouseQuery for MouseCoords {
    fn get_coords(&self) -> (i32, i32) {
        self.query_pointer()
    }

    fn get_keys(&self) -> () {
        self.query_keymap();
        ()
    }
}
