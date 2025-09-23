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
    pub main_nav_link: Html,

    #[prop_or_default]
    pub left_nav_links: Html,

    #[prop_or_default]
    pub center_nav_links: Html,

    #[prop_or_default]
    pub right_nav_links: Html,
}

#[macro_export]
macro_rules! navbar_classes {
    () => {
        ::yew::classes!("flex", "justify-between", "items-center")
    };
}

#[macro_export]
macro_rules! navcontainer_classes {
    () => {
        ::yew::classes!("flex", "items-center", "gap-4")
    };
}

#[function_component]
pub fn NavBar(
    NavBarProps {
        classes,
        container_classes,
        main_nav_link,
        left_nav_links,
        center_nav_links,
        right_nav_links,
    }: &NavBarProps,
) -> Html {
    html! {
        <nav class={classes!(classes.clone())}>
            { main_nav_link.clone() }
            <div class={classes!(container_classes.clone())}>
                { left_nav_links.clone() }
            </div>
            <div class={classes!(container_classes.clone())}>
                { center_nav_links.clone() }
            </div>
            <div class={classes!(container_classes.clone())}>
                { right_nav_links.clone() }
            </div>
            <NavMenuButton />
        </nav>
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct NavMenuState {
	pub shown: bool
}

pub enum NavMenuStateAction {
    Open,
    Close,
    Toggle,
}

impl Reducible for NavMenuState {
    type Action = NavMenuStateAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            NavMenuStateAction::Open => NavMenuState { shown: true },
            NavMenuStateAction::Close => NavMenuState { shown: false },
            NavMenuStateAction::Toggle => NavMenuState { shown: !self.shown },
        }
        .into()
    }
}

pub type NavMenuStateContext = UseReducerHandle<NavMenuState>;

#[derive(Properties, Debug, PartialEq)]
pub struct NavMenuStateProviderProps {
    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn NavMenuStateProvider(props: &NavMenuStateProviderProps) -> Html {
    let nav_state_reducer = use_reducer(|| NavMenuState { shown: false });

    html! {
        <ContextProvider<NavMenuStateContext> context={nav_state_reducer}>
            {props.children.clone()}
        </ContextProvider<NavMenuStateContext>>
    }
}

#[derive(Properties, PartialEq)]
pub struct NavMenuButtonProps {
    #[prop_or_default]
    pub classes: Classes,
}

#[function_component]
pub fn NavMenuButton(NavMenuButtonProps { classes }: &NavMenuButtonProps) -> Html {
    html! {
        <button class={classes!(classes.clone())}>{ "TOGGLE" }</button>
    }
}
