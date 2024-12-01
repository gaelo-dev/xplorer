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

use crate::bluetooth::ConnectionState;

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

impl Screen {
    pub fn create(state: &ConnectionState) -> Self {
        match state {
            ConnectionState::Connected { central, xplorer } => connected::Connected.into(),
            ConnectionState::Disconnected { peripherals, .. } => {
                disconnected::Disconnected::new(peripherals.clone()).into()
            },
            ConnectionState::Loading => loading::Loading.into(),
        }
    }
}
