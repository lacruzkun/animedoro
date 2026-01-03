use core::fmt;

use gtk::glib::{self, SourceId, clone};
use gtk::{gdk, prelude::*};
use relm4::prelude::*;

struct App {
    second: i32,
    minute: i32,
    hour: i32,
    theme: AppTheme,
    timer: Option<SourceId>,
    pause_continue: String,
}

#[derive(Debug)]
enum AppMsg {
    Settings,
    IncTime,
    StartTimer,
    ResetTimer,
}

#[derive(Debug)]
enum AppTheme {
    Dark,
    Blue,
}

impl AppTheme {
    fn get_next_theme(&self) -> Self {
        match self {
            AppTheme::Dark => AppTheme::Blue,
            AppTheme::Blue => AppTheme::Dark,
        }
    }
}

impl fmt::Display for AppTheme {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[relm4::component]
impl SimpleComponent for App {
    type Input = AppMsg;
    type Output = ();
    type Init = i32;

    view!(gtk::Window {
        set_title: Some("Animedoro"),
        set_default_width: 800,
        set_default_height: 400,
        #[watch]
        set_widget_name: &model.theme.to_string(),
        set_modal: true,

        gtk::CenterBox {
            set_orientation: gtk::Orientation::Horizontal,
            set_margin_bottom: 50,
            set_margin_top: 50,
            set_margin_start: 50,
            set_margin_end: 50,

            #[wrap(Some)]
            set_start_widget = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,

                gtk::Label {
                    set_label: "Animedoro",
                    set_halign: gtk::Align::Start,
                    add_css_class: "animedoro",
                },

                gtk::Label {
                    set_label: "by La Cruz",
                    set_halign: gtk::Align::End,
                    add_css_class: "laCruz",
                },
            },

            #[wrap(Some)]
            set_center_widget = &gtk::CenterBox{
                set_widget_name: "centerBox",
                set_orientation: gtk::Orientation::Vertical,

                #[wrap(Some)]
                set_center_widget = &gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,

                    gtk::CenterBox{
                        set_orientation: gtk::Orientation::Horizontal,

                        #[wrap(Some)]
                        set_center_widget = &gtk::Box{
                            set_orientation: gtk::Orientation::Horizontal,
                            set_spacing: 10,

                            gtk::Button{

                                gtk::Box{
                                    set_orientation: gtk::Orientation::Horizontal,
                                    set_spacing: 10,

                                    gtk::Image {
                                        set_from_file: Some("resources/study.svg"),
                                    },

                                    gtk::Label {
                                        set_label: "Work",
                                    },
                                },
                            },

                            gtk::Button {
                                set_label: "Watch",
                            },

                        },
                    },

                    gtk::Label {
                        #[watch]
                        set_label: &format!("{:02}::{:02}::{:02}", model.hour, model.minute, model.second),
                        add_css_class: "time",
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,

                        gtk::Button {
                            #[watch]
                            set_label: &format!("{}", model.pause_continue),
                            connect_clicked => AppMsg::StartTimer,
                        },

                        gtk::Button {
                            set_label: "Reset",
                            connect_clicked => AppMsg::ResetTimer
                        },

                        gtk::Button {
                            set_label: "Settings",
                            connect_clicked => AppMsg::Settings,
                        },
                    },
                },
            },
        },


    });

    fn init(
        _init_value: Self::Init,
        window: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = App {
            second: 0,
            minute: 0,
            hour: 0,
            theme: AppTheme::Blue,
            timer: None,
            pause_continue: String::from("Start"),
        };

        let provider = gtk::CssProvider::new();
        provider.load_from_path("resources/main.css");

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            AppMsg::StartTimer => {
                if (self.timer.is_none()) || ("Continue" == self.pause_continue) {
                    let id = glib::timeout_add_seconds_local(
                        1,
                        clone!(
                            #[strong]
                            sender,
                            move || {
                                sender.input(AppMsg::IncTime);
                                glib::ControlFlow::Continue
                            }
                        ),
                    );
                    self.timer = Some(id);
                    self.pause_continue = String::from("Pause");
                } else if let Some(id) = self.timer.take() {
                    id.remove();
                    self.pause_continue = String::from("Continue");
                }
            }

            AppMsg::IncTime => {
                if self.second == 59 {
                    self.second = 0;
                    self.minute += 1;
                } else {
                    self.second += 1;
                }

                if self.minute == 59 {
                    self.minute = 0;
                    self.hour += 1;
                }
            }

            AppMsg::Settings => {
                self.theme = self.theme.get_next_theme();
                println!("{}", self.theme.to_string());
            }

            AppMsg::ResetTimer => {
                self.pause_continue = String::from("Start");
                if let Some(id) = self.timer.take() {
                    id.remove();
                }
                self.second = 0;
                self.minute = 0;
                self.hour = 0;
                self.theme = AppTheme::Dark;
            }
        }
    }
}

fn main() {
    let app = RelmApp::new("animedoro.org");
    app.run::<App>(0);
}
