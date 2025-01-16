use iocraft::prelude::*;
mod tea_component;
use tea_component::FormField;
mod hteacup;

#[derive(Default, Props)]
struct FormProps<'a> {
    first_name_out: Option<&'a mut String>,
}

#[component]
fn App(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut system = hooks.use_context_mut::<SystemContext>();
    let (width, height) = hooks.use_terminal_size();
    let mut first_name = hooks.use_state(|| "".to_string());
    let mut second_name = hooks.use_state(|| "".to_string());
    let mut focus = hooks.use_state(|| 0);
    let mut should_exit = hooks.use_state(|| false);

    hooks.use_terminal_events(move |event| match event {
        TerminalEvent::Key(KeyEvent { code, kind, .. }) if kind != KeyEventKind::Release => {
            match code {
                KeyCode::Enter => (),
                KeyCode::Tab => focus.set((focus + 1) % 2),
                KeyCode::Esc => should_exit.set(true),
                _ => {}
            }
        }
        _ => {}
    });

    if should_exit.get() {
        system.exit();
        element!(
        Box() {
            Text(content: "bye")
        })
    } else {
        element! {
            Box(
                width: width - 1,
                height: height - 1,
                border_style: BorderStyle::Round,
                border_color: Color::Blue,
                flex_direction: FlexDirection::Row
            ) {
                Box(
                    width: width / 3 * 2,
                    height: height - 3,
                    border_color: Color::Green,
                    flex_direction: FlexDirection::Row,
                    border_style: BorderStyle::Single
                ) {
                FormField(label: "test",value:  first_name, has_focus: focus == 0)
                }
                Box(
                    width: width / 3,
                    height: height - 3,
                    border_color: Color::Green,
                    flex_direction: FlexDirection::Row,
                    border_style: BorderStyle::Single
                ) {
                    FormField(label: "test",value:  second_name, has_focus: focus == 1)
                }

            }
        }
    }
}

fn main() {
    smol::block_on(
        element! {
                App()
        }
        .render_loop(),
    )
    .unwrap()
}
