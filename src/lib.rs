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

#[derive(Clone, Debug, PartialEq)]
pub struct NavMenuState {
    pub shown: bool,
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

    #[prop_or_default]
    pub children: Html,
}

#[function_component]
pub fn NavMenuButton(NavMenuButtonProps { classes, children }: &NavMenuButtonProps) -> Html {
    let nav_menu_state_context =
        use_context::<NavMenuStateContext>().expect("no nav menu state context found");

    let on_click = {
        let nav_menu_state_context = nav_menu_state_context.clone();

        Callback::from(move |_| {
            nav_menu_state_context.dispatch(NavMenuStateAction::Toggle);
        })
    };

    html! {
        <button
            onclick={on_click}
            class={classes!(classes.clone())}
        >
            { children.clone() }
        </button>
    }
}

#[hook]
pub fn use_hide_nav_menu<T>(deps: T)
where
	T: PartialEq + 'static
{
	let nav_menu_state_context = use_context::<NavMenuStateContext>().expect("no nav menu state found");

	use_effect_with(deps, move |_| {
		nav_menu_state_context.dispatch(NavMenuStateAction::Close);
	});
}
