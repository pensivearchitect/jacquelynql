// encoding of all the server responses I've seen
#![allow(dead_code)]

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct StatsMessage {
    event_type: EventType,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Frag {
    killer: Player,
    match_guid: String,
    weapon_mod: String,
    other_team_alive: i32,
    other_team_dead: i32,
    round: i32,
    suicide: bool,
    teamkill: bool,
    team_alive: i32,
    team_dead: i32,
    time: i32,
    victim: Player,
    warmup: bool,
}
type PlayerKill = Frag;
type PlayerDeath = Frag;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Player {
    airborne: bool,
    ammo: i32,
    armor: i32,
    bot: bool,
    bot_skill: i32,
    health: i32,
    holdable: String,
    name: String,
    position: Position,
    powerups: Vec<String>,
    speed: f32,
    steam_id: String,
    streak: i32,
    submerged: bool,
    team: u8,
    view: View,
    weapon: Weapon,
}

// players position on the map
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Position {
    x: f32,
    y: f32,
    z: f32,
}

// where the player was facing when they died
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct View {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct MatchReport {
    aborted: bool,
    capture_limit: i32,
    exit_msg: String,
    factory: String,
    factory_title: String,
    first_scorer: String,
    frag_limit: i32,
    game_length: i32,
    game_type: String,
    infected: i32,
    instagib: u8,
    last_lead_change_time: i32,
    last_scorer: String,
    last_teamscorer: String,
    map: String,
    match_guid: String,
    mercy_limit: i32,
    quad_hog: i32,
    restarted: i32,
    round_limit: i32,
    score_limit: i32,
    server_title: String,
    time_limit: i32,
    training: i32,
    tscore0: i32,
    tscore1: i32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct MatchStarted {
    capture_limit: i32,
    factory: String,
    factory_title: String,
    frag_limit: i32,
    game_type: String,
    infected: i32,
    instagib: i32,
    map: String,
    match_guid: String,
    mercy_limit: i32,
    players: Vec<Player>,
    quad_hog: i32,
    round_limit: i32,
    score_limit: i32,
    server_title: String,
    time_limit: i32,
    training: i32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct PlayerMedal {
    match_guid: String,
    medal: Medal,
    name: String,
    steam_id: String,
    time: i32,
    total: i32,
    warmup: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct PlayerStats {
    // TODO: resolve how to dry up medals field vs Medal enum
    aborted: bool,
    blue_flag_pickups: i32,
    damage: Damage,
    deaths: i32,
    holy_shits: i32,
    item_timing: ItemTiming,
    kills: i32,
    lose: i32,
    match_guid: String,
    max_streak: i32,
    model: String,
    name: String,
    neutral_flag_pickups: i32,
    pickups: Pickups,
    play_time: i32,
    quit: i32,
    rank: i32,
    red_flag_pickups: i32,
    score: i32,
    steam_id: String,
    team: u8,
    team_join_time: i32,
    team_rank: i32,
    tied_rank: i32,
    tied_team_rank: i32,
    warmup: bool,
    win: i32, // weapons
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Pickups {
    ammo: i32,
    armor: i32,
    armor_regen: i32,
    battlesuit: i32,
    doubler: i32,
    flight: i32,
    green_armor: i32,
    guard: i32,
    haste: i32,
    health: i32,
    invis: i32,
    invulnerability: i32,
    kamikaze: i32,
    medkit: i32,
    mega_health: i32,
    other_holdable: i32,
    portal: i32,
    quad: i32,
    red_armor: i32,
    regen: i32,
    scout: i32,
    teleporter: i32,
    yellow_armor: i32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct ItemTiming {
    green_armor: i32,
    mega_health: i32,
    red_armor: i32,
    yellow_armor: i32,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct Damage {
    dealt: i32,
    taken: i32,
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
struct RoundOver {
    match_guid: String,
    round: i32,
    team_won: Team,
    time: i32,
    warmup: bool,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
enum Team {
    Red,
    Blue,
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
enum Medal {
    Impressive,
    Excellent,
}
#[derive(Debug, Deserialize, PartialEq, Serialize)]
enum Weapon {
    Lightning,
    RailGun,
    Bfg,
    ChainGun,
    Gauntlet,
    Grenade,
    Hmg,
    MachineGun,
    NailGun,
    OtherWeapon,
    Plasma,
    ProxMine,
    Rocket,
    ShotGun,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
enum EventType {
    MatchReport,
    MatchStarted,
    PlayerDeath,
    PlayerKill,
    PlayerMedal,
    PlayerStats,
    RoundOver,
}
