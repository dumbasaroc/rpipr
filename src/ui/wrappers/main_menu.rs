use glib::Object;
use gtk4::*;
use subclass::box_::BoxImpl;
use subclass::prelude::{ObjectImpl, ObjectSubclass};
use gtk4::glib::subclass::InitializingObject;
use gtk4::subclass::widget::*;
use subclass::widget::WidgetImpl;

/*
final class Gtk.CenterBox : Gtk.Widget
  implements Gtk.Accessible, Gtk.Buildable, Gtk.ConstraintTarget, Gtk.Orientable {
  /* No available fields */
}
*/

glib::wrapper! {
    pub struct MainMenu(ObjectSubclass<priv_MainMenu>)
        @extends Box, Widget,
        @implements Accessible, Buildable, ConstraintTarget, Orientable;
}

impl MainMenu {

    pub fn new() -> Self {
        Object::builder().build()
    }
}



#[allow(non_camel_case_types)]
#[derive(Default, CompositeTemplate)]
#[template(file = "../source/main_menu.ui")]
pub struct priv_MainMenu {

    #[template_child]
    pub centerbox: TemplateChild<CenterBox>,
}

#[glib::object_subclass]
impl ObjectSubclass for priv_MainMenu {
    const NAME: &'static str = "MainMenu";
    type Type = MainMenu;
    type ParentType = gtk4::Box;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

impl BoxImpl for priv_MainMenu {}
impl WidgetImpl for priv_MainMenu {}
impl ObjectImpl for priv_MainMenu {}


