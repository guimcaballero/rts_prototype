use crate::systems::{bullet::*, enemy::*, unit::*};
use bevy::{math::Vec3, prelude::*};

pub struct Ranged {
    pub range: f32,        // Range that the enemy needs to be in before it fires
    pub attack_speed: u16, // Number of attacks per second
    pub last_attack: f64,
}
impl Default for Ranged {
    fn default() -> Self {
        Self {
            range: 20.,
            attack_speed: 1,
            last_attack: 0.,
        }
    }
}

impl Ranged {
    fn can_shoot(&self, current_time: f64) -> bool {
        self.last_attack + (1. / self.attack_speed as f64) < current_time
    }
}

fn shoot_against_enemies(
    mut commands: Commands,
    time: Res<Time>,
    bullet_resource: Res<BulletMeshResource>,
    mut own_query: Query<Without<Enemy, (&Unit, &mut Ranged, &Transform)>>,
    mut enemy_query: Query<(&Unit, &Enemy, &Transform)>,
) {
    for (_, mut ranged, transform) in &mut own_query.iter() {
        let translation = transform.translation();
        if ranged.can_shoot(time.seconds_since_startup) {
            // Get the closest enemy
            let mut enemy: Option<(Vec3, f32)> = None; // Option with (difference_vector, difference_distance)
            for (_, _, enemy_transform) in &mut enemy_query.iter() {
                let difference = translation - enemy_transform.translation();
                let difference_distance = difference.length();

                // If it's in range, we check if it's closer or the first enemy
                if difference_distance < ranged.range {
                    if let Some((_, distance)) = enemy {
                        if difference_distance < distance {
                            enemy = Some((difference, difference_distance));
                        }
                    } else {
                        enemy = Some((difference, difference_distance));
                    }
                }
            }

            // If there is a closest enemy, we shoot
            if let Some((vector, _)) = enemy {
                Bullet::new(
                    &mut commands,
                    &bullet_resource,
                    translation,
                    -vector.normalize(),
                );

                ranged.last_attack = time.seconds_since_startup;
            }
        }
    }
}

pub struct AttackPlugin;
impl Plugin for AttackPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(shoot_against_enemies.system());
    }
}
