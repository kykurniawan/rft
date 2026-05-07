mod application;
mod entities;
mod services;

fn main() {
    let mut app = application::Application::new();

    app.run();
}
