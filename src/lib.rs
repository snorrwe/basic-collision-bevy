pub mod prelude;
pub mod primitives;

use bevy::{prelude::*, transform::TransformSystem};

/// Bitmask of layers. Each bit represents a different collision layer
pub type LayerMask = u32;

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct CollisionFilter {
    pub self_layers: LayerMask,
    pub collisions_mask: LayerMask,
}

impl CollisionFilter {
    /// checks self layers against other collision
    ///
    /// not commutative!
    #[inline]
    pub fn collides(self, other: CollisionFilter) -> bool {
        (self.self_layers & other.collisions_mask) != 0
    }
}

#[derive(Debug, Resource)]
struct AABBArray {
    aabbs: Vec<(Entity, AABB, CollisionFilter)>,
    sort_axis: usize,
}

#[derive(Clone, Debug, Copy, Event)]
pub struct AABBCollision {
    pub entity1: Entity,
    pub entity2: Entity,
}

/// Relative to the entity
#[derive(Default, Clone, Copy, Debug, Component)]
pub struct AABB {
    pub min: Vec2,
    pub max: Vec2,
}

#[derive(Default, Clone, Debug, Bundle)]
pub struct AABBBundle {
    pub aabb: AABB,
    pub filter: CollisionFilter,
    pub transform: TransformBundle,
}

/// Updated by [CollisionPlugin](CollisionPlugin).
#[derive(Default, Clone, Debug, Component, Deref, DerefMut)]
pub struct AABBGlobal(pub AABB);

fn aabb_sort_sweep_system(
    mut e: EventWriter<AABBCollision>,
    mut collection: ResMut<AABBArray>,
    mut tick: Local<u64>,
    mut collisions_to_send: Local<Vec<AABBCollision>>,
    q: Query<(Entity, &AABBGlobal, &CollisionFilter)>,
) {
    *tick += 1;

    // rebuild the array
    collection.aabbs.clear();
    for (e, a, f) in q.iter() {
        collection.aabbs.push((e, a.0, *f));
    }

    if collection.aabbs.is_empty() {
        return;
    }

    // sort the array by the current axis
    let sort_axis = collection.sort_axis;
    collection
        .aabbs
        .sort_by(move |(_, aabb1, _), (_, aabb2, _)| {
            aabb1.min[sort_axis]
                .partial_cmp(&aabb2.min[sort_axis])
                .unwrap_or(std::cmp::Ordering::Equal)
        });

    // sweep for collisions
    let mut s = Vec2::ZERO;
    let mut s2 = Vec2::ZERO;
    collisions_to_send.clear();
    for (i, (e1, aabb1, f1)) in collection.aabbs.iter().enumerate() {
        let p = (aabb1.max + aabb1.min) * 0.5;
        // update sums
        s += p;
        s2 += p * p;
        // test collisions against all posible overlapping AABBs, following current one
        for (e2, aabb2, f2) in collection.aabbs[i + 1..].iter() {
            // stop when tested AABBs are beyond the end of the current AABB
            if aabb2.min[sort_axis] > aabb1.max[sort_axis] {
                break;
            }

            // `collides` is not commutative
            if (f1.collides(*f2) || f2.collides(*f1)) && primitives::aabb_aabb(aabb1, aabb2) {
                collisions_to_send.push(AABBCollision {
                    entity1: *e1,
                    entity2: *e2,
                });
            }
        }
    }

    e.send_batch(collisions_to_send.iter().copied());

    let variance = s2 - s * s; // no need to divide by N as we only check for the greater
                               // axis

    // update sorting axis to be the one with the greatest variance
    collection.sort_axis = (variance.y > variance.x) as usize;
}

fn update_aabbs(mut q: Query<(&mut AABBGlobal, &AABB, &GlobalTransform)>) {
    q.par_iter_mut().for_each(|(mut out, aabb, tr)| {
        let AABB { min, max } = *aabb;
        debug_assert!(min.x <= max.x, "assumes min({:?}) <= max({:?})", min, max);
        debug_assert!(min.y <= max.y, "assumes min({:?}) <= max({:?})", min, max);

        let a = tr.transform_point(min.extend(0.0));
        let b = tr.transform_point(max.extend(0.0));

        for i in 0..2 {
            out.min[i] = a[i].min(b[i]);
            out.max[i] = a[i].max(b[i]);
        }
    });
}

pub struct CollisionPlugin;

/// Collistion stages
#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
enum Labels {
    UpdateBoxes,
    Sweep,
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AABBArray {
            aabbs: Vec::with_capacity(4096),
            sort_axis: 0,
        })
        .add_event::<AABBCollision>()
        .add_systems(
            PostUpdate,
            (
                update_aabbs
                    .after(TransformSystem::TransformPropagate)
                    .in_set(Labels::UpdateBoxes),
                aabb_sort_sweep_system
                    .after(Labels::UpdateBoxes)
                    .in_set(Labels::Sweep),
            ),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_collides_test() {
        let a = CollisionFilter {
            self_layers: 1,
            collisions_mask: 2 | 4 | 8,
        };
        let b = CollisionFilter {
            self_layers: 2,
            collisions_mask: 1,
        };

        assert!(a.collides(b));
        assert!(b.collides(a));
    }

    #[test]
    fn collides_is_not_commutative_test() {
        let a = CollisionFilter {
            self_layers: 0,
            collisions_mask: 2,
        };
        let b = CollisionFilter {
            self_layers: 2,
            collisions_mask: 0,
        };

        assert!(b.collides(a));
        assert!(!a.collides(b));
    }
}
