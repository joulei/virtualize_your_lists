use makepad_widgets::*;

const NUMBER_OF_ROWS: usize = 100_000_000;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    Row = <View> {
        show_bg: true
        align: {x: 0.5, y: 0.5}
        lbl = <Label> {
            draw_text:{
                color: #fff
            }
        }
    }

    ShortRow = <Row> {
        height: 100
        draw_bg: {
            color: #079992
        }
    }

    TallRow = <Row> {
        height: 200
        draw_bg: {
            color: #0a3d62
        }
    }

    App = {{App}} {
        ui: <Window> {
            body = {
                flow: Overlay
                padding: 0.0
                spacing: 0
                align: {
                    x: 0.0, y: 0.0
                }

                my_widget = {{MyWidget}} {
                    list = <PortalList>{
                        Short = <ShortRow> {}
                        Long = <TallRow> {}
                        scroll_bar: {
                            bar_size: 20.0
                            min_handle_size: 30.0
                            draw_bar: {
                                bar_width: 15.0
                            }
                        }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook, Widget)]
struct MyWidget {
    #[deref]
    view: View,
}

impl Widget for MyWidget {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            
            if let Some(mut list) = item.as_portal_list().borrow_mut() {
                list.set_item_range(cx, 0, NUMBER_OF_ROWS);

                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = if item_id % 2 == 0 {
                        live_id!(Long)
                    } else {
                        live_id!(Short)
                    };

                    let item = list.item(cx, item_id, template).unwrap();
                    item.label(id!(lbl)).set_text(&format!("{}", item_id));
                    item.draw_all(cx, &mut Scope::empty());
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
