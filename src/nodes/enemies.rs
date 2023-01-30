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
pub enum ItemType {
    Bird = 1,
}

pub struct Enemy {
    pub pos: Vec2,
    pub item_type: ItemType,
    visual_scale: f32,
}

impl Enemy {
    pub fn new(pos: Vec2, item_type: ItemType) -> Enemy {
        Enemy {
            pos,
            visual_scale: 1.0,
            item_type,
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

        let mut flipped: bool = false;
        if rand::gen_range(0.0,1.0) > 0.5 {
            flipped = true; 
        }

        match node.item_type {
            ItemType::Bird => draw_texture_ex(
                resources.bird, 
                node.pos.x, 
                node.pos.y, 
                WHITE,
                DrawTextureParams { 
                    dest_size: Some(vec2(32.*node.visual_scale,32.*node.visual_scale)), 
                    source: None, 
                    rotation: 0.0, 
                    flip_x: false, 
                    flip_y: false, 
                    pivot: None }
            ),
        }
    }
}
