#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use discord_rich_presence::{activity::{Activity, Timestamps}, DiscordIpc, DiscordIpcClient};
use std::sync::Mutex;

#[derive(Default)]
struct Rpc(Mutex<Option<(String, DiscordIpcClient)>>);

// Called from the app with the track and elapsed time; Discord must be running.
#[tauri::command]
fn presence(state: tauri::State<Rpc>, app_id: String, title: String, artist: String, start: i64, playing: bool) -> Result<(), String> {
    let mut g = state.0.lock().map_err(|e| e.to_string())?;
    if g.as_ref().map(|(id, _)| id != &app_id).unwrap_or(true) {
        let mut c = DiscordIpcClient::new(&app_id).map_err(|e| e.to_string())?;
        c.connect().map_err(|e| e.to_string())?;
        *g = Some((app_id, c));
    }
    let c = &mut g.as_mut().unwrap().1;
    if playing {
        c.set_activity(Activity::new().details(&title).state(&artist).timestamps(Timestamps::new().start(start))).map_err(|e| e.to_string())
    } else {
        c.clear_activity().map_err(|e| e.to_string())
    }
}

fn main() {
    tauri::Builder::default()
        .manage(Rpc::default())
        .invoke_handler(tauri::generate_handler![presence])
        .run(tauri::generate_context!())
        .expect("error while running Delta");
}
