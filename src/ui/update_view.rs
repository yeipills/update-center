use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::services::package::PackageService;

mod imp {
    use super::*;
    use gtk::glib;
    use gtk::{Box, Button, Label, ListBox, ScrolledWindow, Spinner};
    use std::cell::{RefCell, Cell};

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/UpdateCenter/ui/update_view.ui")]
    pub struct UpdateView {
        #[template_child]
        pub updates_list: TemplateChild<ListBox>,
        
        #[template_child]
        pub check_button: TemplateChild<Button>,
        
        #[template_child]
        pub apply_button: TemplateChild<Button>,
        
        #[template_child]
        pub status_label: TemplateChild<Label>,
        
        #[template_child]
        pub spinner: TemplateChild<Spinner>,
        
        pub package_service: RefCell<Option<PackageService>>,
        pub is_checking: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for UpdateView {
        const NAME: &'static str = "UpdateView";
        type Type = super::UpdateView;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for UpdateView {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            
            // Initialize package service
            *self.package_service.borrow_mut() = Some(PackageService::new());
            
            // Connect signals
            self.check_button.connect_clicked(glib::clone!(@weak obj => move |_| {
                obj.check_for_updates();
            }));
            
            self.apply_button.connect_clicked(glib::clone!(@weak obj => move |_| {
                obj.apply_updates();
            }));
            
            // Initial state
            self.apply_button.set_sensitive(false);
            self.status_label.set_text("Sistema listo para comprobar actualizaciones");
        }
    }

    impl BoxImpl for UpdateView {}
    impl WidgetImpl for UpdateView {}
}

glib::wrapper! {
    pub struct UpdateView(ObjectSubclass<imp::UpdateView>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl UpdateView {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
    
    pub fn check_for_updates(&self) {
        let imp = self.imp();
        
        if imp.is_checking.get() {
            return;
        }
        
        imp.is_checking.set(true);
        imp.spinner.set_visible(true);
        imp.spinner.start();
        imp.status_label.set_text("Comprobando actualizaciones...");
        imp.check_button.set_sensitive(false);
        
        // Clear existing updates
        while let Some(child) = imp.updates_list.first_child() {
            imp.updates_list.remove(&child);
        }
        
        // Simulate update check (would be async with actual PackageKit)
        let service = imp.package_service.borrow();
        if let Some(service) = service.as_ref() {
            glib::timeout_add_seconds_local(
                2,
                glib::clone!(@weak self as view => @default-return glib::Continue(false), move || {
                    view.imp().is_checking.set(false);
                    view.imp().spinner.stop();
                    view.imp().spinner.set_visible(false);
                    view.imp().check_button.set_sensitive(true);
                    
                    // Simulated updates for demonstration
                    let updates = vec![
                        ("firefox", "115.0", "Navegador web Firefox"),
                        ("gnome-shell", "44.2", "Shell de GNOME"),
                        ("kernel", "6.4.0", "Núcleo del sistema operativo"),
                    ];
                    
                    if updates.is_empty() {
                        view.imp().status_label.set_text("El sistema está actualizado");
                        view.imp().apply_button.set_sensitive(false);
                    } else {
                        view.imp().status_label.set_markup(&format!(
                            "<b>{}</b> actualizaciones disponibles", 
                            updates.len()
                        ));
                        view.imp().apply_button.set_sensitive(true);
                        
                        // Add updates to list
                        for (name, version, description) in updates {
                            let row = adw::ActionRow::builder()
                                .title(name)
                                .subtitle(&format!("Versión: {}", version))
                                .build();
                                
                            let description_label = gtk::Label::new(Some(description));
                            description_label.add_css_class("dim-label");
                            row.add_suffix(&description_label);
                            
                            let check = gtk::CheckButton::new();
                            check.set_active(true);
                            row.add_prefix(&check);
                            
                            view.imp().updates_list.append(&row);
                        }
                    }
                    
                    glib::Continue(false)
                }),
            );
        }
    }
    
    pub fn apply_updates(&self) {
        let imp = self.imp();
        
        imp.status_label.set_text("Aplicando actualizaciones...");
        imp.spinner.set_visible(true);
        imp.spinner.start();
        imp.check_button.set_sensitive(false);
        imp.apply_button.set_sensitive(false);
        
        // Simulate update application
        glib::timeout_add_seconds_local(
            3,
            glib::clone!(@weak self as view => @default-return glib::Continue(false), move || {
                view.imp().spinner.stop();
                view.imp().spinner.set_visible(false);
                view.imp().status_label.set_text("Actualizaciones aplicadas correctamente");
                view.imp().check_button.set_sensitive(true);
                
                // Clear the list
                while let Some(child) = view.imp().updates_list.first_child() {
                    view.imp().updates_list.remove(&child);
                }
                
                glib::Continue(false)
            }),
        );
    }
}