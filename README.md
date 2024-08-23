# basic-collision-bevy

Simple 2D AABB collisions plugin for the Bevy game engine.

Usage:

```toml
# Cargo.toml
basic-collision-bevy = { git="https://github.com/snorrwe/basic-collision-bevy.git" }
```

```rs
use basic_collision_bevy::prelude::*;

// in your game plugin
fn build(app: &mut App) {
    app.add_plugin(CollisionPlugin);
}

// add AABBBundle to entities
fn setup(mut commands: Commands) {
    commands.spawn(AABBBundle {
        aabb: AABB {
            // ...
        },
        filter: CollisionFilter {
            self_layers: 1 | (1<<2),
            collisions_mask: (1<<2) | (1<<3),
        },
        transform: // ...
    });
}

fn handle_collision_system(mut collisions: EventReader<AABBCollision>) {
    for collision_event in collisions.read() {
        // handle collision
    }
}
```
