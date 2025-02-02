use druid::{
    AppLauncher, Color, Env, Event, EventCtx, LocalizedString, RenderContext, Widget, WidgetExt, WindowDesc, 
    widget::Controller
};
use druid::widget::{Button, Checkbox, Flex, Label, List, TextBox, Scroll, Painter};
use std::sync::Arc;
use crate::types::{Task, AppState};
use crate::utils::{load_tasks, add_task, filtered_tasks_lens};

mod types;
mod utils;

fn main() {
    println!("Hello, Rust Learn Project!");
    let main_window = WindowDesc::new(build_ui)
        .title(LocalizedString::new("To-Do List"))
        .window_size((400.0, 600.0));

    let initial_state = AppState {
        tasks: load_tasks().unwrap_or_else(|_| Arc::new(Vec::new())),
        new_task_description: String::new(),
        show_completed: true,
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");
}

fn build_ui() -> impl Widget<AppState> {
    let description_input = TextBox::new()
        .with_placeholder("Enter a new task")
        .expand_width()
        .lens(AppState::new_task_description)
        .controller(EnterController);

    let task_list = Scroll::new(List::new(|| {
        Flex::row()
            .with_child(Checkbox::new("").lens(Task::done))
            .with_child(Label::dynamic(|task: &Task, _env: &Env| {
                format!("{}", task.description)
            }).with_text_color(Color::BLACK)) // Set text color to black
            .padding(5.0)
    }).lens(filtered_tasks_lens()))
    .vertical()
    .expand_height();

    let add_task_button = Button::new("Add Task")
        .on_click(|_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
            add_task(data);
        })
        .background(Painter::new(|ctx, _, _| {
            let bounds = ctx.size().to_rect();
            ctx.fill(bounds, &Color::rgb8(0x00, 0x7A, 0xCC));
        }))
        .border(Color::WHITE, 2.0)
        .rounded(5.0)
        .padding(10.0)
        .expand_width();

    let show_completed_checkbox = Flex::row()
        .with_child(Checkbox::new("").lens(AppState::show_completed))
        .with_child(Label::new("Show completed tasks").with_text_color(Color::BLACK));

    Flex::column()
        .with_child(description_input)
        .with_spacer(10.0)
        .with_child(add_task_button)
        .with_spacer(10.0)
        .with_child(show_completed_checkbox)
        .with_spacer(10.0)
        .with_flex_child(task_list, 1.0)
        .padding(10.0)
        .background(Color::rgb8(0xE0, 0xE0, 0xE0)) // Adjusted background color for better readability
}

struct EnterController;

impl<W: Widget<AppState>> Controller<AppState, W> for EnterController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        if let Event::KeyDown(key_event) = event {
            if key_event.key == druid::keyboard_types::Key::Enter {
                add_task(data);
            }
        }
        child.event(ctx, event, data, env);
    }
}