use serde::{Deserialize, Serialize};
mod GameData

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")] // new line
pub struct GameDataJSON{
  Enemies: EnemiesDataJSON,
  SpecialMoves: SpecialMovesDataJSON,
  Equipment: EquipmentDataJSON,
  CharacterLevels: CharacterLevelDataJSON
}


impl GameDataJSON{
    pub fn readJSON(){
        //Code to read json into this object:
    }

    pub fn convertJSONToGameData() -> GameData{
        //Code to convert from GameDataJSON to GameData per example
    }
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemiesDataJSON{
  EasyEnemies: EasyDataJSON,
  MediuEnemies: MediumDataJSON,
  HardEnemies: HardDataJSON
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EasyDataJSON{
  enemy1: EnemyDataJSON,
  enemy2: EnemyDataJSON,
  enemy3: EnemyDataJSON
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediumDataJSON{
  enemy4: EnemyDataJSON,
  enemy5: EnemyDataJSON,
  enemy6: EnemyDataJSON
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HardDataJSON{
  enemy7: EnemyDataJSON,
  enemy8: EnemyDataJSON,
  enemy9: EnemyDataJSON
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyDataJSON{
  Name: String,
  Health: i32,
  Attack: i32,
  Weapon: String,
  SpecialMove: String
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecialMoveDataJSON{
  Name: String,
  Attack: i32,
  Turns: i32,
  TurnsDamage: i32,
  EffectApplied: String
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecialMovesDataJSON{
  SpecialMove1: SpecialMoveDataJSON,
  SpecialMove2: SpecialMoveDataJSON,
  SpecialMove3: SpecialMoveDataJSON,
  SpecialMove4: SpecialMoveDataJSON,
  SpecialMove5: SpecialMoveDataJSON,
  SpecialMove6: SpecialMoveDataJSON
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CharLevelDataJSON{
  Level1: i32,
  Level2: i32,
  Level3: i32,
  Level4: i32,
  Level5: i32,
  Level6: i32,
  Level7: i32,
  Level8: i32
}