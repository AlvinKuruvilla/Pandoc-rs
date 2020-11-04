#[derive(Debug, Clone, Default)]
pub struct Settings {
    pandocExec: String
}
impl Settings {
    pub fn get_pando_exec(self) -> String {
        return self.pandocExec.clone();
    }
    pub fn set_pando_exec(mut self, p_exec: &str) {
        self.pandocExec = p_exec.to_string()

    }
}