use serde::{Deserialize, Serialize};
use std::fs::File;
mod GameData

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")] // new line
pub struct GameDataJSON{
  Enemies: EnemiesDataJSON,
  SpecialMoves: SpecialMovesDataJSON,
  Equipment: EquipmentDataJSON,
  CharacterLevels: CharacterLevelDataJSON
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
  expDice: i32,
  enemy1: EnemyDataJSON,
  enemy2: EnemyDataJSON,
  enemy3: EnemyDataJSON
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediumDataJSON{
  expDice: i32,
  enemy4: EnemyDataJSON,
  enemy5: EnemyDataJSON,
  enemy6: EnemyDataJSON
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HardDataJSON{
  expDice: i32,
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
  SpecialMove: SpecialMoveData
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

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EquipmentData{
  BasicWeapons: i32,
  MediumWeapons: i32,
  GoodWeapons: i32
}

impl GameDataJSON{
  pub fn readJSON(){
      //Code to read json into this object:
      let mut game_data :GameDataJSON= serde_json::from_reader(File::open("./Resources.json"));
      fillGameData(game_data)
  }

  pub fn fillGameData(GameData: &game_data) -> GameData  {
    let mut game: GameData {
      GridData: null,
      PlayerData: null,
      Enemies: game_data.Enemies,
      Equipment: game_data.Equipment,
      CharacterLevels: game_data.CharacterLevels
    }
    game;
  }
}