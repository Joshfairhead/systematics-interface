use yew::prelude::*;
use crate::core::system_config::SystemConfig;
use crate::components::graph_view::GraphView;
use crate::components::system_selector::SystemSelector;

pub enum AppMsg {
    SelectSystem(String),
}

pub struct App {
    systems: Vec<SystemConfig>,
    selected_system: String,
}

impl Component for App {
    type Message = AppMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            systems: SystemConfig::get_all_systems(),
            selected_system: "monad".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMsg::SelectSystem(name) => {
                self.selected_system = name;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let selected_config = SystemConfig::get_by_name(&self.selected_system)
            .unwrap_or_else(|| self.systems[0].clone());

        let on_select = ctx.link().callback(AppMsg::SelectSystem);

        html! {
            <div class="app">
                <div class="app-content">
                    <aside class="sidebar">
                        <SystemSelector
                            systems={ self.systems.clone() }
                            selected={ self.selected_system.clone() }
                            on_select={ on_select }
                        />
                    </aside>

                    <main class="main-view">
                        <GraphView system={ selected_config } />
                    </main>
                </div>
            </div>
        }
    }
}
