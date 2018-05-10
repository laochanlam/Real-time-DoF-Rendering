extern crate gtk;
extern crate gdk;
extern crate gdk_pixbuf;

extern crate image;

use relm::{Relm, Update, Widget};
use gtk::Orientation::{Vertical};

use image::{GenericImage};
// Widget
use gtk::{
    Window,
    WindowType,
    GtkWindowExt,
    WidgetExt,
    Button,
    ButtonExt,
    ContainerExt,
    Inhibit,
    FileChooserExt,
    FileChooserButton,
    FileChooserAction,
    FileChooserButtonExt,
    Image,
    ImageExt,
    GestureMultiPress,
    GestureMultiPressExt,
};

use self::gdk:: {Screen, ScreenExt};


use self::gdk_pixbuf::{Pixbuf};

pub struct Model {
    screen_height: i32,
    screen_width: i32,
    unit_width: f64,
    unit_height: f64,
}

#[derive(Clone)]
pub struct Widgets {
    exit_button: Button,
    filechooser_button: FileChooserButton,
    blur_button: Button,
    preview_img: Image,
    gesture_drag: GestureMultiPress,
    window: Window,
}

#[derive(Msg)]
pub enum Msg {
    Quit,
    ChooseFile,
    Blur,
    Click(f64, f64)
}

pub struct Win {
    model: Model,
    widgets: Widgets,
}



impl Update for Win {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
            screen_height: Screen::get_default().unwrap().get_height(),
            screen_width: Screen::get_default().unwrap().get_width(),
            unit_width: 1.25,
            unit_height: 1.7142857142857142,
        }
    }
    
    fn update(&mut self, event: Msg) {
        let preview_img = &self.widgets.preview_img;
        let filechooser_button = &self.widgets.filechooser_button;

        match event {
            Msg::Quit => gtk::main_quit(),
            Msg::ChooseFile => {
                let pathname = filechooser_button.get_filename().unwrap().into_os_string().into_string().unwrap();

                let img = image::open(&pathname).unwrap();
                let (width_img, height_img) = img.dimensions();
                self.model.unit_width = (width_img as f64) / (self.model.screen_width as f64);
                self.model.unit_height = (height_img as f64) / ((self.model.screen_height - 100) as f64);
                println!("uw:{}, uh:{}",self.model.unit_width ,self.model.unit_height);

                // range 1280~ (800 - 100)
                let new_img_pixbuf = Pixbuf::new_from_file_at_scale(&pathname, self.model.screen_width, self.model.screen_height - 100, false).unwrap();
                preview_img.set_from_pixbuf(&new_img_pixbuf);
                self.widgets.window.fullscreen();
                println!("{}", pathname);
            },
            Msg::Blur => {},
            Msg::Click(x, y) => { println!("y:{}, x:{}",x * self.model.unit_width, y * self.model.unit_height); 
            
            
            }
        }
    }
}

impl Widget for Win {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.widgets.window.clone()
    }

    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {

        // Create a box
        let vbox = gtk::Box::new(Vertical, 0);
        // Add botton into box
        let exit_button = Button::new_with_label("Exit");
        let filechooser_button = FileChooserButton::new("Select", FileChooserAction::Open);
        let blur_button = Button::new_with_label("Blur");
        let window = Window::new(WindowType::Toplevel);

        let screen_height = Screen::get_default().unwrap().get_height();
        let screen_width = Screen::get_default().unwrap().get_width();
        let img_pixbuf = Pixbuf::new_from_file_at_scale("data/input.bmp", screen_width, screen_height - 100, false).unwrap();
    
        let preview_img = Image::new_from_pixbuf(&img_pixbuf);
        let gesture_drag = GestureMultiPress::new(&window);

        let img = image::open("data/input.bmp").unwrap();
        let (width_img, height_img) = img.dimensions();
        let unit_width = (width_img as f64) / (screen_width as f64);
        let unit_height = (height_img as f64) / ((screen_height - 100) as f64);
        println!("uw:{}, uh:{}",unit_width ,unit_height);

        vbox.add(&preview_img);
        vbox.add(&filechooser_button);
        vbox.add(&blur_button);
        vbox.add(&exit_button);
        window.add(&vbox);

        window.fullscreen();

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));
        connect!(relm, exit_button, connect_clicked(_), Msg::Quit);
        connect!(relm, filechooser_button, connect_file_set(_), Msg::ChooseFile);
        connect!(relm, blur_button, connect_clicked(_), Msg::Blur);
        connect!(relm, gesture_drag, connect_pressed(_, _, x, y), Msg::Click(x, y));
        window.show_all();
        
        // return
        Win {
            model,
            widgets: Widgets {
                exit_button,
                filechooser_button,
                blur_button,
                preview_img,
                gesture_drag,
                window : window,
            }
        }
    }
}
