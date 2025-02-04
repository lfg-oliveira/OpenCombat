use std::{fs, path::PathBuf, time::SystemTime};

use anyhow::{Context, Result};
use battle_core::{deployment::Deployment, sync::BattleStateCopy};
use oc_core::resources::{EnsureDir, Resources};

use crate::saves::writer::BattleStateWriter;

use super::Engine;

impl Engine {
    // TODO: Maybe we should ask server to send us the save
    // to avoid error due to incomplete gui battle state
    pub fn save_battle_state(&self) -> Result<PathBuf> {
        let now_ns = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let save_to = Resources::new()?
            .battle_saves_abs(self.battle_state.map().name())
            .join(format!("{}.ocbs", now_ns));
        save_to
            .parent()
            .expect("Save file must have parent folder")
            .to_path_buf()
            .ensure()?;

        BattleStateWriter::new(save_to.clone()).write(&self.battle_state)?;
        Ok(save_to)
    }

    pub fn load_from_save(&self, save: &PathBuf) -> Option<BattleStateCopy> {
        if let Ok(bytes) = fs::read(save) {
            if let Ok(copy) = bincode::deserialize::<BattleStateCopy>(&bytes) {
                return Some(copy);
            }
        }

        None
    }

    pub fn save_deployment(&self) -> Result<PathBuf> {
        let now_ns = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        let save_to = PathBuf::from(format!(
            "{}_{}_deployment.json",
            self.battle_state.map().name(),
            now_ns
        ));

        let deployment = Deployment::from_battle_state(&self.battle_state);
        fs::write(
            &save_to,
            serde_json::to_string(&deployment).context("Serialize deployment")?,
        )
        .context("Write deployment file")?;

        Ok(save_to)
    }
}
