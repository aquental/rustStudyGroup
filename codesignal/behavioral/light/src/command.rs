use crate::light::Light;

pub trait Command {
    fn execute(&self, light: &mut Light);
}
