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

// GameState - data structure used to keep track of higher-level game and overworld state
struct GameState
{
    player_x: i32,
    player_y: i32,
    player_width: i32,
    player_height: i32,
    player_head_radius: i32,

    enemy_x: i32,
    enemy_y: i32,
    enemy_radius: i32,

    // this really should be a setting/stat, not a state parameter. at least for now...
    player_overworld_speed: i32,

    menu_selection_cursor: u32,

    ui_phase: UIPhase,
}

// BattleState - data structure used to keep track of battle-related state info
struct BattleState
{
    player_hp: i32,
    player_sp: i32,
    player_power: i32,

    enemy_hp: i32,
    enemy_power: i32,
}


//////////////////////////////
// STATE UPDATING FUNCTIONS //
//////////////////////////////

// TODO - key bindings should ultimately be mutable - make issue for this eventually; i'm not going
// to address it in the 3/12 sprint

// has_hitbox_collision - function that determines whether the player and enemy hitboxes collide
fn has_hitbox_collision(game_state: &GameState) -> bool
{
    // compute the distance from the center of the player's head to the center of the enemy
    // this is just the pythagorean distance formula
    let head_enemy_centers_distance = f64::sqrt(
                                        f64::powi(((game_state.player_x + game_state.player_head_radius) -
                                                (game_state.enemy_x + game_state.enemy_radius)).into(), 2) +
                                        f64::powi(((game_state.player_y + game_state.player_head_radius) -
                                                (game_state.enemy_y + game_state.enemy_radius)).into(), 2));

    // check if player's head collides with enemy
    if head_enemy_centers_distance <= (game_state.player_head_radius + game_state.enemy_radius).into()
    {
        // if the distance between the circle centers is less than or equal to the sum of the
        // circle radii, then the circles intersect, and we have a collision
        return true;
    }

    // check if the player's body collides with the enemy
    // TODO: determine algorithm for square-circle hitbox collision

    return false;
}

// shift_battle_phase - shift to a new phase of the battle UI
fn shift_battle_phase(game_state: &mut GameState, new_phase: UIPhase)
{
    // transition to new UI phase
    game_state.ui_phase = new_phase;

    // set default menu selection
    game_state.menu_selection_cursor = 0;
}

// update_state_overworld - update state while player is on the overworld
fn update_state_overworld(game_state: &mut GameState, rl: &mut RaylibHandle)
{
    // check for player hitbox collision with enemy
    if has_hitbox_collision(game_state)
    {
        // go directly into battle if the player and enemy collide
        shift_battle_phase(game_state, UIPhase::BattleActionSelect);
    }

    // check if a movement key is down
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP)
    {
        // move player up
        game_state.player_y -= game_state.player_overworld_speed;
    }
    else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_A)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_LEFT)
    {
        // move player left
        game_state.player_x -= game_state.player_overworld_speed;
    }
    else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN)
    {
        // move player down
        game_state.player_y += game_state.player_overworld_speed;
    }
    else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_D)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_RIGHT)
    {
        // move player right
        game_state.player_x += game_state.player_overworld_speed;
    }
}

// update_state_battle_action_select - update state while player is selecting a battle action
fn update_state_battle_action_select(game_state: &mut GameState, rl: &mut RaylibHandle)
{
    // TODO: this value shouldn't be hard-coded (another reason for a lookup table, or perhaps a valued enum, of menu options...)
    let action_menu_items = 3u32;

    // check if an action key is down
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP)
    {
        // move menu selection cursor up, if there's room to move up
        if game_state.menu_selection_cursor > 0
        {
            game_state.menu_selection_cursor -= 1;
        }
    }
    else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN)
    {
        // move menu selection cursor down, if there's room to move down
        if game_state.menu_selection_cursor < action_menu_items - 1
        {
            game_state.menu_selection_cursor += 1;
        }
    }
    else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_ENTER)
    {
        // TODO: get rid of magic numbers; implement lookup table, perhaps?
        if game_state.menu_selection_cursor == 0
        {
            // fight selected
            game_state.ui_phase = UIPhase::BattleTargetSelect;
        }
        else if game_state.menu_selection_cursor == 1
        {
            // skills selected
            shift_battle_phase(game_state, UIPhase::BattleItemSelect);
        }
        else if game_state.menu_selection_cursor == 2
        {
            // items selected
            shift_battle_phase(game_state, UIPhase::BattleItemSelect);
        }
    }
}

