use core::fmt;
use std::env;
use std::path::PathBuf;

use gtk::glib::{self, SourceId, clone};
use gtk::{gdk, prelude::*};
use relm4::prelude::*;

struct MainScreen {
    second: i32,
    minute: i32,
    hour: i32,
    work: i32,
    watch: i32,
    theme: AppTheme,
    timer: Option<SourceId>,
    pause_continue: String,
    mode: AppMode,
}

#[derive(Debug)]
enum MainMsg {
    IncTime,
    StartTimer,
    ResetTimer,
    ChangeTheme,
    Work,
    Watch,
    SetTheme(AppTheme),
    SetWatch(i32),
    SetWork(i32),
    SwitchToSettings,
}

#[relm4::component]
impl SimpleComponent for MainScreen {
    type Input = MainMsg;
    type Output = AppMsg;
    type Init = ();

    view!(gtk::CenterBox {
            set_orientation: gtk::Orientation::Horizontal,
            set_margin_bottom: 50,
            set_margin_top: 50,
            set_margin_start: 50,
            set_margin_end: 50,

            #[wrap(Some)]
            set_start_widget = &gtk::CenterBox{
                set_orientation: gtk::Orientation::Vertical,

                #[wrap(Some)]
                set_start_widget = &gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,

                    gtk::Label {
                        set_label: "Animedoro",
                        set_halign: gtk::Align::Start,
                        #[watch]
                        set_widget_name: &format!("{}animedoro", model.theme.to_string()),
                    },

                    gtk::Label {
                        set_label: "by La Cruz",
                        set_halign: gtk::Align::End,
                        #[watch]
                        set_widget_name: &format!("{}laCruz", model.theme.to_string()),
                    },
                },

                #[wrap(Some)]
                set_end_widget = &gtk::Box{
                    set_orientation: gtk::Orientation::Vertical,
                    set_spacing: 10,
                    gtk::Label {
                        set_label: "Current Theme",
                        #[watch]
                        set_widget_name: &format!("{}ThemeLabel", model.theme.to_string()),
                    },

                    gtk::Button{
                        #[watch]
                        set_label: &model.theme.to_string(),
                        #[watch]
                        set_widget_name: &format!("{}ThemeButton", model.theme.to_string()),
                        connect_clicked => MainMsg::ChangeTheme,
                    },
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
                                        set_from_file: Some(binary_relative("study.svg")),
                                    },

                                    gtk::Label {
                                        set_label: "Work",
                                    },
                                },
                                #[watch]
                                set_widget_name: &format!("{}Button", model.theme.to_string()),
                                connect_clicked => MainMsg::Work,
                            },

