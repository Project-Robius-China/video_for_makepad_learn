use makepad_widgets::*;

use crate::button::{BasicButtonEvent, BasicButtonWidgetRefExt};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import crate::button::BasicButton;

    App = {{App}}{
        ui : <Root>{
            main_window = <Window>{
                <View>{
                    b_btn = <BasicButton>{
                        height: 60.0,
                        width: 160.0,
                        margin: 8.0,
                        text: "Click me",
                        draw_text: {
                            color: #FF0000,
                            text_style: <THEME_FONT_BOLD>{
                                font_size: 16.0,
                            }
                        },
                        draw_bg: {
                            fn pixel(self) -> vec4{
                                return #ddd;
                            }
                        },
                        align: {x: 0.5, y: 0.5},
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
        crate::button::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        let b_btn = self.ui.basic_button(id!(b_btn));
        let uid = b_btn.widget_uid();
        if let BasicButtonEvent::Clicked = actions.find_widget_action_cast(uid) {
            //  通过borrow_mut()方法获取BasicButton的可变引用，直接更改pub的属性，然后重绘----------
            //    b_btn.borrow_mut().map(|mut x|{
            //        x.text.as_mut_empty().push_str("I have been clicked");
            //        x.draw_text.redraw(cx);
            //    });
            // 通过apply_over_and_redraw()方法直接覆盖并重绘 ----------------------------------
            // b_btn.apply_over_and_redraw(
            //     cx,
            //     live! {
            //         text: "I have been clicked"
            //     },
            // );
            b_btn.set_text_and_redraw(cx, "I have been clicked");

        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
