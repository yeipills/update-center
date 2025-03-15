use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::services::drivers::DriversService;

mod imp {
    use super::*;
    use gtk::glib;
    use gtk::{Box, Button, Label, ListBox, ScrolledWindow, Spinner};
    use std::cell::{RefCell, Cell};

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/UpdateCenter/ui/drivers_view.ui")]
    pub struct DriversView {
        #[template_child]
        pub drivers_list: TemplateChild<ListBox>,
        
        #[template_child]
        pub detect_button: TemplateChild<Button>,
        
        #[template_child]
        pub install_button: TemplateChild<Button>,
        
        #[template_child]
        pub status_label: TemplateChild<Label>,
        
        #[template_child]
        pub spinner: TemplateChild<Spinner>,
        
        pub drivers_service: RefCell<Option<DriversService>>,
        pub is_checking: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for DriversView {
        const NAME: &'static str = "DriversView";
        type Type = super::DriversView;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for DriversView {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            
            // Initialize drivers service
            *self.drivers_service.borrow_mut() = Some(DriversService::new());
            
            // Connect signals
            self.detect_button.connect_clicked(glib::clone!(@weak obj => move |_| {
                obj.detect_drivers();
            }));
            
            self.install_button.connect_clicked(glib::clone!(@weak obj => move |_| {
                obj.install_drivers();
            }));
            
            // Initial state
            self.install_button.set_sensitive(false);
            self.status_label.set_text("Buscar controladores disponibles para el hardware");
        }
    }

    impl BoxImpl for DriversView {}
    impl WidgetImpl for DriversView {}
}

glib::wrapper! {
    pub struct DriversView(ObjectSubclass<imp::DriversView>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl DriversView {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }
    
    pub fn detect_drivers(&self) {
        let imp = self.imp();
        
        if imp.is_checking.get() {
            return;
        }
        
        imp.is_checking.set(true);
        imp.spinner.set_visible(true);
        imp.spinner.start();
        imp.status_label.set_text("Detectando hardware y controladores disponibles...");
        imp.detect_button.set_sensitive(false);
        
        // Clear existing driver list
        while let Some(child) = imp.drivers_list.first_child() {
            imp.drivers_list.remove(&child);
        }
        
        // Simulate driver detection
        let service = imp.drivers_service.borrow();
        if let Some(_service) = service.as_ref() {
            glib::timeout_add_seconds_local(
                2,
                glib::clone!(@weak self as view => @default-return glib::Continue(false), move || {
                    view.imp().is_checking.set(false);
                    view.imp().spinner.stop();
                    view.imp().spinner.set_visible(false);
                    view.imp().detect_button.set_sensitive(true);
                    
                    // Sample drivers for demonstration
                    let drivers = vec![
                        ("NVIDIA GeForce RTX 3060", "nvidia-driver-535", "Controlador propietario NVIDIA", "Rendimiento 3D completo, CUDA, ray tracing", true),
                        ("NVIDIA GeForce RTX 3060", "nouveau", "Controlador de código abierto", "Soporte básico, sin aceleración 3D completa", false),
                        ("Broadcom BCM43142", "broadcom-wl", "Controlador privativo Broadcom", "Soporte completo WiFi y Bluetooth", true),
                    ];
                    
                    if drivers.is_empty() {
                        view.imp().status_label.set_text("No se encontraron controladores adicionales disponibles");
                        view.imp().install_button.set_sensitive(false);
                    } else {
                        let recommended_count = drivers.iter().filter(|d| d.4).count();
                        
                        view.imp().status_label.set_markup(&format!(
                            "<b>{}</b> controladores recomendados disponibles", 
                            recommended_count
                        ));
                        view.imp().install_button.set_sensitive(recommended_count > 0);
                        
                        // Add drivers to list
                        for (device, name, description, features, recommended) in drivers {
                            let row = adw::ActionRow::builder()
                                .title(name)
                                .subtitle(device)
                                .build();
                                
                            // Description label
                            let desc_label = gtk::Label::new(Some(description));
                            desc_label.add_css_class("caption");
                            row.add_suffix(&desc_label);
                            
                            // Features label below
                            let feat_label = gtk::Label::new(Some(features));
                            feat_label.add_css_class("dim-label");
                            feat_label.add_css_class("caption");
                            feat_label.set_wrap(true);
                            feat_label.set_xalign(1.0);
                            row.add_suffix(&feat_label);
                            
                            // Radio button
                            let radio = gtk::CheckButton::new();
                            radio.set_active(recommended);
                            row.add_prefix(&radio);
                            
                            // Add recommended badge if applicable
                            if recommended {
                                let badge = gtk::Label::new(Some("Recomendado"));
                                badge.add_css_class("accent");
                                badge.add_css_class("caption-heading");
                                row.add_suffix(&badge);
                            }
                            
                            view.imp().drivers_list.append(&row);
                        }
                    }
                    
                    glib::Continue(false)
                }),
            );
        }
    }
    
    pub fn install_drivers(&self) {
        let imp = self.imp();
        
        imp.status_label.set_text("Instalando controladores...");
        imp.spinner.set_visible(true);
        imp.spinner.start();
        imp.detect_button.set_sensitive(false);
        imp.install_button.set_sensitive(false);
        
        // Simulate driver installation
        glib::timeout_add_seconds_local(
            5,
            glib::clone!(@weak self as view => @default-return glib::Continue(false), move || {
                view.imp().spinner.stop();
                view.imp().spinner.set_visible(false);
                view.imp().status_label.set_text("Controladores instalados correctamente. Se recomienda reiniciar el sistema.");
                view.imp().detect_button.set_sensitive(true);
                
                // Update rows to show installed status
                if let Some(list) = view.imp().drivers_list.first_child() {
                    let row = list.downcast_ref::<adw::ActionRow>().unwrap();
                    
                    // Find the badge or add a new one
                    let mut found = false;
                    for child in row.observe_children().snapshot().iter() {
                        if let Some(label) = child.downcast_ref::<gtk::Label>() {
                            if label.text() == "Recomendado" {
                                label.set_text("Instalado");
                                label.remove_css_class("accent");
                                label.add_css_class("success");
                                found = true;
                                break;
                            }
                        }
                    }
                    
                    if !found {
                        let badge = gtk::Label::new(Some("Instalado"));
                        badge.add_css_class("success");
                        badge.add_css_class("caption-heading");
                        row.add_suffix(&badge);
                    }
                }
                
                glib::Continue(false)
            }),
        );
    }
}