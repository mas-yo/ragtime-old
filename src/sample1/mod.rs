use entity_component::component::*;
use entity_component::entity::*;
use entity_component::system::*;
use super::sub_component::game_logic::connection_manager::*;
use super::sub_component::game_logic::db_manager::*;
use super::sub_component::game_logic::game_object_manager::*;
use super::sub_component::game_object::position::*;
use super::string_message::StringMessage;

enum GameLogicComponents {
    ConnectionManagerSC(ConnectionManager<StringMessage, GameObjectComponents>),
    DBManagerSC(DBManager),
    GameObjectManagerSC(GameObjectManager<GameObjectComponents>),
}

impl SubComponent for GameLogicComponents {
    fn start(&mut self) {

    }
    fn update(&mut self) {

    }
}

enum GameObjectComponents {
    PositionSC(Position),
}

impl HandleMessage<StringMessage> for GameObjectComponents {
    fn on_message(&mut self, msg:&StringMessage ) {

    }
}

impl SubComponent for GameObjectComponents {
    fn start(&mut self) {

    }
    fn update(&mut self) {

    }
}

struct Sample1Game {
    system:System<GameLogicComponents>,
}

pub fn sample1_start() {
    let mut system = Sample1Game{system:System::<GameLogicComponents>::new()};
    let conn = GameLogicComponents::ConnectionManagerSC(ConnectionManager::<StringMessage,GameObjectComponents>::new("127.0.0.1:8080".to_string()));
    let conn = Component::new( system.system.generate_entity_id(), conn);

    //make game AsMut
    //add connection manager
    //add game object manager

}
