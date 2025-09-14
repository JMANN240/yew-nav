use yew::prelude::*;
use yew_router::{Routable, components::Link, hooks::use_route};

#[hook]
pub fn use_is_active_route<R: Routable + 'static>(to: &R) -> bool {
    use_route::<R>().is_some_and(|route| route == *to)
}

#[derive(Properties, PartialEq)]
pub struct NavLinkProps<R: PartialEq> {
    pub to: R,
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
    pub classes: Classes,

    #[prop_or(classes!("flex", "items-center", "gap-4"))]
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
        <nav class={classes!("flex", "justify-between", "items-center", classes.clone())}>
            <div class={container_classes.clone()}>
                { left_nav_links.clone() }
            </div>
            <div class={container_classes.clone()}>
                { center_nav_links.clone() }
            </div>
            <div class={container_classes.clone()}>
                { right_nav_links.clone() }
            </div>
        </nav>
    }
}
