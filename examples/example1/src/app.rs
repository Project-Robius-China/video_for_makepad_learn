use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import crate::views::my_button::MyButton;
    App = {{App}}{
        ui : <Root>{
            main_window = <Window>{
                <View>{
                    <Label>{
                        text: "Hello, World!",
                        padding: {
                            left: 10.0,
                            right: 10.0,
                            top: 4.6,
                        },
                        align: {x: 0.5, y: 0.5},
                        height: Fit,
                        width: Fit,
                        draw_text: {
                            color: #FF0000,
                            text_style: {
                                font_size: 32.0,

                            }
                        }
                    }
                }
            }
        }
    }

}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
