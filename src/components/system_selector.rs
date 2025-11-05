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
        <div class="system-selector">
            <h3>{ "Select System" }</h3>
            <div class="system-buttons">
                {
                    props.systems.iter().map(|system| {
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
                                class={ if is_selected { "system-button selected" } else { "system-button" } }
                                onclick={ onclick }
                                style={ format!("border-color: {}", system.color_scheme.nodes) }
                            >
                                <div class="button-label">{ &system.display_name }</div>
                                <div class="button-notation">{ &system.k_notation }</div>
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>
        </div>
    }
}
