use iocraft::prelude::*;

use crate::{fight::Enemy};

#[component]
fn EnemyBox(props: &Enemy) -> impl Into<AnyElement<'static>> {
    element! {
        View(
            flex_grow: 1.0,
        ) {
            Text(
                content: props.name,
                weight: Weight::Bold,
            )
        }
    }
}