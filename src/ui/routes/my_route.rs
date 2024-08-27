use crate::*;

#[derive(Component)]
pub struct MyRoute;

/// System that builds the route when the component is added
fn build_route(
    mut commands: Commands,
    query: Query<Entity, Added<MyRoute>>,
) {
    for route_entity in &query {
        // Make our route a spatial entity
        commands
            .entity(route_entity)
            .insert(SpatialBundle::default())
            .with_children(|route| {
                // Spawn some additional non UI components if you need to.

                // Here you can spawn the UI
                route
                    .spawn((
                        UiTreeBundle::<MainUi>::from(UiTree::new2d("MyRoute")),
                        MovableByCamera,
                    ))
                    .with_children(|ui| {
                        // Spawn the root div
                        let root = UiLink::<MainUi>::path("Root");  // Here we can define the name of the node
                        ui.spawn((
                            root.clone(),                           // Here we add the link
                            UiLayout::window_full().pack::<Base>(),         // This is where we define layout
                        ));

                        // Spawn the board
                        let board = root.add("Solid");
                        ui.spawn((
                            board.clone(),
                            UiLayout::solid().size((1920.0, 1080.0)).pack::<Base>(), // Just different layout type that preserves aspect ratio
                        ));

                        let board = board.add("Board");
                        ui.spawn((
                            board.clone(),
                            UiLayout::window().x(Rl(50.0)).anchor(Anchor::TopCenter).size(Rl(105.0)).pack::<Base>(),
                        ));

                        // #=========================#
                        // #=== MAIN MENU BUTTONS ===#

                        // Spawn button boundary
                        let list = board.add("List");
                        ui.spawn((
                            list.clone(),
                            UiLayout::window().pos(Rl((5.0, 5.0))).size(Rl((7.5, 40.0))).pack::<Base>(),
                        ));

                        // Spawn buttons
                        let gap = 3.0;
                        let size = 14.0;
                        let mut offset = 0.0;
                        for button in [MainMenuButton::Start, MainMenuButton::Exit] {
                            ui.spawn((
                                // Link the entity
                                list.add(button.str()),

                                // Add the button type
                                button.clone(),

                                // Add layout
                                UiLayout::window().y(Rl(offset)).size(Rl((100.0, size))).pack::<Base>(),

                                // Add the button component
                                MainButton { text: button.str() },
                            ));

                            offset += gap + size;
                        }
                    });
            });
    }
}

/// Good practice is to use custom component for buttons, so we can easily know what type of button was pressed
#[derive(Component, Clone, PartialEq)]
enum MainMenuButton {
    Start,
    Exit,
}
impl MainMenuButton {
    fn str(&self) -> String {
        match self {
            MainMenuButton::Start => "START".into(),
            MainMenuButton::Exit => "EXIT".into(),
        }
    }
}

pub struct MyRoutePlugin;
impl Plugin for MyRoutePlugin {
    fn build(&self, app: &mut App) {
        app
            // NOTE! Systems changing the UI need to run before UiSystems::Compute
            // or they will not get picked up by change detection.
            .add_systems(Update, build_route.before(UiSystems::Compute));
    }
}
