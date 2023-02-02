use macroquad::{
    experimental::{
        collections::storage,
        coroutines::{start_coroutine, wait_seconds},
        scene::{self, RefMut},
    },
    prelude::*,
};

use crate::Resources;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum EnemyType {
    Bird = 1,
}

pub struct Enemy {
    pub hp: f32,
    pub pos: Vec2,
    pub direction: Vec2,
    pub enemy_type: EnemyType,
    visual_scale: f32,
    flipped: bool,
}

impl Enemy {
    pub fn new(pos: Vec2, enemy_type: EnemyType) -> Enemy {
        let mut flipped: bool = false;
        if rand::gen_range(0.0,1.0) > 0.5 {
            flipped = true; 
        }
        let direction: Vec2 = vec2(
            rand::gen_range(0.0,1.0)*6 as f32,
            rand::gen_range(0.0,1.0)*6 as f32
        );

        let hp = pos[0];
        Enemy {
            hp,
            pos,
            direction,
            visual_scale: 1.0,
            enemy_type,
            flipped,
        }
    }
}

impl scene::Node for Enemy {
    fn ready(node: RefMut<Self>) {
        let handle = node.handle();

        start_coroutine(async move {
            let n = 25;
            for i in 0..n {
                // if player pick up the item real quick - the node may be already removed here
                if let Some(mut this) = scene::try_get_node(handle) {
                    this.visual_scale =
                        1.0 + (i as f32 / n as f32 * std::f32::consts::PI).sin() * 3.0;
                }

                next_frame().await;
            }
        });

        start_coroutine(async move {
            loop {
                wait_seconds(0.05).await;
                if let Some(mut this) = scene::try_get_node(handle) {
                    this.pos.x += this.direction.x;
                    this.pos.y += this.direction.y;
                }
                next_frame().await;
            }
        });

        start_coroutine(async move {
            wait_seconds(10.).await;

            let n = 10;
            for _ in 0..n {
                if let Some(mut this) = scene::try_get_node(handle) {
                    this.visual_scale -= 1.0 / n as f32;
                }
                next_frame().await;
            }

            if let Some(this) = scene::try_get_node(handle) {
                this.delete();
            }
        });
    }

    fn draw(node: RefMut<Self>) {
        let resources = storage::get_mut::<Resources>();

        match node.enemy_type {
            EnemyType::Bird => draw_texture_ex(
                resources.bird, 
                node.pos.x, 
                node.pos.y, 
                WHITE,
                DrawTextureParams { 
                    dest_size: Some(vec2(32.*node.visual_scale,32.*node.visual_scale)), 
                    source: None, 
                    rotation: 0.0, 
                    flip_x: node.flipped, 
                    flip_y: false, 
                    pivot: None }
            ),
        }
    }
}
