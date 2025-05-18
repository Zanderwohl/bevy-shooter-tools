use clap::{Arg, Command};

pub struct EditorParams {
    pub lang: String,
    pub game_id: String,
}

impl EditorParams {
    pub fn new() -> Result<Self, String> {
        let matches = Command::new("Editor")
            .arg(
                Arg::new("lang")
                    .long("lang")
                    .short('l')
                    .default_value("en-US")
                    .help("Language file to use for editor UI.")
            )
            .arg(
                Arg::new("game_id")
                    .long("game")
                    .short('g')
                    .default_value("none")
                    .help("ID of game to launch with special tools.")
            )
            .get_matches();

        let lang = matches.get_one::<String>("lang").ok_or("invalid language")?;
        let game_id = matches.get_one::<String>("game_id").ok_or("invalid game_id")?;

        Ok(Self {
            lang: lang.to_owned(),
            game_id: game_id.to_owned(),
        })
    }
}
