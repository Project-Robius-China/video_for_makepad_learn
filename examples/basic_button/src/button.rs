use makepad_widgets::*;

live_design! {
    BasicButton = {{BasicButton}}{

    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BasicButton {
    #[redraw]
    #[live]
    draw_bg: DrawQuad,
    #[walk]
    walk: Walk,
    #[live]
    pub text: ArcStringMut,
    #[live]
    pub draw_text: DrawText,
    #[layout]
    layout: Layout,
}

#[derive(DefaultNone, Clone, Debug)]
pub enum BasicButtonEvent {
    Clicked,
    None,
}

impl Widget for BasicButton {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.draw_bg.begin(cx, walk, self.layout);

        self.draw_text
            .draw_walk(cx, Walk::fit(), Align::default(), self.text.as_ref());
        self.draw_bg.end(cx);
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        match event.hits(cx, self.area()) {
            Hit::FingerHoverIn(finger_hover_event) => {
                dbg!("Hover in");
            }
            Hit::FingerHoverOut(finger_hover_event) => {
                dbg!("Hover out");
            }
            Hit::FingerUp(finger_up_event) => {
                cx.widget_action(self.widget_uid(), &scope.path, BasicButtonEvent::Clicked);
            }
            _ => (),
        }
    }
    fn text(&self) -> String {
        self.text.as_ref().to_string()
    }
    fn set_text(&mut self, v: &str) {
        self.text.as_mut_empty().push_str(v);
    }
    fn set_text_and_redraw(&mut self, cx: &mut Cx, v: &str) {
        self.set_text(v);
        self.draw_text.redraw(cx);
    }
}

impl BasicButton {
    pub fn area(&self) -> Area {
        self.draw_bg.area()
    }
}
