use bevy::prelude::*;

use super::gen::GreacherInfo;

#[derive(Component, Deref, DerefMut)]
pub struct GreacherBodyAnimation(pub Timer);

pub fn animate_greacher_body(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query_child: Query<(
        &Parent,
        &mut GreacherBodyAnimation,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
    query_parent: Query<&GreacherInfo>,
) {
    for (parent, mut timer, mut sprite, texture_atlas_handle) in &mut query_child {
        let parent = query_parent.get(parent.get());

        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
