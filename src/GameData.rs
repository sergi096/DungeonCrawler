use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")] // new line
pub struct GameData{
  GridData: Vec<Vec<Zone>>,
  PlayerData: PlayerData
  Enemies: EnemiesData,
  SpecialMoves: SpecialMovesData,
  Equipment: EquipmentData,
  CharacterLevels: CharacterLevelData   
}

#[derive(Serialize, Deserialize)]
pub struct PlayerData{
    name: String,
    attack: i32,
    hitpoints: i32,
    weapon: WeaponData,
    bag: Vec<WeaponData>
}

pub struct WeaponData
{
    name: String,
    attack: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")] // new line
pub struct Zone{
    coordinatesX: Vec<i32>,
    coordinatesY: Vec<i32>,
    message: String
}

impl GameData{
    pub fn initialize(){
        //initialize grid
        let mut counterx : i32 = 0;
        let mut countery : i32 = 0;
        for x in 0..3{
            for y in 0..3{
                let zone: Zone = {x: counterx, y; countery, message: "Test"};
                GridData[counterx[countery]] = zone;
            }
        }

        //Ahora tenemos una grid 3x3 pero que está vacía,
        //Initialize Enemies in grid

        for x in 0..3{
            for y in 0..3{
                
            }
        }
    }

    pub fn roll_dice(num_dice: i32) -> i32 {
        let mut tiradas: Vec<i32> = Vec::new();
        let mut rng = rand::thread_rng();
        let mut total = 0;
        print!("Rolling {}d6 ", num_dice);
        print!("Rolled ");
        for _ in 0..num_dice {
            let roll = rng.gen_range(1, 7);
            tiradas.push(roll);
            print!("[{}]", roll);
            total += roll;
        }
        print!("Total: {}", total);
        total
    }
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