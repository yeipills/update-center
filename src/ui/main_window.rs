use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::ui::update_view::UpdateView;
use crate::ui::firmware_view::FirmwareView;
use crate::ui::drivers_view::DriversView;
use crate::ui::settings_view::SettingsView;

mod imp {
    use super::*;
    use adw::{ApplicationWindow, NavigationView, NavigationPage};
    use gtk::glib;
    use std::cell::RefCell;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/UpdateCenter/ui/main_window.ui")]
    pub struct UpdateCenterWindow {
        #[template_child]
        pub navigation_view: TemplateChild<NavigationView>,
        
        #[template_child]
        pub dashboard_page: TemplateChild<NavigationPage>,
        
        pub update_view: RefCell<Option<UpdateView>>,
        pub firmware_view: RefCell<Option<FirmwareView>>,
        pub drivers_view: RefCell<Option<DriversView>>,
        pub settings_view: RefCell<Option<SettingsView>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UpdateCenterWindow {
        const NAME: &'static str = "UpdateCenterWindow";
        type Type = super::UpdateCenterWindow;
        type ParentType = ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for UpdateCenterWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            
            // Setup window
            obj.set_title(Some("GNOME Update Center"));
            obj.set_default_size(900, 600);
            
            // Initialize views
            let update_view = UpdateView::new();
            let firmware_view = FirmwareView::new();
            let drivers_view = DriversView::new();
            let settings_view = SettingsView::new();
            
            // Store views
            *self.update_view.borrow_mut() = Some(update_view.clone());
            *self.firmware_view.borrow_mut() = Some(firmware_view.clone());
            *self.drivers_view.borrow_mut() = Some(drivers_view.clone());
            *self.settings_view.borrow_mut() = Some(settings_view.clone());
            
            // Setup navigation
            self.setup_navigation();
        }
    }

    impl ApplicationWindowImpl for UpdateCenterWindow {}
    impl AdwApplicationWindowImpl for UpdateCenterWindow {}
    impl WindowImpl for UpdateCenterWindow {}
    impl WidgetImpl for UpdateCenterWindow {}

    impl UpdateCenterWindow {
        fn setup_navigation(&self) {
            let update_view = self.update_view.borrow();
            let firmware_view = self.firmware_view.borrow();
            let drivers_view = self.drivers_view.borrow();
            let settings_view = self.settings_view.borrow();
            
            // Create navigation pages
            let update_page = NavigationPage::builder()
                .title("Actualizaciones")
                .child(&*update_view.as_ref().unwrap())
                .build();
                
            let firmware_page = NavigationPage::builder()
                .title("Firmware")
                .child(&*firmware_view.as_ref().unwrap())
                .build();
                
            let drivers_page = NavigationPage::builder()
                .title("Controladores")
                .child(&*drivers_view.as_ref().unwrap())
                .build();
                
            let settings_page = NavigationPage::builder()
                .title("Configuración")
                .child(&*settings_view.as_ref().unwrap())
                .build();
            
            // Add pages to navigation view
            self.navigation_view.add(&update_page);
            self.navigation_view.add(&firmware_page);
            self.navigation_view.add(&drivers_page);
            self.navigation_view.add(&settings_page);
        }
    }
}

glib::wrapper! {
    pub struct UpdateCenterWindow(ObjectSubclass<imp::UpdateCenterWindow>)
        @extends adw::ApplicationWindow, gtk::ApplicationWindow, gtk::Window, gtk::Widget,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl UpdateCenterWindow {
    pub fn new(app: &impl IsA<gtk::Application>) -> Self {
        glib::Object::builder()
            .property("application", app)
            .build()
    }
    
    pub fn check_for_updates(&self) {
        let update_view = self.imp().update_view.borrow();
        if let Some(view) = update_view.as_ref() {
            view.check_for_updates();
        }
    }
}