use macroquad::{
    experimental::{
        collections::storage,
        coroutines::{start_coroutine, wait_seconds},
        scene::{self, Handle, RefMut},
    },
    prelude::*,
};

use crate::{
    nodes::{
        pickup::ItemType,
        player::{Fish, Weapon},
    },
    Resources,
};

pub struct RemotePlayer {
    pub username: String,
    pub id: String,
    fish: Fish,

    pub dead: bool,
    pub ready: bool,
    pos_delta: Vec2,
    last_move_time: f64,
}

impl RemotePlayer {
    pub fn new(username: &str, id: &str) -> RemotePlayer {
        let pos = vec2(100., 105.);

        RemotePlayer {
            fish: Fish::new(pos),
            username: username.to_string(),
            id: id.to_string(),
            pos_delta: vec2(0.0, 0.0),
            last_move_time: 0.0,
            ready: false,
            dead: false,
        }
    }

    pub fn pick_weapon(&mut self, item_type: ItemType) {
        self.fish.pick_weapon(item_type)
    }

    pub fn disarm(&mut self) {
        self.fish.disarm()
    }

    pub fn weapon(&self) -> Option<ItemType> {
        match self.fish.weapon {
            None => None,
            Some(Weapon::Gun { .. }) => Some(ItemType::Gun),
            Some(Weapon::Sword) => Some(ItemType::Sword),
        }
    }

    pub fn set_pos(&mut self, pos: Vec2) {
        self.last_move_time = get_time();
        self.pos_delta = pos - self.fish.pos();
        self.fish.set_pos(pos);
    }

    pub fn set_facing(&mut self, facing: bool) {
        self.fish.set_facing(facing);
    }

    pub fn set_dead(&mut self, dead: bool) {
        self.dead = dead;
    }

    pub fn pos(&self) -> Vec2 {
        self.fish.pos()
    }

    pub fn shoot(&mut self, handle: Handle<Self>) {
        match self.fish.weapon {
            Some(Weapon::Gun { .. }) => {
                let mut bullets = scene::find_node_by_type::<crate::nodes::Bullets>().unwrap();
                bullets.spawn_bullet(self.pos(), self.fish.facing());
            }
            Some(Weapon::Sword) => {
                let swing = async move {
                    {
                        if let Some(mut node) = scene::try_get_node(handle) {
                            node.fish.sword_sprite.set_animation(1);
                        }
                    }

                    for i in 0u32..3 {
                        {
                            if let Some(mut node) = scene::try_get_node(handle) {
                                node.fish.sword_sprite.set_frame(i);
                            }
                        }

                        wait_seconds(0.08).await;
                    }

                    {
                        if let Some(mut node) = scene::try_get_node(handle) {
                            node.fish.sword_sprite.set_animation(0);
                        }
                    }
                };
                start_coroutine(swing);
            }
            None => {
                println!("well");
            }
        }
    }
}
impl scene::Node for RemotePlayer {
    fn draw(mut node: RefMut<Self>) {
        draw_text_ex(
            &node.username,
            node.fish.pos().x - 1.,
            node.fish.pos().y - 1.,
            TextParams {
                font_size: 50,
                font_scale: 0.25,
                ..Default::default()
            },
        );

        node.fish.draw();
    }

    fn update(mut node: RefMut<Self>) {
        if node.dead {
            let resources = storage::get::<Resources>();
            let on_ground = resources
                .collision_world
                .collide_check(node.fish.collider, node.fish.pos() + vec2(0., 1.));

            if on_ground {
                node.fish.set_animation(3);
            } else {
                node.fish.set_animation(2);
            }
        } else if get_time() - node.last_move_time > 0.2 || node.pos_delta.length() < 0.01 {
            node.fish.set_animation(0);
        } else {
            node.fish.set_animation(1);
        }
    }
}
