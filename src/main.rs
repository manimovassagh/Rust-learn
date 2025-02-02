use druid::{
    AppLauncher, Color, Data, Env, Event, EventCtx, Lens, LocalizedString, RenderContext, Widget, WidgetExt, WindowDesc, 
    widget::Controller
};
use druid::widget::{Button, Checkbox, Flex, Label, List, TextBox, Scroll, Painter};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::sync::Arc;
use rand::random;

#[derive(Clone, Data, Lens, Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

#[derive(Clone, Data, Lens)]
struct AppState {
    tasks: Arc<Vec<Task>>,
    new_task_description: String,
    show_completed: bool,
}

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
            }))
            .padding(5.0)
    }).lens(filtered_tasks_lens()))
    .vertical()
    .expand_height();

    let add_task_button = Button::new("Add Task")
        .on_click(|_ctx: &mut EventCtx, data: &mut AppState, _env: &Env| {
            add_task(data);
        })
        .background(Painter::new(|ctx, _, env| {
            let bounds = ctx.size().to_rect();
            ctx.fill(bounds, &Color::rgb8(0x00, 0x7A, 0xCC));
        }))
        .border(Color::WHITE, 2.0)
        .rounded(5.0)
        .padding(10.0)
        .expand_width();

    let show_completed_checkbox = Checkbox::new("Show completed tasks")
        .lens(AppState::show_completed);

    Flex::column()
        .with_child(description_input)
        .with_spacer(10.0)
        .with_child(add_task_button)
        .with_spacer(10.0)
        .with_child(show_completed_checkbox)
        .with_spacer(10.0)
        .with_flex_child(task_list, 1.0)
        .padding(10.0)
        .background(Color::rgb8(0xF0, 0xF0, 0xF0))
}

fn add_task(data: &mut AppState) {
    let description = data.new_task_description.trim();
    if !description.is_empty() {
        let mut new_tasks = Vec::clone(&data.tasks);
        new_tasks.push(Task {
            id: random(),
            description: description.to_string(),
            done: false,
        });
        data.tasks = Arc::new(new_tasks);
        data.new_task_description.clear();
        if let Err(e) = save_tasks(&data.tasks) {
            eprintln!("Error saving tasks: {}", e);
        }
    }
}

fn filtered_tasks_lens() -> impl Lens<AppState, Arc<Vec<Task>>> {
    druid::lens::Map::new(
        |data: &AppState| {
            if data.show_completed {
                data.tasks.clone()
            } else {
                Arc::new(data.tasks.iter()
                    .filter(|task| !task.done)
                    .cloned()
                    .collect())
            }
        },
        |data: &mut AppState, filtered: Arc<Vec<Task>>| {
            if data.show_completed {
                data.tasks = filtered;
            }
        }
    )
}

fn load_tasks() -> Result<Arc<Vec<Task>>, Box<dyn Error>> {
    if std::path::Path::new("tasks.json").exists() {
        let file = File::open("tasks.json")?;
        let tasks: Vec<Task> = serde_json::from_reader(file)?;
        Ok(Arc::new(tasks))
    } else {
        Ok(Arc::new(Vec::new()))
    }
}

fn save_tasks(tasks: &Arc<Vec<Task>>) -> Result<(), Box<dyn Error>> {
    let file = File::create("tasks.json")?;
    serde_json::to_writer(file, tasks.as_ref())?;
    Ok(())
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