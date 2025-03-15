use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

use crate::services::firmware::FirmwareService;

mod imp {
    use super::*;
    use gtk::glib;
    use gtk::{Box, Button, Label, ListBox, ScrolledWindow, Spinner};
    use std::cell::{RefCell, Cell};

    #[derive(Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/UpdateCenter/ui/firmware_view.ui")]
    pub struct FirmwareView {
        #[template_child]
        pub devices_list: TemplateChild<ListBox>,
        
        #[template_child]
        pub check_button: TemplateChild<Button>,
        
        #[template_child]
        pub update_button: TemplateChild<Button>,
        
        #[template_child]
        pub status_label: TemplateChild<Label>,
        
        #[template_child]
        pub spinner: TemplateChild<Spinner>,
        
        pub firmware_service: RefCell<Option<FirmwareService>>,
        pub is_checking: Cell<bool>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for FirmwareView {
        const NAME: &'static str = "FirmwareView";
        type Type = super::FirmwareView;
        type ParentType = Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for FirmwareView {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            
            // Initialize firmware service
            *self.firmware_service.borrow_mut() = Some(FirmwareService::new());
            
            // Connect signals
            self.check_button.connect_clicked(glib::clone!(@weak obj => move |_| {
                obj.check_for_updates();
            }));
            
            self.update_button.connect_clicked(glib::clone!(@weak obj => move |_| {
                obj.update_firmware();
            }));
            
            // Initial state
            self.update_button.set_sensitive(false);
            self.status_label.set_text("Buscar actualizaciones de firmware");
        }
    }

    impl BoxImpl for FirmwareView {}
    impl WidgetImpl for FirmwareView {}
}

glib::wrapper! {
    pub struct FirmwareView(ObjectSubclass<imp::FirmwareView>)
        @extends gtk::Box, gtk::Widget,
        @implements gtk::Accessible, gtk::Buildable, gtk::ConstraintTarget;
}

impl FirmwareView {
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
        imp.status_label.set_text("Buscando dispositivos con firmware actualizable...");
        imp.check_button.set_sensitive(false);
        
        // Clear existing device list
        while let Some(child) = imp.devices_list.first_child() {
            imp.devices_list.remove(&child);
        }
        
        // Simulate firmware check
        let service = imp.firmware_service.borrow();
        if let Some(_service) = service.as_ref() {
            glib::timeout_add_seconds_local(
                2,
                glib::clone!(@weak self as view => @default-return glib::Continue(false), move || {
                    view.imp().is_checking.set(false);
                    view.imp().spinner.stop();
                    view.imp().spinner.set_visible(false);
                    view.imp().check_button.set_sensitive(true);
                    
                    // Sample devices for demonstration
                    let devices = vec![
                        ("BIOS/UEFI", "Dell Inc.", "XPS 13 9370", "1.6.0", "1.8.0", true),
                        ("Touchpad", "Synaptics", "PS/2 Touchpad", "10.2", "11.0", false),
                        ("Bluetooth", "Intel", "AX200 Bluetooth", "22.30", "23.10", true),
                    ];
                    
                    if devices.is_empty() {
                        view.imp().status_label.set_text("No se encontraron dispositivos con firmware actualizable");
                        view.imp().update_button.set_sensitive(false);
                    } else {
                        let updatable_count = devices.iter().filter(|d| d.5).count();
                        
                        if updatable_count > 0 {
                            view.imp().status_label.set_markup(&format!(
                                "<b>{}</b> dispositivos con actualizaciones disponibles", 
                                updatable_count
                            ));
                            view.imp().update_button.set_sensitive(true);
                        } else {
                            view.imp().status_label.set_text("Todos los dispositivos tienen el firmware actualizado");
                            view.imp().update_button.set_sensitive(false);
                        }
                        
                        // Add devices to list
                        for (kind, vendor, model, current, available, updatable) in devices {
                            let row = adw::ActionRow::builder()
                                .title(&format!("{} - {}", kind, model))
                                .subtitle(&format!("Fabricante: {}", vendor))
                                .build();
                                
                            let version_label = gtk::Label::new(Some(&format!("v{} → v{}", current, available)));
                            if updatable {
                                version_label.add_css_class("accent");
                                version_label.add_css_class("caption-heading");
                            } else {
                                version_label.add_css_class("dim-label");
                            }
                            row.add_suffix(&version_label);
                            
                            let check = gtk::CheckButton::new();
                            check.set_active(updatable);
                            check.set_sensitive(updatable);
                            row.add_prefix(&check);
                            
                            view.imp().devices_list.append(&row);
                        }
                    }
                    
                    glib::Continue(false)
                }),
            );
        }
    }
    
    pub fn update_firmware(&self) {
        let imp = self.imp();
        
        imp.status_label.set_text("Actualizando firmware...");
        imp.spinner.set_visible(true);
        imp.spinner.start();
        imp.check_button.set_sensitive(false);
        imp.update_button.set_sensitive(false);
        
        // Simulate firmware update
        glib::timeout_add_seconds_local(
            4,
            glib::clone!(@weak self as view => @default-return glib::Continue(false), move || {
                view.imp().spinner.stop();
                view.imp().spinner.set_visible(false);
                view.imp().status_label.set_text("Firmware actualizado correctamente. Es posible que se requiera reiniciar.");
                view.imp().check_button.set_sensitive(true);
                
                // Update rows to show updated status
                if let Some(list) = view.imp().devices_list.first_child() {
                    let row = list.downcast_ref::<adw::ActionRow>().unwrap();
                    let suffix = row.last_child().unwrap();
                    
                    if let Some(label) = suffix.downcast_ref::<gtk::Label>() {
                        // Update to show completed status
                        label.set_text("Actualizado");
                        label.remove_css_class("accent");
                        label.add_css_class("success");
                    }
                }
                
                glib::Continue(false)
            }),
        );
    }
}