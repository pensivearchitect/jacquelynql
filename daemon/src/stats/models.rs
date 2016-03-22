// encoding of all the server responses I've seen
// repeat the fields for playerkill and death so that we can impl Event for each
#[derive(Debug, Deserialize, Serialize)]
pub struct Killer {
    name: Option<String>,
    old_team: Option<String>,
    steam_id: Option<String>,
    team: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerSwitchTeam {
    killer: Option<Killer>,
    match_guid: Option<String>,
    time: Option<i64>,
    warmup: Option<bool>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerConnect {
    #[serde(rename="MATCH_GUID")]
    match_guid: Option<String>,
    #[serde(rename="NAME")]
    name: Option<String>,
    #[serde(rename="STEAM_ID")]
    steam_id: Option<String>,
    #[serde(rename="TIME")]
    time: Option<i64>,
    #[serde(rename="WARMUP")]
    warmup: Option<bool>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerDisconnect {
    #[serde(rename="MATCH_GUID")]
    match_guid: Option<String>,
    #[serde(rename="NAME")]
    name: Option<String>,
    #[serde(rename="STEAM_ID")]
    steam_id: Option<String>,
    #[serde(rename="TIME")]
    time: Option<i64>,
    #[serde(rename="WARMUP")]
    warmup: Option<bool>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerDeath {
    #[serde(rename="KILLER")]
    killer: Option<Player>,
    #[serde(rename="MATCH_GUID")]
    match_guid: Option<String>,
    #[serde(rename="WEAPON_MOD")]
    weapon_mod: Option<String>,
    #[serde(rename="OTHER_TEAM_ALIVE")]
    other_team_alive: Option<i64>,
    #[serde(rename="OTHER_TEAM_DEAD")]
    other_team_dead: Option<i64>,
    #[serde(rename="ROUND")]
    round: Option<i64>,
    #[serde(rename="SUICIDE")]
    suicide: Option<bool>,
    #[serde(rename="TEAMKILL")]
    teamkill: Option<bool>,
    #[serde(rename="TEAM_ALIVE")]
    team_alive: Option<i64>,
    #[serde(rename="TEAM_DEAD")]
    team_dead: Option<i64>,
    #[serde(rename="TIME")]
    time: Option<i64>,
    #[serde(rename="VICTIM")]
    victim: Option<Player>,
    #[serde(rename="WARMUP")]
    warmup: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerKill {
    #[serde(rename="KILLER")]
    killer: Option<Player>,
    #[serde(rename="MATCH_GUID")]
    match_guid: Option<String>,
    #[serde(rename="WEAPON_MOD")]
    weapon_mod: Option<String>,
    #[serde(rename="OTHER_TEAM_ALIVE")]
    other_team_alive: Option<i64>,
    #[serde(rename="OTHER_TEAM_DEAD")]
    other_team_dead: Option<i64>,
    #[serde(rename="ROUND")]
    round: Option<i64>,
    #[serde(rename="SUICIDE")]
    suicide: Option<bool>,
    #[serde(rename="TEAMKILL")]
    teamkill: Option<bool>,
    #[serde(rename="TEAM_ALIVE")]
    team_alive: Option<i64>,
    #[serde(rename="TEAM_DEAD")]
    team_dead: Option<i64>,
    #[serde(rename="TIME")]
    time: Option<i64>,
    #[serde(rename="VICTIM")]
    victim: Option<Player>,
    #[serde(rename="WARMUP")]
    warmup: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MatchReport {
    #[serde(rename="ABORTED")]
    aborted: Option<bool>,
    #[serde(rename="CAPTURE_LIMIT")]
    capture_limit: Option<i64>,
    #[serde(rename="EXIT_MSG")]
    exit_msg: Option<String>,
    #[serde(rename="FACTORY")]
    factory: Option<String>,
    #[serde(rename="FACTORY_TITLE")]
    factory_title: Option<String>,
    #[serde(rename="FIRST_SCORER")]
    first_scorer: Option<String>,
    #[serde(rename="FRAG_LIMIT")]
    frag_limit: Option<i64>,
    #[serde(rename="GAME_LENGTH")]
    game_length: Option<i64>,
    #[serde(rename="GAME_TYPE")]
    game_type: Option<String>,
    #[serde(rename="INFECTED")]
    infected: Option<i64>,
    #[serde(rename="INSTAGIB")]
    instagib: Option<u8>,
    #[serde(rename="LAST_LEAD_CHANGE_TIME")]
    last_lead_change_time: Option<i64>,
    #[serde(rename="LAST_SCORER")]
    last_scorer: Option<String>,
    #[serde(rename="LAST_TEAMSCORER")]
    last_teamscorer: Option<String>,
    #[serde(rename="MAP")]
    map: Option<String>,
    #[serde(rename="MATCH_GUID")]
    match_guid: Option<String>,
    #[serde(rename="MERCY_LIMIT")]
    mercy_limit: Option<i64>,
    #[serde(rename="QUAD_HOG")]
    quad_hog: Option<i64>,
    #[serde(rename="RESTARTED")]
    restarted: Option<i64>,
    #[serde(rename="ROUND_LIMIT")]
    round_limit: Option<i64>,
    #[serde(rename="SCORE_LIMIT")]
    score_limit: Option<i64>,
    #[serde(rename="SERVER_TITLE")]
    server_title: Option<String>,
    #[serde(rename="TIME_LIMIT")]
    time_limit: Option<i64>,
    #[serde(rename="training")]
    training: Option<i64>,
    #[serde(rename="TSCORE0")]
    tscore0: Option<i64>,
    #[serde(rename="TSCORE1")]
    tscore1: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerMedal {
    #[serde(rename="MATCH_GUID")]
    match_guid: Option<String>,
    #[serde(rename="MEDAL")]
    medal: Option<String>,
    #[serde(rename="NAME")]
    name: Option<String>,
    #[serde(rename="STEAM_ID")]
    steam_id: Option<String>,
    #[serde(rename="TIME")]
    time: Option<i64>,
    #[serde(rename="TOTAL")]
    total: Option<i64>,
    #[serde(rename="WARMUP")]
    warmup: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerStats {
    #[serde(rename="ABORTED")]
    aborted: Option<bool>,
    #[serde(rename="BLUE_FLAG_PICKUPS")]
    blue_flag_pickups: Option<i64>,
    #[serde(rename="DAMAGE")]
    damage: Option<Damage>,
    #[serde(rename="DEATHS")]
    deaths: Option<i64>,
    #[serde(rename="HOLY_SHITS")]
    holy_shits: Option<i64>,
    #[serde(rename="ITEM_TIMING")]
    item_timing: Option<ItemTiming>,
    #[serde(rename="KILLS")]
    kills: Option<i64>,
    #[serde(rename="LOSE")]
    lose: Option<i64>,
    #[serde(rename="MATCH_GUID")]
    match_guid: Option<String>,
    #[serde(rename="MAX_STREAK")]
    max_streak: Option<i64>,
    #[serde(rename="MODEL")]
    model: Option<String>,
    #[serde(rename="NAME")]
    name: Option<String>,
    #[serde(rename="NEUTRAL_FLAG_PICKUPS")]
    neutral_flag_pickups: Option<i64>,
    #[serde(rename="PICKUPS")]
    pickups: Option<Pickups>,
    #[serde(rename="PLAY_TIME")]
    play_time: Option<i64>,
    #[serde(rename="QUIT")]
    quit: Option<i64>,
    #[serde(rename="RANK")]
    rank: Option<i64>,
    #[serde(rename="RED_FLAG_PICKUPS")]
    red_flag_pickups: Option<i64>,
    #[serde(rename="SCORE")]
    score: Option<i64>,
    #[serde(rename="STEAM_ID")]
    steam_id: Option<String>,
    #[serde(rename="TEAM")]
    team: Option<u8>,
    #[serde(rename="TEAM_JOIN_TIME")]
    team_join_time: Option<i64>,
    #[serde(rename="TEAM_RANK")]
    team_rank: Option<i64>,
    #[serde(rename="TIED_RANK")]
    tied_rank: Option<i64>,
    #[serde(rename="TIED_TEAM_RANK")]
    tied_team_rank: Option<i64>,
    #[serde(rename="WARMUP")]
    warmup: Option<bool>,
    #[serde(rename="WIN")]
    win: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pickups {
    #[serde(rename="AMMO")]
    ammo: Option<i64>,
    #[serde(rename="ARMOR")]
    armor: Option<i64>,
    #[serde(rename="ARMOR_REGEN")]
    armor_regen: Option<i64>,
    #[serde(rename="BATTLESUIT")]
    battlesuit: Option<i64>,
    #[serde(rename="DOUBLER")]
    doubler: Option<i64>,
    #[serde(rename="FLIGHT")]
    flight: Option<i64>,
    #[serde(rename="GREEN_ARMOR")]
    green_armor: Option<i64>,
    #[serde(rename="GUARD")]
    guard: Option<i64>,
    #[serde(rename="HASTE")]
    haste: Option<i64>,
    #[serde(rename="HEALTH")]
    health: Option<i64>,
    #[serde(rename="INVIS")]
    invis: Option<i64>,
    #[serde(rename="INVULNERABILITY")]
    invulnerability: Option<i64>,
    #[serde(rename="KAMIKAZE")]
    kamikaze: Option<i64>,
    #[serde(rename="MEDKIT")]
    medkit: Option<i64>,
    #[serde(rename="MEGA_HEALTH")]
    mega_health: Option<i64>,
    #[serde(rename="OTHER_HOLDABLE")]
    other_holdable: Option<i64>,
    #[serde(rename="PORTAL")]
    portal: Option<i64>,
    #[serde(rename="QUAD")]
    quad: Option<i64>,
    #[serde(rename="RED_ARMOR")]
    red_armor: Option<i64>,
    #[serde(rename="REGEN")]
    regen: Option<i64>,
    #[serde(rename="SCOUT")]
    scout: Option<i64>,
    #[serde(rename="TELEPORTER")]
    teleporter: Option<i64>,
    #[serde(rename="YELLOW_ARMOR")]
    yellow_armor: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemTiming {
    #[serde(rename="GREEN_ARMOR")]
    green_armor: Option<i64>,
    #[serde(rename="MEGA_HEALTH")]
    mega_health: Option<i64>,
    #[serde(rename="RED_ARMOR")]
    red_armor: Option<i64>,
    #[serde(rename="YELLOW_ARMOR")]
    yellow_armor: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Damage {
    #[serde(rename="DEALT")]
    dealt: Option<i64>,
    #[serde(rename="TAKEN")]
    taken: Option<i64>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct RoundOver {
    #[serde(rename="MATCH_GUID")]
    match_guid: Option<String>,
    #[serde(rename="ROUND")]
    round: Option<i64>,
    #[serde(rename="TEAM_WON")]
    team_won: String,
    #[serde(rename="TIME")]
    time: Option<i64>,
    #[serde(rename="WARMUP")]
    warmup: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct MatchStarted {
    #[serde(rename="CAPTURE_LIMIT")]
    capture_limit: i64,
    #[serde(rename="FACTORY")]
    factory: String,
    #[serde(rename="FACTORY_TITLE")]
    factory_title: String,
    #[serde(rename="FRAG_LIMIT")]
    frag_limit: i64,
    #[serde(rename="GAME_TYPE")]
    game_type: String,
    #[serde(rename="INFECTED")]
    infected: i64,
    #[serde(rename="INSTAGIB")]
    instagib: i64,
    #[serde(rename="MAP")]
    map: String,
    #[serde(rename="MATCH_GUID")]
    match_guid: String,
    #[serde(rename="MERCY_LIMIT")]
    mercy_limit: i64,
    #[serde(rename="PLAYERS")]
    players: Vec<Player>,
    #[serde(rename="QUAD_HOG")]
    quad_hog: Option<i64>,
    #[serde(rename="ROUND_LIMIT")]
    round_limit: i64,
    #[serde(rename="SCORE_LIMIT")]
    score_limit: i64,
    #[serde(rename="SERVER_TITLE")]
    server_title: String,
    #[serde(rename="TIME_LIMIT")]
    time_limit: i64,
    #[serde(rename="TRAINING")]
    training: i64,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Player {
    #[serde(rename="AIRBORNE")]
    airborne: Option<bool>,
    #[serde(rename="AMMO")]
    ammo: Option<i64>,
    #[serde(rename="ARMOR")]
    armor: Option<i64>,
    #[serde(rename="BOT")]
    bot: Option<bool>,
    #[serde(rename="BOT_SKILL")]
    bot_skill: Option<i64>,
    #[serde(rename="HEALTH")]
    health: Option<i64>,
    #[serde(rename="HOLDABLE")]
    holdable: Option<String>,
    #[serde(rename="NAME")]
    name: Option<String>,
    #[serde(rename="POSITION")]
    position: Option<Position>,
    #[serde(rename="POWERUPS")]
    powerups: Option<Vec<String>>,
    #[serde(rename="SPEED")]
    speed: Option<f32>,
    #[serde(rename="STEAM_ID")]
    steam_id: Option<String>,
    #[serde(rename="STREAK")]
    streak: Option<i64>,
    #[serde(rename="SUBMERGED")]
    submerged: Option<bool>,
    #[serde(rename="TEAM")]
    team: Option<u8>,
    #[serde(rename="VIEW")]
    view: Option<View>,
    #[serde(rename="WEAPON")]
    weapon: Option<String>,
}

// players position on the map
#[derive(Debug, Deserialize, Serialize)]
pub struct Position {
    #[serde(rename="X")]
    x: Option<f32>,
    #[serde(rename="Y")]
    y: Option<f32>,
    #[serde(rename="Z")]
    z: Option<f32>,
}

// where the player was facing when they died
#[derive(Debug, Deserialize, Serialize)]
pub struct View {
    #[serde(rename="X")]
    x: Option<f32>,
    #[serde(rename="Y")]
    y: Option<f32>,
    #[serde(rename="Z")]
    z: Option<f32>,
}
