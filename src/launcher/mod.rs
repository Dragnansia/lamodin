use crate::modifier::Modifier;

pub mod lutris;
pub mod steam;

pub trait Launcher {
    fn containt_version(&self, name: &str) -> bool;

    fn modifiers(&self) -> Vec<Modifier>;
}
