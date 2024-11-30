use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::widgets::*;
    use link::shaders::*;
    // use crate::link1::*;
    // use crate::link2::*;
    use link::MyLib::*;

    App = {{App}}{
        ui : <Root>{
            main_window = <Window>{
                <View>{
                    spacing: 12.0,
                    flow: Down,
                    <ABtn>{}
                    <BBtn>{}
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
        crate::link1::live_design(cx);
        crate::link2::live_design(cx);
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
