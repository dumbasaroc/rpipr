use gsk::glib::Object;
use gtk4::glib;
use gtk4::*;
use gtk4::subclass::prelude::*;
use gtk4::glib::subclass::InitializingObject;

glib::wrapper! {
    pub struct BaseWindow(ObjectSubclass<priv_BaseWindow>)
        @extends ApplicationWindow, Window, Widget,
        @implements gio::ActionMap, gio::ActionGroup,
                    gtk4::Buildable;
}

impl BaseWindow {

    pub fn new(app: &Application) -> Self {
        Object::builder().property("application", &app).build()
    }
}

#[allow(non_camel_case_types)]
#[derive(Default, CompositeTemplate)]
#[template(file = "../source/base_window.ui")]
pub struct priv_BaseWindow {

    #[template_child]
    pub main_menu: TemplateChild<super::MainMenu>
}

#[glib::object_subclass]
impl ObjectSubclass for priv_BaseWindow {
    const NAME: &'static str = "BaseWindow";
    type Type = BaseWindow;
    type ParentType = gtk4::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl ApplicationWindowImpl for priv_BaseWindow {}
impl WindowImpl for priv_BaseWindow {}
impl WidgetImpl for priv_BaseWindow {}
impl ObjectImpl for priv_BaseWindow {}