// update_state_battle_target_select - update state while player is selecting a target in battle
fn update_state_battle_target_select(game_state: &mut GameState, battle_state: &mut BattleState, rl: &mut RaylibHandle)
{
    // check if an action key is down
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_ENTER)
    {
        // hit enemy with damage
        battle_state.enemy_hp -= battle_state.player_power;
        if battle_state.enemy_hp < 0
        {
            battle_state.enemy_hp = 0;
        }

        // shift to damage display
        shift_battle_phase(game_state, UIPhase::BattleDamageDisplay);
    }
}

// update_state_battle_item_select - update state while player is selecting an item in battle
fn update_state_battle_item_select(game_state: &mut GameState, rl: &mut RaylibHandle)
{
    // TODO: this value shouldn't be hard-coded (another reason for a lookup table, or perhaps a valued enum, of menu options...)
    let menu_items = 3u32;

    // check if an action key is down
    if rl.is_key_down(raylib::consts::KeyboardKey::KEY_W)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_UP)
    {
        // move menu selection cursor up, if there's room to move up
        if game_state.menu_selection_cursor > 0
        {
            game_state.menu_selection_cursor -= 1;
        }
    }
    else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_S)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_DOWN)
    {
        // move menu selection cursor down, if there's room to move down
        if game_state.menu_selection_cursor < menu_items - 1
        {
            game_state.menu_selection_cursor += 1;
        }
    }
    else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_SPACE)
        || rl.is_key_down(raylib::consts::KeyboardKey::KEY_ENTER)
    {
        // TODO: get rid of magic numbers; implement lookup table, perhaps?
        if game_state.menu_selection_cursor == 0
        {
            // nothing selected
        }
        else if game_state.menu_selection_cursor == 1
        {
            // nothing selected
        }
        else if game_state.menu_selection_cursor == 2
        {
            // back selected
            shift_battle_phase(game_state, UIPhase::BattleActionSelect);
        }
    }
    else if rl.is_key_down(raylib::consts::KeyboardKey::KEY_BACKSPACE)
    {
        // go back
        shift_battle_phase(game_state, UIPhase::BattleActionSelect);
    }
}

// update_state_battle_damage_display - update state while player is viewing damage in battle
fn update_state_battle_damage_display(game_state: &mut GameState, battle_state: &mut BattleState, rl: &mut RaylibHandle)
{
    // TODO: 3 second timer

    if battle_state.enemy_hp > 0
    {
        // move to the enemy's turn
        shift_battle_phase(game_state, UIPhase::BattleEnemyTurn);
    }
    else
    {
        // go to victory
        shift_battle_phase(game_state, UIPhase::Victory);
    }
}

// update_state_battle_enemy_turn - update state while player is viewing enemy turn in battle
fn update_state_battle_enemy_turn(game_state: &mut GameState, battle_state: &mut BattleState, rl: &mut RaylibHandle)
{
    // enemy hits the player
    battle_state.player_hp -= battle_state.enemy_power;

    // TODO: 3 second timer

    // move to the next turn
    shift_battle_phase(game_state, UIPhase::BattleActionSelect);
}

