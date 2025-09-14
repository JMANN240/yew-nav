use yew::prelude::*;
use yew_router::{Routable, components::Link, hooks::use_route};

#[hook]
pub fn use_is_active_route<R: Routable + 'static>(to: &R) -> bool {
    use_route::<R>().is_some_and(|route| route == *to)
}

#[derive(Properties, PartialEq)]
pub struct NavLinkProps<R: PartialEq> {
    pub to: R,

    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub inactive_classes: Classes,

    #[prop_or_default]
    pub active_classes: Classes,

    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn NavLink<R: Routable + 'static>(
    NavLinkProps {
        to,
        classes,
        inactive_classes,
        active_classes,
        children,
    }: &NavLinkProps<R>,
) -> Html {
    let is_active_route = use_is_active_route(to);

    html! {
        <Link<R>
            classes={classes!(classes.clone(), (!is_active_route).then_some(inactive_classes.clone()), is_active_route.then_some(active_classes.clone()))}
            to={to.clone()}
        >
            { children.clone() }
        </Link<R>>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavBarProps {
    #[prop_or_default]
    pub classes: Classes,

    #[prop_or_default]
    pub container_classes: Classes,

    #[prop_or_default]
    pub left_nav_links: Html,

    #[prop_or_default]
    pub center_nav_links: Html,

    #[prop_or_default]
    pub right_nav_links: Html,
}

#[function_component]
pub fn NavBar(
    NavBarProps {
        classes,
        container_classes,
        left_nav_links,
        center_nav_links,
        right_nav_links,
    }: &NavBarProps,
) -> Html {
    html! {
        <nav style="display: flex; justify-content: space-between; align-items: center;" class={classes!(classes.clone())}>
            <div style="display: flex; align-items: center;" class={classes!(container_classes.clone())}>
                { left_nav_links.clone() }
            </div>
            <div style="display: flex; align-items: center;" class={classes!(container_classes.clone())}>
                { center_nav_links.clone() }
            </div>
            <div style="display: flex; align-items: center;" class={classes!(container_classes.clone())}>
                { right_nav_links.clone() }
            </div>
        </nav>
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NavState {
	pub shown: bool
}

pub enum NavStateAction {
    Open,
    Close,
    Toggle,
}

impl Reducible for NavState {
    type Action = NavStateAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            NavStateAction::Open => NavState { shown: true },
            NavStateAction::Close => NavState { shown: false },
            NavStateAction::Toggle => NavState { shown: !self.shown },
        }
        .into()
    }
}

pub type NavStateContext = UseReducerHandle<NavState>;

#[derive(Properties, Debug, PartialEq)]
pub struct NavStateProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn NavStateProvider(props: &NavStateProviderProps) -> Html {
    let nav_state_reducer = use_reducer(|| NavState { shown: false });

    html! {
        <ContextProvider<NavStateContext> context={nav_state_reducer}>
            {props.children.clone()}
        </ContextProvider<NavStateContext>>
    }
}
