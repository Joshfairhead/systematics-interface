use yew::prelude::*;
use crate::core::system_config::SystemConfig;

#[derive(Properties, PartialEq)]
pub struct SystemSelectorProps {
    pub systems: Vec<SystemConfig>,
    pub selected: String,
    pub on_select: Callback<String>,
    #[prop_or_default]
    pub show_edge_labels: bool,
    #[prop_or_default]
    pub on_toggle_edge_labels: Option<Callback<()>>,
}

#[function_component(SystemSelector)]
pub fn system_selector(props: &SystemSelectorProps) -> Html {
    html! {
        <nav class="top-nav">
            <div class="nav-items">
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
                                class={ if is_selected { "nav-button selected" } else { "nav-button" } }
                                onclick={ onclick }
                                title={ system.k_notation.clone() }
                            >
                                { &system.display_name }
                            </button>
                        }
                    }).collect::<Html>()
                }
            </div>

            // Edge labels toggle switch
            if let Some(ref on_toggle) = props.on_toggle_edge_labels {
                <div class="nav-controls">
                    <label class="edge-label-toggle">
                        <span class="toggle-label">{"Edge Labels"}</span>
                        <div class="toggle-switch">
                            <input
                                type="checkbox"
                                checked={props.show_edge_labels}
                                onclick={{
                                    let on_toggle = on_toggle.clone();
                                    Callback::from(move |_| on_toggle.emit(()))
                                }}
                            />
                            <span class="slider"></span>
                        </div>
                    </label>
                </div>
            }
        </nav>
    }
}
