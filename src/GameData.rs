use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")] // new line
pub struct GameData{
  Enemies: EnemiesData,
  SpecialMoves: SpecialMovesData,
  Equipment: EquipmentData,
  CharacterLevels: CharacterLevelData
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemiesData{
  EasyEnemies: EasyData,
  MediuEnemies: MediumData,
  HardEnemies: HardData
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EasyData{
  enemy1: EnemyData,
  enemy2: EnemyData,
  enemy3: EnemyData
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MediumData{
  enemy4: EnemyData,
  enemy5: EnemyData,
  enemy6: EnemyData
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct HardData{
  enemy7: EnemyData,
  enemy8: EnemyData,
  enemy9: EnemyData
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EnemyData{
  Name: String,
  Health: i32,
  Attack: i32,
  Weapon: String,
  SpecialMove: String
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecialMoveData{
  Name: String,
  Attack: i32,
  Turns: i32,
  TurnsDamage: i32,
  EffectApplied: String
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpecialMovesData{
  SpecialMove1: SpecialMoveData,
  SpecialMove2: SpecialMoveData,
  SpecialMove3: SpecialMoveData,
  SpecialMove4: SpecialMoveData,
  SpecialMove5: SpecialMoveData,
  SpecialMove6: SpecialMoveData
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CharLevelData{
  Level1: i32,
  Level2: i32,
  Level3: i32,
  Level4: i32,
  Level5: i32,
  Level6: i32,
  Level7: i32,
  Level8: i32
}