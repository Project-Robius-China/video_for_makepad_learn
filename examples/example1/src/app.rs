use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    App = {{App}}{
        ui : <Root>{
            main_window = <Window>{
                <View>{
                    <Label>{
                        text: "Hello, World!",
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
