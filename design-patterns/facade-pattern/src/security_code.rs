pub struct SecurityCode {
    code: u32,
}

impl SecurityCode {
    pub fn new(code: u32) -> Self {
        Self { code }
    }

    pub fn check(&self, code: u32) -> Result<(), String> {
        if self.code != code {
            return Err("Security code is incorect".into());
        }

        println!("Security code verified");

        Ok(())
    }
}
