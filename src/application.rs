use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::ui::main_window::UpdateCenterWindow;

mod imp {
    use super::*;
    use adw::{Application, ApplicationWindow};
    use gtk::glib;
    use std::cell::OnceCell;

    #[derive(Default)]
    pub struct UpdateCenterApplication {
        pub window: OnceCell<UpdateCenterWindow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UpdateCenterApplication {
        const NAME: &'static str = "UpdateCenterApplication";
        type Type = super::UpdateCenterApplication;
        type ParentType = Application;
    }

    impl ObjectImpl for UpdateCenterApplication {}

    impl ApplicationImpl for UpdateCenterApplication {
        fn activate(&self) {
            let app = self.obj();
            let window = if let Some(window) = self.window.get() {
                window.clone()
            } else {
                let window = UpdateCenterWindow::new(&app);
                self.window.set(window.clone()).expect("Window already set");
                window
            };

            window.present();
        }

        fn startup(&self) {
            self.parent_startup();
            let app = self.obj();
            
            // Set application name
            glib::set_application_name("GNOME Update Center");
            
            // Load CSS
            let provider = gtk::CssProvider::new();
            provider.load_from_data(include_bytes!("../data/style.css"));
            
            gtk::style_context_add_provider_for_display(
                &gtk::gdk::Display::default().expect("Could not connect to display"),
                &provider,
                gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
            );
        }
    }

    impl GtkApplicationImpl for UpdateCenterApplication {}
    impl AdwApplicationImpl for UpdateCenterApplication {}
}

glib::wrapper! {
    pub struct UpdateCenterApplication(ObjectSubclass<imp::UpdateCenterApplication>)
        @extends gio::Application, gtk::Application, adw::Application,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl UpdateCenterApplication {
    pub fn new(application_id: &str, flags: &gio::ApplicationFlags) -> Self {
        glib::Object::builder()
            .property("application-id", application_id)
            .property("flags", flags)
            .build()
    }

    pub fn run(&self) -> glib::ExitCode {
        ApplicationExtManual::run(self)
    }
}