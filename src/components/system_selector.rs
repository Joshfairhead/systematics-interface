use yew::prelude::*;
use crate::core::system_config::SystemConfig;

#[derive(Properties, PartialEq)]
pub struct SystemSelectorProps {
    pub systems: Vec<SystemConfig>,
    pub selected: String,
    pub on_select: Callback<String>,
}

#[function_component(SystemSelector)]
pub fn system_selector(props: &SystemSelectorProps) -> Html {
    html! {
        <div class="number-selector">
            {
                props.systems.iter().enumerate().map(|(index, system)| {
                    let is_selected = system.name == props.selected;
                    let system_name = system.name.clone();
                    let onclick = {
                        let on_select = props.on_select.clone();
                        Callback::from(move |_| {
                            on_select.emit(system_name.clone());
                        })
                    };

                    html! {
                        <button
                            class={ if is_selected { "number-button selected" } else { "number-button" } }
                            onclick={ onclick }
                            style={ if is_selected {
                                format!("background-color: {}; border-color: {}", system.color_scheme.nodes, system.color_scheme.nodes)
                            } else {
                                format!("border-color: {}", system.color_scheme.nodes)
                            }}
                            title={ format!("{} ({})", system.display_name, system.k_notation) }
                        >
                            { index + 1 }
                        </button>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
