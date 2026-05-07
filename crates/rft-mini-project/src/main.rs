mod application;
mod entities;

fn main() {
    let mut app = application::Application::new();
    
    app.run();
}
