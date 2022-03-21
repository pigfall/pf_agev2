pub mod app;
pub mod scene;
pub mod main_entry_macro;

pub use app::App;

pub struct AppToRun<GameEntry: GameEntryTrait>{
    custome_game_entry: Option<GameEntry>,
    app: App,
}

pub trait GameEntryTrait {
    fn android_activity_on_create();
}
