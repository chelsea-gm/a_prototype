/**
 * Chelsea Meyers
 *
 * a_prototype - Mockup prototype of game user interface, intended for testing purposes
 *
 * Essentially a (heavily) simplified copy of the FFX battle interface, with a brief overworld
 * scene
 *
 **/

extern crate raylib;

use raylib::prelude::*;

/////////////
// DEFINES //
/////////////

// UIPhase - enumeration of all UI phases in the game
enum UIPhase
{
    Overworld,
    BattleActionSelect,
    BattleTargetSelect,
    BattleItemSelect,
    BattleDamageDisplay,
    BattleEnemyTurn,
    Victory,
}

// GraphicsSettings - data structure used to keep track of graphical display settings
struct GraphicsSettings
{
    screen_width: i32,
    screen_height: i32,
    target_fps: u32,
}

// OverworldState - data structure used to keep track of player position in the overworld
struct OverworldState
{
    player_x: i32,
    player_y: i32,
    player_width: i32,
    player_height: i32,

    enemy_x: i32,
    enemy_y: i32,
    enemy_width: i32,
    enemy_height: i32,
}

// GameState - data structure used to keep track of gameplay-related state info
struct GameState
{
    player_hp: i32,
    player_sp: i32,
    enemy_hp: i32,

    ui_phase: UIPhase,

    overworld_state: OverworldState,
}

//////////////////////////////
// STATE UPDATING FUNCTIONS //
//////////////////////////////

// TODO - key bindings should ultimately be mutable

// update_state_overworld - update state while player is on the overworld
fn update_state_overworld(game_state: &mut GameState, rl: &mut RaylibHandle)
{
    // check for player hitbox collision with enemy
    // TODO: check for hitbox collision

    // check if a movement key is down
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP)
    {
        // TODO: move player up
    }
    else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT)
    {
        // TODO: move player left
    }
    else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN)
    {
        // TODO: move player down
    }
    else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_D)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT)
    {
        // TODO: move player right
    }
}

// update_state_battle_action_select - update state while player is selecting a battle action
fn update_state_battle_action_select(game_state: &mut GameState, rl: &mut RaylibHandle)
{
    // TODO: update battle action select state
}

// update_state_battle_target_select - update state while player is selecting a target in battle
fn update_state_battle_target_select(game_state: &mut GameState, rl: &mut RaylibHandle)
{
    // TODO: update battle target select state
}

// update_state_battle_item_select - update state while player is selecting an item in battle
fn update_state_battle_item_select(game_state: &mut GameState, rl: &mut RaylibHandle)
{
    // TODO: update battle item select state
}

// update_state_battle_damage_display - update state while player is viewing damage in battle
fn update_state_battle_damage_display(game_state: &mut GameState, rl: &mut RaylibHandle)
{
    // TODO: update battle damage screen state
}

// update_state_battle_enemy_turn - update state while player is viewing enemy turn in battle
fn update_state_battle_enemy_turn(game_state: &mut GameState, rl: &mut RaylibHandle)
{
    // TODO: update battle enemy turn state
}

// update_state - check for keyboard input and timeouts (?), and update game state according to
// which UI phase the player is currently in
fn update_state(game_state: &mut GameState, rl: &mut RaylibHandle)
{
    match &game_state.ui_phase
    {
        UIPhase::Overworld => update_state_overworld(game_state, rl),
        UIPhase::BattleActionSelect => update_state_battle_action_select(game_state, rl),
        UIPhase::BattleTargetSelect => update_state_battle_target_select(game_state, rl),
        UIPhase::BattleItemSelect => update_state_battle_item_select(game_state, rl),
        UIPhase::BattleDamageDisplay => update_state_battle_damage_display(game_state, rl),
        UIPhase::BattleEnemyTurn => update_state_battle_enemy_turn(game_state, rl),
        UIPhase::Victory => (),
    }
}

///////////////////////
// DRAWING FUNCTIONS //
///////////////////////

// draw_stub - stub function for handling unsupported draw operations
// TODO - this is for debug purposes only; remove for release
// there's probably a way to do that through an annotation-like language feature or something...
fn draw_stub(dh: &mut RaylibDrawHandle)
{
    dh.clear_background(Color::GRAY);
}

// draw_overworld - draw the overworld scene
fn draw_overworld(dh: &mut RaylibDrawHandle)
{
    // TODO: draw overworld
}

// draw_battle_action_select - draw the action select phase of the battle interface
fn draw_battle_action_select(dh: &mut RaylibDrawHandle)
{
    // TODO: draw battle action select
}

// draw_battle_target_select - draw the target select phase of the battle interface
fn draw_battle_target_select(dh: &mut RaylibDrawHandle)
{
    // TODO: draw battle target select
}

// draw_battle_item_select - draw the item select phase of the battle interface
fn draw_battle_item_select(dh: &mut RaylibDrawHandle)
{
    // TODO: draw battle item select
}

// draw_battle_damage_display - draw the damage display phase of the battle interface
fn draw_battle_damage_display(dh: &mut RaylibDrawHandle)
{
    // TODO: draw battle damage display
}

// draw_battle_enemy_turn - draw the enemy turn phase of the battle interface
fn draw_battle_enemy_turn(dh: &mut RaylibDrawHandle)
{
    // TODO: draw battle enemy turn display
}

// draw_victory - draw the victory screen
fn draw_victory(dh: &mut RaylibDrawHandle)
{
    // TODO: draw victory
}

// draw_graphics - draw game graphics based on current UI phase
fn draw_graphics(game_state: &GameState, rl: &mut RaylibHandle, thread: &RaylibThread)
{
    // get raylib draw handle
    let mut dh = rl.begin_drawing(&thread);

    // TODO: replace stub function calls here as functions get implemented
    match &game_state.ui_phase
    {
        UIPhase::Overworld => draw_stub(&mut dh),
        UIPhase::BattleActionSelect => draw_stub(&mut dh),
        UIPhase::BattleTargetSelect => draw_stub(&mut dh),
        UIPhase::BattleItemSelect => draw_stub(&mut dh),
        UIPhase::BattleDamageDisplay => draw_stub(&mut dh),
        UIPhase::BattleEnemyTurn => draw_stub(&mut dh),
        UIPhase::Victory => draw_stub(&mut dh),
    }
}

///////////////////
// MAIN FUNCTION //
///////////////////

// main - application entry point and main game loop
fn main()
{

    // initialize graphics settings
    // TODO - graphics settings should ultimately be mutable
    let graphics_settings = GraphicsSettings
    {
        screen_width: 800,
        screen_height: 600,
        target_fps: 60
    };

    // initialize game state parameters
    let mut game_state = GameState
    {
        player_hp: 100,
        player_sp: 20,
        enemy_hp: 60,

        ui_phase: UIPhase::Overworld,

        // TODO - figure out player and enemy locations, and player and enemy hitbox sizes
        overworld_state: OverworldState
        {
            player_x: 200,
            player_y: 150,
            player_width: 30,
            player_height: 100,
            
            enemy_x: 575,
            enemy_y: 225,
            enemy_width: 50,
            enemy_height: 50,
        }
    };

    // initialize raylib
    let (mut rl, thread) = raylib::init()
        .size(graphics_settings.screen_width, graphics_settings.screen_height)
        .title("a_prototype")
        .build();

    // set target FPS
    rl.set_target_fps(graphics_settings.target_fps);

    // main game loop
    while !rl.window_should_close()
    {
        update_state(&mut game_state, &mut rl);
        draw_graphics(&game_state, &mut rl, &thread);
    }

}

