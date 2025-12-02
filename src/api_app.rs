use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::api::models::SystemData;
use crate::api::client::MockApiClient;
use crate::components::api_graph_view::ApiGraphView;
use crate::components::system_selector::SystemSelector;
use crate::core::system_config::SystemConfig;

pub enum ApiAppMsg {
    SelectSystem(String),
    SystemsLoaded(Vec<SystemData>),
    SystemLoaded(SystemData),
    LoadError(String),
}

pub struct ApiApp {
    systems: Vec<SystemData>,
    selected_system: Option<SystemData>,
    loading: bool,
    error: Option<String>,
}

impl Component for ApiApp {
    type Message = ApiAppMsg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // Load all systems on initialization
        let link = ctx.link().clone();
        spawn_local(async move {
            match MockApiClient::fetch_all_systems().await {
                Ok(systems) => {
                    link.send_message(ApiAppMsg::SystemsLoaded(systems));
                }
                Err(e) => {
                    link.send_message(ApiAppMsg::LoadError(e.to_string()));
                }
            }
        });

        Self {
            systems: vec![],
            selected_system: None,
            loading: true,
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ApiAppMsg::SelectSystem(name) => {
                self.loading = true;
                self.error = None;

                // Fetch the selected system
                let link = ctx.link().clone();
                spawn_local(async move {
                    match MockApiClient::fetch_system(&name).await {
                        Ok(system) => {
                            link.send_message(ApiAppMsg::SystemLoaded(system));
                        }
                        Err(e) => {
                            link.send_message(ApiAppMsg::LoadError(e.to_string()));
                        }
                    }
                });

                true
            }
            ApiAppMsg::SystemsLoaded(systems) => {
                self.loading = false;

                // Select the first system by default
                if let Some(first_system) = systems.first() {
                    self.selected_system = Some(first_system.clone());
                }

                self.systems = systems;
                true
            }
            ApiAppMsg::SystemLoaded(system) => {
                self.loading = false;
                self.selected_system = Some(system);
                true
            }
            ApiAppMsg::LoadError(error) => {
                self.loading = false;
                self.error = Some(error);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_select = ctx.link().callback(ApiAppMsg::SelectSystem);

        html! {
            <div class="app">
                <div class="app-content">
                    <aside class="sidebar">
                        {
                            if self.loading && self.systems.is_empty() {
                                html! { <div class="loading">{"Loading systems..."}</div> }
                            } else {
                                // Convert SystemData to SystemConfig for SystemSelector
                                let legacy_systems: Vec<SystemConfig> = self.systems.iter().map(|sys| {
                                    SystemConfig {
                                        name: sys.system_name.clone(),
                                        display_name: sys.display_name.clone(),
                                        node_count: sys.node_count,
                                        k_notation: sys.k_notation.clone(),
                                        description: sys.description.clone(),
                                        color_scheme: crate::core::system_config::ColorScheme {
                                            nodes: sys.color_scheme.nodes.clone(),
                                            edges: sys.color_scheme.edges.clone(),
                                            selected_node: sys.color_scheme.selected_node.clone(),
                                            selected_edge: sys.color_scheme.selected_edge.clone(),
                                        },
                                    }
                                }).collect();

                                let selected_name = self.selected_system
                                    .as_ref()
                                    .map(|s| s.system_name.clone())
                                    .unwrap_or_else(|| "monad".to_string());

                                html! {
                                    <SystemSelector
                                        systems={ legacy_systems }
                                        selected={ selected_name }
                                        on_select={ on_select }
                                    />
                                }
                            }
                        }
                    </aside>

                    <main class="main-view">
                        {
                            if let Some(ref error) = self.error {
                                html! {
                                    <div class="error">
                                        <h2>{"Error"}</h2>
                                        <p>{ error }</p>
                                    </div>
                                }
                            } else if self.loading {
                                html! { <div class="loading">{"Loading system..."}</div> }
                            } else if let Some(ref system) = self.selected_system {
                                html! { <ApiGraphView system={ system.clone() } /> }
                            } else {
                                html! { <div class="loading">{"Select a system"}</div> }
                            }
                        }
                    </main>
                </div>
            </div>
        }
    }
}
