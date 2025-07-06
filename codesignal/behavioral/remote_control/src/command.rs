use crate::fan::Fan;

pub trait Command {
    fn execute(&self, fan: &mut Fan);
}
