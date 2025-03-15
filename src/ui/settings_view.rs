use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::services::repositories::RepositoriesService;

mod imp {
    use super::*;
    use gtk::glib;
    use gtk::{Box, Button, Label, ListBox, ScrolledWindow, Switch};
    use std::cell::RefCell;

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/UpdateCenter/ui/settings_view.ui")]
    pub struct SettingsView {
        #[template_child]
        pub repositories_list: TemplateChild<ListBox>,
        
        #[template_child]
        pub auto_check_switch: TemplateChild<Switch>,
        
        #[template_child]
        pub auto_download_switch: TemplateChild<Switch>,
        
        #[template_child]
        pub security_only_switch: TemplateChild<Switch>,
        
        pub repositories_service: RefCell<Option<RepositoriesService>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for SettingsView {
        const NAME: &'static str = "SettingsView";
        type Type = super::SettingsView;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for SettingsView {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            
            // Initialize repositories service
            *self.repositories_service.borrow_mut() = Some(RepositoriesService::new());
            
            // Connect signals
            self.auto_check_switch.connect_state_set(glib::clone!(@weak obj => @default-return glib::signal::Inhibit(false), move |_, state| {
                obj.set_auto_check(state);
                glib::signal::Inhibit(false)
            }));
            
            self.auto_download_switch.connect_state_set(glib::clone!(@weak obj => @default-return glib::signal::Inhibit(false), move |_, state| {
                obj.set_auto_download(state);
                glib::signal::Inhibit(false)
            }));
            
            self.security_only_switch.connect_state_set(glib::clone!(@weak obj => @default-return glib::signal::Inhibit(false), move |_, state| {
                obj.set_security_only(state);
                glib::signal::Inhibit(false)
            }));
            
            // Initial setup
            self.load_repositories();
            self.load_settings();
        }
    }

    impl BoxImpl for SettingsView {}
    impl WidgetImpl for SettingsView {}
    
    impl SettingsView {
        fn load_repositories(&self) {
            // Sample repositories for demonstration
            let repositories = vec![
                ("Ubuntu Main Repository", "Paquetes oficiales de Ubuntu", true),
                ("Ubuntu Updates", "Actualizaciones para paquetes oficiales", true),
                ("Ubuntu Security", "Actualizaciones de seguridad", true), 
                ("Canonical Partners", "Software de partners de Canonical", false),
                ("Ubuntu Backports", "Paquetes nuevos para versiones antiguas", false),
            ];
            
            // Add repositories to list
            for (name, description, enabled) in repositories {
                let row = adw::ActionRow::builder()
                    .title(name)
                    .subtitle(description)
                    .build();
                    
                let toggle = gtk::Switch::new();
                toggle.set_active(enabled);
                toggle.set_valign(gtk::Align::Center);
                row.add_suffix(&toggle);
                
                self.repositories_list.append(&row);
            }
        }
        
        fn load_settings(&self) {
            // Set default values
            self.auto_check_switch.set_active(true);
            self.auto_download_switch.set_active(false);
            self.security_only_switch.set_active(true);
        }
    }
}

glib::wrapper! {
    pub struct SettingsView(ObjectSubclass<imp::SettingsView>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl SettingsView {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
    
    pub fn set_auto_check(&self, enabled: bool) {
        let imp = self.imp();
        
        // Enable/disable auto download switch based on auto check
        imp.auto_download_switch.set_sensitive(enabled);
        
        if !enabled {
            imp.auto_download_switch.set_active(false);
        }
        
        // In a real app, save this setting to configuration
        println!("Setting auto-check: {}", enabled);
    }
    
    pub fn set_auto_download(&self, enabled: bool) {
        // In a real app, save this setting to configuration
        println!("Setting auto-download: {}", enabled);
    }
    
    pub fn set_security_only(&self, enabled: bool) {
        // In a real app, save this setting to configuration
        println!("Setting security-only: {}", enabled);
    }
}