                            gtk::Button {
                                set_label: "Watch",
                                #[watch]
                                set_widget_name: &format!("{}Button", model.theme.to_string()),
                                connect_clicked => MainMsg::Watch,
                            },

                        },
                    },

                    gtk::Label {
                        #[watch]
                        set_label: &format!("{:02}::{:02}::{:02}", model.hour, model.minute, model.second),
                        #[watch]
                        set_widget_name: &format!("{}time", model.theme.to_string()),
                    },

                    gtk::Box {
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 10,

                        gtk::Button {
                            #[watch]
                            set_label: &format!("{}", model.pause_continue),
                            connect_clicked => MainMsg::StartTimer,
                            #[watch]
                            set_widget_name: &format!("{}Button", model.theme.to_string()),
                        },

                        gtk::Button {
                            set_label: "Reset",
                            connect_clicked => MainMsg::ResetTimer,
                            #[watch]
                            set_widget_name: &format!("{}Button", model.theme.to_string()),
                        },

                        gtk::Button {
                            set_label: "Settings",
                            connect_clicked => MainMsg::SwitchToSettings,
                            #[watch]
                            set_widget_name: &format!("{}Button", model.theme.to_string()),
                        },
                    },
                },
            },
        },);

    fn init(
        _init_value: Self::Init,
        window: Self::Root,
        _sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = MainScreen {
            second: 0,
            minute: 50,
            hour: 0,
            work: 50,
            watch: 25,
            theme: AppTheme::Blue,
            timer: None,
            pause_continue: String::from("Start"),
            mode: AppMode::Work,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }
    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            MainMsg::IncTime => {
                if self.second == 0 {
                    self.second = 59;
                    if self.minute == 0 {
                        sender.input(MainMsg::ResetTimer);
                    }
                    self.minute -= 1;
                } else {
                    self.second -= 1;
                }
            }

            MainMsg::StartTimer => {
                if (self.timer.is_none()) || ("Continue" == self.pause_continue) {
                    let id = glib::timeout_add_seconds_local(
                        1,
                        clone!(
                            #[strong]
                            sender,
                            move || {
                                sender.input(MainMsg::IncTime);
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

            MainMsg::ResetTimer => {
                self.pause_continue = String::from("Start");
                if let Some(id) = self.timer.take() {
                    id.remove();
                }
                match self.mode {
                    AppMode::Work => {
                        self.second = 0;
                        self.hour = 0;
                        self.minute = self.work;
                    }
                    AppMode::Watch => {
                        self.second = 0;
                        self.hour = 0;
                        self.minute = self.watch;
                    }
                }
            }
            MainMsg::Work => {
                self.mode = AppMode::Work;
                sender.input(MainMsg::ResetTimer);
            }
            MainMsg::Watch => {
                self.mode = AppMode::Watch;
                sender.input(MainMsg::ResetTimer);
            }

            MainMsg::ChangeTheme => {
                let _ = sender.output(AppMsg::ChangeTheme);
            }

            MainMsg::SetTheme(x) => {
                self.theme = x;
            }

            MainMsg::SwitchToSettings => {
                let _ = sender.output(AppMsg::SwitchToSettings);
            }

            MainMsg::SetWatch(watch) => {
                self.watch = watch;
            }

            MainMsg::SetWork(work) => {
                self.work = work;
            }
        }
    }
}

struct App {
    theme: AppTheme,
    screen: Screen,
    work: i32,
    watch: i32,
    main: Controller<MainScreen>,
    settings: Controller<SettingsScreen>,
}

#[derive(Debug)]
enum Screen {
    Main,
    Settings,
}

#[derive(Debug)]
enum AppMsg {
    ChangeTheme,
    SwitchToSettings,
    SwitchToMain(i32, i32),
}

#[derive(Debug)]
enum AppMode {
    Work,
    Watch,
}

#[derive(Debug, Clone)]
enum AppTheme {
    Dark,
    Blue,
    Serene,
    Dream,
}

impl AppTheme {
    fn get_next_theme(&self) -> Self {
        match self {
            AppTheme::Dark => AppTheme::Blue,
            AppTheme::Blue => AppTheme::Serene,
            AppTheme::Serene => AppTheme::Dream,
            AppTheme::Dream => AppTheme::Dark,
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
        #[watch]
        set_widget_name: &model.theme.to_string(),
        set_modal: true,

        gtk::Stack  {
            set_transition_type: gtk::StackTransitionType::SlideLeftRight,
            set_transition_duration: 250,
            add_named: (model.main.widget(), Some("main")),
            add_named: (model.settings.widget(), Some("settings")),

            #[watch]
            set_visible_child_name: match model.screen {
                Screen::Main => "main",
                Screen::Settings => "settings",
            },

        }

    });

    fn init(
        _init_value: Self::Init,
        window: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let main = MainScreen::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);

        let settings = SettingsScreen::builder()
            .launch(())
            .forward(sender.input_sender(), |msg| msg);

        let model = App {
            screen: Screen::Main,
            theme: AppTheme::Blue,
            work: 50,
            watch: 25,
            main: main,
            settings: settings,
        };

        let provider = gtk::CssProvider::new();
        let abs_path = binary_relative("main.css");
        provider.load_from_path(abs_path);

        gtk::style_context_add_provider_for_display(
            &gdk::Display::default().unwrap(),
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, _sender: ComponentSender<Self>) {
        match message {
            AppMsg::ChangeTheme => {
                self.theme = self.theme.get_next_theme();
                self.main.emit(MainMsg::SetTheme(self.theme.clone()));
                self.settings.emit(SetMsg::SetTheme(self.theme.clone()));
                println!("{}", self.theme.to_string());
            }

            AppMsg::SwitchToSettings => {
                self.screen = Screen::Settings;
            }

            AppMsg::SwitchToMain(work, watch) => {
                self.screen = Screen::Main;
                self.work = work;
                self.watch = watch;
                self.main.emit(MainMsg::SetWork(self.work.clone()));
                self.main.emit(MainMsg::SetWatch(self.watch.clone()));
                self.main.emit(MainMsg::ResetTimer);
            }
        }
    }
}

struct SettingsScreen {
    theme: AppTheme,
    work: i32,
    watch: i32,
}

#[derive(Debug)]
enum SetMsg {
    ChangeTheme,
    SetTheme(AppTheme),
    Back,
    SetWork(i32),
    SetWatch(i32),
}

#[relm4::component]
impl SimpleComponent for SettingsScreen {
    type Input = SetMsg;
    type Output = AppMsg;
    type Init = ();

    view!(gtk::CenterBox {
        set_orientation: gtk::Orientation::Horizontal,

        #[wrap(Some)]
        set_start_widget =  &gtk::CenterBox {
            set_orientation: gtk::Orientation::Vertical,
            set_margin_bottom: 50,
            set_margin_top: 50,
            set_margin_start: 50,
            set_margin_end: 50,

            #[wrap(Some)]
            set_start_widget = &gtk::Button{
                gtk::Box{
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 10,

                    gtk::Image {
                        set_from_file: Some(binary_relative("back.svg")),
                    },
                },
                #[watch]
                set_widget_name: &format!("{}Button", model.theme.to_string()),
                connect_clicked => SetMsg::Back,
            },
        },

        #[wrap(Some)]
        set_center_widget = &gtk::CenterBox{
            set_orientation: gtk::Orientation::Vertical,

            #[wrap(Some)]
            set_center_widget = &gtk::Box{
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 10,

                gtk::Label {
                    set_label: "Current Theme",
                    #[watch]
                    set_widget_name: &format!("{}ThemeLabel", model.theme.to_string()),
                },

                gtk::Button {
                    set_label: &format!("{}", model.theme.to_string()),
                    #[watch]
                    set_widget_name: &format!("{}Button", model.theme.to_string()),
                    connect_clicked => SetMsg::ChangeTheme,
                },

                gtk::Label {
                    set_label: "Work Duration",
                    #[watch]
                    set_widget_name: &format!("{}ThemeLabel", model.theme.to_string()),
                },

                gtk::Box{
                    set_spacing: 10,
                    
                    gtk::Button {
                        set_label: "+",
                        connect_clicked => SetMsg::SetWork(1),
                        #[watch]
                        set_widget_name: &format!("{}Button", model.theme.to_string()),
                    },

                    gtk::Label {
                        #[watch]
                        set_label: &format!("{}", model.work),
                        #[watch]
                        set_widget_name: &format!("{}ThemeLabel", model.theme.to_string()),
                    },

                    gtk::Button {
                        set_label: "-",
                        connect_clicked => SetMsg::SetWork(-1),
                        #[watch]
                        set_widget_name: &format!("{}Button", model.theme.to_string()),
                    },
                },

                gtk::Label {
                    #[watch]
                    set_label: "Watch Duration",
                    #[watch]
                    set_widget_name: &format!("{}ThemeLabel", model.theme.to_string()),
                },

                gtk::Box{
                    set_spacing: 10,
                    gtk::Button {
                        set_label: "+",
                        connect_clicked => SetMsg::SetWatch(1),
                        #[watch]
                        set_widget_name: &format!("{}Button", model.theme.to_string()),
                    },

                    gtk::Label {
                        #[watch]
                        set_label: &format!("{}", model.watch),
                        set_widget_name: &format!("{}ThemeLabel", model.theme.to_string()),
                    },

                    gtk::Button {
                        set_label: "-",
                        connect_clicked => SetMsg::SetWatch(-1),
                        #[watch]
                        set_widget_name: &format!("{}Button", model.theme.to_string()),
                    },
                }
            }
        }
    },);

    fn init(
        _init_value: Self::Init,
        window: Self::Root,
        sender: relm4::ComponentSender<Self>,
    ) -> relm4::ComponentParts<Self> {
        let model = SettingsScreen {
            theme: AppTheme::Blue,
            work: 50,
            watch: 25,
        };

        let widgets = view_output!();
        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            SetMsg::ChangeTheme => {
                let _ = sender.output(AppMsg::ChangeTheme);
            }

            SetMsg::SetTheme(x) => {
                self.theme = x;
            }

            SetMsg::Back => {
                let _ = sender.output(AppMsg::SwitchToMain(self.work, self.watch));
            }

            SetMsg::SetWork(x) => {
                self.work += x;
            }

            SetMsg::SetWatch(x) => {
                self.watch += x;
            }
        }
    }
}

fn binary_relative(path: &str) -> PathBuf{
    if cfg!(target_os = "macos"){
        let exe = env::current_exe().expect("cannot locate executable");
        exe.parent().expect("executable doesn't have parent").join("Resources").join("resources").join(path)
    }
    else {
        env::current_exe().expect("cannot locate executable")
            .parent()
            .expect("executable doesn't have a parent")
            .join("resources")
            .join(path)
    }
}

fn main() {
    let app = RelmApp::new("animedoro.org");
    app.run::<App>(0);
}
