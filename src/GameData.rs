use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")] // new line
pub struct GameData{
  GridData: Vec<Vec<Zone>>,
  PlayerData: PlayerData
  Enemies: EnemiesData,
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
    rating: WeaponRating
    name: String,
    attack: i32,
}

pub enum WeaponRating{
  Basic,
  Medium,
  Advanced,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")] // new line
pub struct Zone{
    difficulty: String,
    coordinatesX: Vec<i32>,
    coordinatesY: Vec<i32>,
    message: String
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

impl GameData{
  pub fn initializeGrid(){
      //initialize grid
      for x in 0..3{
          for y in 0..3{
              //TODO: Generación aleatoria de zonas
              let zone: Zone = {difficulty: "Easy", coordinatesX: x, coordinatesX; y, message: "Test"};
              GridData[x[y]] = zone;
          }
      }

      //Ahora tenemos una grid 3x3 pero que está vacía,
      //Inicializamos enemigos dentro de la grid basándose en su dificultad
      for x in 0..3{
          for y in 0..3{
            if GridData[x[y]].zone.difficulty.eq("Easy"){
              //TODO: Generación aleatoria de enemigos
            } else if GridData[x[y]].zone.difficulty.eq("Medium"){
              //TODO: Generación aleatoria de enemigos
            } else if GridData[x[y]].zone.difficulty.eq("Hard"){
              //TODO: Generación aleatoria de enemigos
            }
          }
      }

      //Ahora tenemos nuestra grid con todos sus enemigos y recompensas
      //Inicializamos datos sobre el jugador
  }

  pub fn initializePlayer(String &name) -> PlayerData{
    let attack = roll_dice(2);
    let hitpoints = roll_dice(2);

    let weapon: WeaponData= generate_weapon_data(WeaponRating.Basic)
    let player :PlayerData  = {name: &name, attack: &attack, hitpoints: &hitpoints, weapon: null, bag: null}
    player;
  }

  pub fn generate_weapon_data(WeaponRating: &rating) -> WeaponData{
    if &rating.eq(WeaponRating.Basic){
      //Generar arma con 1 d6 de bonus
      let names = Vec!["Small Sword", "Rusty Axe", "Wooden Spear", "Old Claws", "2 Small Daggers", ""];
      let weapon_data = WeaponData{rating: &rating, name: "", attack: roll_dice(1)}
    } else if &rating.eq(WeaponRating.Medium){
      //Generar arma con 2 d6 de bonus
      let weapon_data = WeaponData{rating: &rating, name: "", attack: roll_dice(2)}
    } else if &rating.eq(WeaponRating.Advanced){
      //Generar arma con 3 d6 de bonus
      let weapon_data = WeaponData{rating: &rating, name: "", attack: roll_dice(3)}
    }
  }

  pub fn generate_weapon_name(WeaponRating: &rating){
    if &rating.eq
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