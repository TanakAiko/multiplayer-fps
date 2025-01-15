
pub struct ClientState {
    pub local_player: Option<Player>,
    pub predicted_players: Vec<Player>,
    pub interpolation_buffer: Vec<GameState>,
    pub input_sequence: u32,
    pub last_server_tick: u32,
    pub ping: f32,
}