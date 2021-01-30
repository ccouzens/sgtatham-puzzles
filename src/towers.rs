pub struct Towers;
use super::puzzles::{ConfigItem, ConfigItemType, Game, GameParams};

#[derive(Clone)]
pub struct TowerParams {
    w: u8,
    diff: Difficulty,
}

impl Default for TowerParams {
    fn default() -> Self {
        Self {
            w: 5,
            diff: Difficulty::Easy,
        }
    }
}

impl GameParams for TowerParams {
    const CAN_CONFIGURE: bool = true;

    fn decode(encoded: &str) -> Self {
        let mut parts = encoded.split('d');
        let w = parts
            .next()
            .and_then(|p| p.parse().ok())
            .unwrap_or_else(|| Self::default().w);
        let diff = match parts.next() {
            Some("e") => Difficulty::Easy,
            Some("h") => Difficulty::Hard,
            Some("x") => Difficulty::Extreme,
            Some("u") => Difficulty::Unreasonable,
            _ => Self::default().diff,
        };
        Self { w, diff }
    }

    fn encode(&self) -> String {
        format!("{}", self.w)
    }

    fn encode_full(&self) -> String {
        format!(
            "{}d{}",
            self.w,
            match self.diff {
                Difficulty::Easy => 'e',
                Difficulty::Hard => 'h',
                Difficulty::Extreme => 'x',
                Difficulty::Unreasonable => 'u',
            }
        )
    }

    fn configure(&self) -> Vec<ConfigItem> {
        vec![
            ConfigItem {
                name: "Grid size".into(),
                value: ConfigItemType::String(format!("{}", self.w)),
            },
            ConfigItem {
                name: "Difficulty".into(),
                value: ConfigItemType::Choices {
                    names: vec![
                        "Easy".into(),
                        "Hard".into(),
                        "Extreme".into(),
                        "Unreasonable".into(),
                    ],
                    selected: match self.diff {
                        Difficulty::Easy => 0,
                        Difficulty::Hard => 1,
                        Difficulty::Extreme => 2,
                        Difficulty::Unreasonable => 3,
                    },
                },
            },
        ]
    }

    fn custom_params(cfg: &[ConfigItem]) -> Option<Self> {
        let w = cfg.get(0)?.value.as_str()?.parse().ok()?;
        let diff = match cfg.get(1)?.value.as_choice()? {
            0 => Some(Difficulty::Easy),
            1 => Some(Difficulty::Hard),
            2 => Some(Difficulty::Extreme),
            3 => Some(Difficulty::Unreasonable),
            _ => None
        }?;
        Some(Self { w, diff })
    }

    fn validate(&self) -> Result<(), &'static str> {
        if self.w < 3 || self.w > 9 {
            Err("Grid size must be between 3 and 9")
        } else {
            Ok(())
        }
    }
}

#[derive(Clone)]
enum Difficulty {
    Easy,
    Hard,
    Extreme,
    Unreasonable,
}

struct GameState {
    par: TowerParams,
    // clues
    // clues done
    // grid
}

impl<'a> Game<'a> for Towers {
    type Params = TowerParams;

    const NAME: &'static str = "Towers";
    const WINHELP_TOPIC: &'static str = "games.towers";
    const HTMLHELP_TOPIC: &'static str = "towers";
    const PRESETS: &'static [(&'static str, TowerParams)] = &[
        (
            "4x4 Easy",
            TowerParams {
                w: 4,
                diff: Difficulty::Easy,
            },
        ),
        (
            "5x5 Easy",
            TowerParams {
                w: 5,
                diff: Difficulty::Easy,
            },
        ),
        (
            "5x5 Hard",
            TowerParams {
                w: 5,
                diff: Difficulty::Hard,
            },
        ),
        (
            "6x6 Easy",
            TowerParams {
                w: 6,
                diff: Difficulty::Easy,
            },
        ),
        (
            "6x6 Hard",
            TowerParams {
                w: 6,
                diff: Difficulty::Hard,
            },
        ),
        (
            "6x6 Extreme",
            TowerParams {
                w: 6,
                diff: Difficulty::Extreme,
            },
        ),
        (
            "6x6 Unreasonable",
            TowerParams {
                w: 6,
                diff: Difficulty::Unreasonable,
            },
        ),
    ];
}
