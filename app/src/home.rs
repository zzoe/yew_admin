use ybc::{Column, ColumnProps, Columns, ColumnsProps};
use ybc::{Container, ContainerProps};
use ybc::Hero;
use ybc::HeroProps;
use ybc::HeroSize::*;
use ybc::TileCtx::*;
use ybc::TileSize::Three;
use yew::prelude::*;

pub struct Home {
    pub props: HomeProps,
    pub link: ComponentLink<Self>,
    pub label: String,
}

#[derive(Clone, Debug, Properties, PartialEq)]
pub struct HomeProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Home {
    type Message = ();
    type Properties = HomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link, label: "Menu label".to_owned() }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <ybc::Container fluid=true>
                <ybc::Columns>
                    <ybc::Column classes=classes!("is-narrow")>
                        <ybc::Menu classes=classes!("menu")>
                            <p class="menu-label">{ &*self.label }</p>
                            <ul class="menu-list">
                                <li><a>{"sldkfjdf"}</a></li>
                                <li><a>{"sldkfjdf"}</a></li>
                            </ul>
                        </ybc::Menu>
                    </ybc::Column>
                    <ybc::Column>
                        <ybc::Hero fixed_nav=true size=FullheightWithNavbar body=html!{
                            <p>{"Lorem ipsum dolor sit amet ..."}</p>
                        } />
                    </ybc::Column>
                </ybc::Columns>
                // <ybc::Tile ctx=Ancestor>
                //   <ybc::Tile ctx=Parent vertical=true size=Three>
                //     <ybc::Tile ctx=Child>
                //
                //     </ybc::Tile>
                //   </ybc::Tile>
                //   <ybc::Tile ctx=Parent vertical=true>
                //     <ybc::Tile ctx=Child classes=classes!("box")>
                //
                //     </ybc::Tile>
                //   </ybc::Tile>
                // </ybc::Tile>
            </ybc::Container>
        }
    }
}