use std::collections::HashMap;

use crate::{game_objects::game_object::{BaseGameObjectProperties, GmObj}, sys::system::System};

#[derive(Clone)]
pub struct Character {
    base_properties: BaseGameObjectProperties,
    max_hp: i32,
    current_hp:i32,
    max_energy: i32,
    current_energy: i32,
    deck_identifier: String,
    player: bool,
    start: fn(&mut System),
    update: fn(&mut System),
    stop: fn(&mut System),
    action:  fn(&mut System),

}
impl Character {
    pub fn new(
        base_properties: BaseGameObjectProperties,
        max_hp: i32,
        current_hp:i32, 
        max_energy: i32,
        current_energy: i32,
        deck_identifier: String,
        player: bool,
        start: fn(&mut System),
        update: fn(&mut System),
        stop: fn(&mut System),
        action:  fn(&mut System),

    ) -> Self {
      
        Character {
            base_properties,
            max_hp,
            current_hp,
            current_energy,
            max_energy,
            deck_identifier,
            player,
            start,
            update,
            stop,
            action
        }
    }
    pub fn character_from_sprites(
        position: [f32; 3],
        texture_paths: HashMap<String, String>,
        default_texture:String,
        max_hp: i32,
        current_hp:i32,
        max_energy: i32,
        current_energy: i32,
        deck_identifier: String,
        player: bool,
        z_index:i32,
        should_render: bool,
        start: fn(&mut System),
        update: fn(&mut System),
        stop: fn(&mut System),
        action:  fn(&mut System),
    ) -> Self {
        let base_properties =
            BaseGameObjectProperties::game_object_from_sprites(
                position,
                texture_paths,
                default_texture,
                z_index,
                should_render,
                false,
            );
        Character {
            base_properties,
            max_hp,
            current_hp,
            max_energy,
            current_energy,
            deck_identifier,
            player,
            start,
            update,
            stop,
            action
        }
    }
    pub fn max_hp(&self) -> i32 {
        self.max_hp
    }
    pub fn set_max_hp(mut self, max_hp: i32){
        self.max_hp = max_hp;
    } 
    pub fn player(&self) -> bool {
        self.player
    }

    pub fn set_player(mut self, player: bool){
        self.player = player;
    }


    pub fn current_hp(&self) -> i32 {
        self.current_hp
    }
    pub fn set_current_hp(mut self, hp: i32){
        self.current_hp = hp;
    }

    pub fn max_energy(&self) -> i32 {
        self.max_energy
    }
    pub fn set_max_energy(mut self, max_hp: i32){
        self.max_energy = max_hp;
    }

    pub fn current_energy(&self) -> i32 {
        self.current_energy
    }
    pub fn set_current_energy(mut self, energy: i32){
        self.current_energy = energy;
    }
    pub fn deck_identifier(&self)->&String{
        &self.deck_identifier
    }
    pub fn deck_identifier_mut(&mut self)->&mut String{
        &mut self.deck_identifier
    }
}
impl GmObj for Character {
    fn base_properties(&self) -> &BaseGameObjectProperties {
        &self.base_properties
    }
    fn base_properties_mut(&mut self) -> &mut BaseGameObjectProperties {
        &mut self.base_properties
    }
    fn start(&mut self) -> fn(&mut System) {
        self.start
    }
    fn update(&mut self) -> fn(&mut System) {
        self.update
    }
    fn stop(&mut self) -> fn(&mut System) {
        self.stop
    }
    fn action(&mut self) -> fn(&mut System) {
        self.action
    }
}