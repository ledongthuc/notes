use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(HelloPlugin)
        .add_startup_system(setup_shapes)
        .run();
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Rect;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Thuc".to_string())));
    commands.spawn((Person, Name("Dong".to_string())));
    commands.spawn((Person, Name("Le".to_string())));
}

#[derive(Resource)]
struct GreetTimer(Timer);

fn hello_people(time: Res<Time>, mut timer: ResMut<GreetTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in query.iter() {
            println!("hello {}!", name.0)
        }
    }
}

fn move_rect (
    keyboard: Res<Input<KeyCode>>,
    mut rect : Query<(&mut Transform,), With<Rect>>,
) {
    if !keyboard.just_pressed(KeyCode::Up) { return; }

    if let Ok(rect) = rect.get_single_mut() {
        let (mut player_position,) = rect;
        player_position.translation.x += 1.0;
    }
}

struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
            .add_startup_system(add_people)
            .add_system(hello_people);
    }
}

fn setup_shapes(
    mut commands: Commands,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle{
        sprite: Sprite { 
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        ..default()
    });
}
