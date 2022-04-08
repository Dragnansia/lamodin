use crate::modifier::Modifier;

pub mod lutris;
pub mod steam;

/// Basic function need by all `Launcher`
///
/// # Example
/// ```
/// use lamodin::{launcher::Launcher, modifier::Modifier};
///
/// struct WineLauncher;
///
/// impl Launcher for WineLauncher {
///     fn containt_version(&self, name: &str) -> bool {
///         true
///     }
///
///     fn modifiers(&self) -> Vec<Modifier> {
///         vec![]
///     }
/// }
/// ```
pub trait Launcher {
    fn containt_version(&self, name: &str) -> Option<Modifier>;

    fn modifiers(&self) -> Vec<Modifier>;
}
