pub trait HttpRequestExtend {
    fn set_remote_addr(&mut self, addr: &str);
    fn get_remote_addr(&self) -> String;
}
