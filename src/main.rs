mod application;
mod ui;
mod services;
mod config;

use application::UpdateCenterApplication;
use gtk::{gio, glib};

static APP_ID: &str = "org.gnome.UpdateCenter";

fn main() -> glib::ExitCode {
    // Initialize logger for debug information
    pretty_env_logger::init();
    
    // Initialize GTK and create application
    gtk::init().expect("Failed to initialize GTK");
    let app = UpdateCenterApplication::new(APP_ID, &gio::ApplicationFlags::empty());
    
    // Run the application
    app.run()
}