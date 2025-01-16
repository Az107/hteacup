use iocraft::prelude::*;

#[derive(Default, Props)]
pub struct FormFieldProps {
    pub label: String,
    pub value: Option<State<String>>,
    pub has_focus: bool,
}

#[component]
pub fn FormField(props: &FormFieldProps) -> impl Into<AnyElement<'static>> {
    let Some(mut value) = props.value else {
        panic!("value is required");
    };

    element! {
        Box(
            border_style: if props.has_focus { BorderStyle::Round } else { BorderStyle::None },
            border_color: Color::Blue,
            padding_left: if props.has_focus { 0 } else { 1 },
            padding_right: if props.has_focus { 0 } else { 1 },
            height: 3
        ) {
            Box(width: 15) {
                Text(content: format!("{}: ", props.label))
            }
            Box(
                background_color: Color::DarkGrey,
                width: 30,
            ) {
                TextInput(
                    has_focus: props.has_focus,
                    value: value.to_string(),
                    on_change: move |new_value| value.set(new_value),
                )
            }
        }
    }
}