// update_state - check for keyboard input and timeouts (?), and update game state according to
// which UI phase the player is currently in
// TODO: will probably need to pass a mutable reference to battle state into these functions as well
fn update_state(game_state: &mut GameState, battle_state: &mut BattleState, rl: &mut RaylibHandle)
{
    match &game_state.ui_phase
    {
        UIPhase::Overworld => update_state_overworld(game_state, rl),
        UIPhase::BattleActionSelect => update_state_battle_action_select(game_state, rl),
        UIPhase::BattleTargetSelect => update_state_battle_target_select(game_state, battle_state, rl),
        UIPhase::BattleItemSelect => update_state_battle_item_select(game_state, rl),
        UIPhase::BattleDamageDisplay => update_state_battle_damage_display(game_state, battle_state, rl),
        UIPhase::BattleEnemyTurn => update_state_battle_enemy_turn(game_state, battle_state, rl),
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
    dh.clear_background(Color::BLACK);
}

// draw_player - draw player on the overworld
fn draw_player(game_state: &GameState, dh: &mut RaylibDrawHandle)
{
    // draw head
    dh.draw_circle(game_state.player_x + game_state.player_head_radius,
                   game_state.player_y + game_state.player_head_radius,
                   game_state.player_head_radius as f32,
                   Color::BLACK);

    // TODO: draw body
}

// draw_enemy - draw enemy on the overworld
// TODO: this code is way too procedural. replace with vector graphics representation?
fn draw_enemy(game_state: &GameState, dh: &mut RaylibDrawHandle)
{
    // TODO: make this a sprite

    // draw enemy outline
    dh.draw_circle_lines(game_state.enemy_x + game_state.enemy_radius,
                         game_state.enemy_y + game_state.enemy_radius,
                         game_state.enemy_radius as f32,
                         Color::RED);

    // TODO: draw eyebrows

    // draw eyes
    dh.draw_line(game_state.enemy_x + 2 * game_state.enemy_radius / 3,
                 game_state.enemy_y + game_state.enemy_radius / 2,
                 game_state.enemy_x + 2 * game_state.enemy_radius / 3,
                 game_state.enemy_y + 3 * game_state.enemy_radius / 4,
                 Color::RED);
    dh.draw_line(game_state.enemy_x + 4 * game_state.enemy_radius / 3,
                 game_state.enemy_y + game_state.enemy_radius / 2,
                 game_state.enemy_x + 4 * game_state.enemy_radius / 3,
                 game_state.enemy_y + 3 * game_state.enemy_radius / 4,
                 Color::RED);

    // TODO: draw mouth
}

// draw_overworld - draw the overworld scene
fn draw_overworld(game_state: &GameState, dh: &mut RaylibDrawHandle)
{
    // draw grassy overworld background
    // TODO: more detailed overworld background
    dh.clear_background(Color::GREEN);

    // draw player avatar
    draw_player(game_state, dh);
    
    // draw enemy
    draw_enemy(game_state, dh);
}

// draw_battle_action_select - draw the action select phase of the battle interface
fn draw_battle_action_select(dh: &mut RaylibDrawHandle)
{
    // TODO: draw battle action select
    dh.clear_background(Color::GRAY);
}

// draw_battle_target_select - draw the target select phase of the battle interface
fn draw_battle_target_select(dh: &mut RaylibDrawHandle)
{
    // TODO: draw battle target select
    dh.clear_background(Color::RED);
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
    // draw green background
    dh.clear_background(Color::GREEN);

    // draw victory text
    // TODO: draw victory text
}

// draw_graphics - draw game graphics based on current UI phase
fn draw_graphics(game_state: &GameState, rl: &mut RaylibHandle, thread: &RaylibThread)
{
    // get raylib draw handle
    let mut dh = rl.begin_drawing(&thread);

    // TODO: replace stub function calls here as functions get implemented
    match &game_state.ui_phase
    {
        UIPhase::Overworld => draw_overworld(game_state, &mut dh),
        UIPhase::BattleActionSelect => draw_battle_action_select(&mut dh),
        UIPhase::BattleTargetSelect => draw_battle_target_select(&mut dh),
        UIPhase::BattleItemSelect => draw_stub(&mut dh),
        UIPhase::BattleDamageDisplay => draw_stub(&mut dh),
        UIPhase::BattleEnemyTurn => draw_stub(&mut dh),
        UIPhase::Victory => draw_victory(&mut dh),
    }
}


///////////////////
// MAIN FUNCTION //
///////////////////

// main - application entry point and main game loop
fn main()
{

    // initialize graphics settings
    // TODO - graphics settings should ultimately be mutable - moving to external json in UG-3;
    // player-alterable settings in graphics and key bindings are for a future sprint
    let graphics_settings = GraphicsSettings
    {
        screen_width: 800,
        screen_height: 600,
        target_fps: 60,
    };

    // initialize game state parameters
    let mut battle_state = BattleState
    {
        player_hp: 100,
        player_sp: 20,
        player_power: 30,

        enemy_hp: 60,
        enemy_power: 20,
    };

    let mut game_state = GameState
    {
        player_x: 200,
        player_y: 150,
        player_width: 30,
        player_height: 100,
        player_head_radius: 15,

        enemy_x: 575,
        enemy_y: 225,
        enemy_radius: 50,

        player_overworld_speed: 3,

        menu_selection_cursor: 0,

        ui_phase: UIPhase::Overworld,
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
        update_state(&mut game_state, &mut battle_state, &mut rl);
        draw_graphics(&game_state, &mut rl, &thread);
    }

}

