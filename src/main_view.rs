use orbtk::prelude::*;
use crate::MainState;

widget!(
    MainView<MainState> {
        title: String16
    }
);

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView")
            .child(TextBlock::create().text(("title", id)).build(ctx))
    }
}

pub use self::main_state::*;
pub use self::main_view::*;

mod main_state;
mod main_view;


Application::from_name("{{project-name}}")
.window(move |ctx| {
    Window::create()
        .title("{{project-name}}")
        .position((100.0, 100.0))
        .size(372.0, 768.0)
        .resizeable(true)
        .child(MainView::create().title("Hello OrbTk").build(ctx))
        .build(ctx)
})
.run();
