use super::Server;

pub struct Application;

impl Server for Application {
    fn handle_request(&mut self, url: &str, method: &str) -> (u16, String) {
        if url == "/app/status" && method == "GET" {
            return (200, "Ok".into());
        }
        if url == "/create/user" && method == "POST" {
            return (201, "User Creadted".into());
        }
        (404, String::from("Not Ok"))
    }
}
