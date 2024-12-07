pub mod loading {
    use iced::{Element, widget};

    #[derive(Debug, Clone)]
    pub struct Loading;
    
    impl Loading {
        pub fn view<Msg>(&self) -> Element<Msg> {
            widget::text("...").into()
        }
    }
}

pub mod connected;
pub mod disconnected;

macro_rules! screen {
    ( $( $screen:ident => $module:ident ,)* ) => {
        #[derive(Debug, Clone)]
        pub enum Screen {
            $(
                $screen($module::$screen),
            )*
        }

        $(
            impl From<$module::$screen> for Screen {
                fn from(value: $module::$screen) -> Self {
                    Self::$screen(value)
                }
            }
        )*
    };
}

screen!(
    Connected => connected,
    Disconnected => disconnected,
    Loading => loading,
);
