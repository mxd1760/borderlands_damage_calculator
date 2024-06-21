use std::{any, path::Path};

use eframe::egui::{containers, widgets, Widget};
use serde::{Deserialize, Serialize};

#[derive(PartialEq)]
enum DamageCalculationMethod{
  JustDamage,
  DamageXFireRate,
  DamageXFireRateMinusReload,
}

impl DamageCalculationMethod{
  fn calc_damage(&self,gun:&Gun) -> f64{
    match *self{
        DamageCalculationMethod::JustDamage => {
          gun.damage as f64
        },
        DamageCalculationMethod::DamageXFireRate => {
          gun.damage as f64 * gun.fire_rate
        },
        DamageCalculationMethod::DamageXFireRateMinusReload => {
          let time_shooting = gun.magazine_size as f64/gun.fire_rate;
          (gun.damage as f64 * gun.fire_rate)*(time_shooting/(time_shooting+gun.reload_speed)) as f64
        },
    }
  }
}

enum View{
  Default,
  EnterGun,
  ShowDPS,
  SavedGunList,
}

#[derive(Serialize,Deserialize,PartialEq, Clone, Debug)]
enum GunType{
  Pistol,
  Shotgun,
  Sniper,
  SMG,
  AssultRifle,
  RocketLauncher,
  LazerGun,
}


impl GunType{
  fn as_text(&self) -> &str{
    match *self{
        GunType::Pistol => "Pistol",
        GunType::Shotgun => "Shotgun",
        GunType::Sniper => "Sniper",
        GunType::SMG => "SMG",
        GunType::AssultRifle => "Assult Rifle",
        GunType::RocketLauncher => "Rocket Launcher",
        GunType::LazerGun => "Lazer Weapon",
    }
  }
}

enum Game{
  Borderlands,
  BorderlandsGOTY,
  BorderlandsPreSequel,
  Borderlands2,
  Borderlands3,
  TinyTinasWonderland,
}
#[derive(Serialize,Deserialize,Clone, Debug)]
struct ElementalInfo{
  //TODO
}

#[derive(Serialize,Deserialize,Clone, Debug)]
struct Gun{
  name:String,
  gun_type:GunType,
  damage:i32,
  accuracy:f64,
  fire_rate:f64,
  reload_speed:f64,
  magazine_size:i16,
  elemental_info:Option<ElementalInfo>,
  other:Vec<String>,
}

impl Gun{
}

// impl Serialize for Gun{
//   fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//       where
//           S: serde::Serializer {
//       serializer.serialize_bytes(self.name.as_bytes());
//       self.gun_type.serialize(serializer);
//       serializer.serialize_i32(self.damage);
//       serializer.serialize_f64(self.accuracy);
//       serializer.serialize_f64(self.fire_rate);
//       serializer.serialize_f64(self.reload_speed);
//       serializer.serialize_i16(self.magazine_size);
//       self.elemental_info.serialize(serializer);
//       self.other.serialize(serializer)
//   }
// }

// struct ForeVisitor;

// impl<'de> serde::de::Visitor<'de> for ForeVisitor{
//   type Value = any;
//   fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
//       formatter.write_str("i shouldn't need to write 100 different visitors if serde is doing the encoding")
//   }
//   fn visit_bytes<E>(self,value:&[u8]) -> Result<self
// }

// impl Deserialize<'de> for Gun{
//   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//       where
//           D: serde::Deserializer<'de> {
//     Self{
//       name:String.from(deserializer.deserialize_bytes(ForeVisitor)?);
//     }
//   }
// }


impl Default for Gun{
  fn default() -> Self {
    Self { name: "".into(),
      gun_type: GunType::Pistol, 
      damage: 10, 
      accuracy: 10., 
      fire_rate: 10., 
      reload_speed: 10., 
      magazine_size: 10, 
      elemental_info: None, 
      other: vec![] 
    }
  }
}

struct MyApp{
  current_view:View,
  current_gun:Gun,
  current_game:Game,
  damage_calculation_method:DamageCalculationMethod,
  gun_list:Vec<Gun>
}

impl MyApp{

}

// Helper Functions
fn load_gunlist() -> Vec<Gun>{
  println!("Wow");
  if let Some(gun_file) = get_gunfile(){
    if let Ok(file_obj) = std::fs::File::open(gun_file){
      // let mut data = Vec::new();
      // file.read_to_end(&mut data).expect("unable to read data");
      let decoded:Option<Vec<Gun>> = bincode::deserialize_from(file_obj).expect("unable to read data");
      match decoded{
        Some(guns) => {
          println!("{:?}", guns);
          return guns;
        },
        None => {println!("No Guns???!!!")}
      }
    }else{
      println!("file not open")
    }
  }else{
    println!("file path not found")
  }
  vec![]
}

fn get_gunfile() -> Option<std::path::PathBuf>{
  if let Some(mut load_dir) = dirs::data_dir(){
    load_dir.push("MDM");
    load_dir.push("Borderlands Damage Calculator");
    load_dir.push("guns_file.gunfi");
    println!("{:?}",load_dir);
    Some(load_dir)
  }else{
    None
  }
}

fn save_gunlist(guns:&Vec<Gun>) {
  if let Some(gun_file) = get_gunfile(){
    std::fs::create_dir_all(gun_file.parent().unwrap()).unwrap();
    if let Ok(file_obj) = std::fs::File::create(gun_file){
      bincode::serialize_into(file_obj, guns).expect("Failed to write");
    }
    
  }
}

fn pair(ui:&mut eframe::egui::Ui,label:&str,widget:impl Widget) -> eframe::egui::InnerResponse<()>{
  ui.horizontal(|ui|{
    ui.label(label);
    ui.add(widget);
  })
}

impl Default for MyApp{
  fn default() -> Self {
    Self{
      current_view:View::Default,
      current_gun:Gun::default(),
      current_game:Game::BorderlandsPreSequel,
      damage_calculation_method: DamageCalculationMethod::DamageXFireRate,
      gun_list: load_gunlist(),
    }
  }
}

impl eframe::App for MyApp{
  fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {

    
    eframe::egui::CentralPanel::default().show(ctx,|ui|{
      ui.horizontal(|ui|{
        if ui.button("Home").clicked(){
          self.current_view = View::Default;
        }
        if ui.button("Enter Gun").clicked(){
          self.current_view = View::EnterGun;
        }
        if ui.button("DPS Calculator").clicked(){
          self.current_view = View::ShowDPS;
        }
        if ui.button("Gun List").clicked(){
          self.current_view = View::SavedGunList;
        }
      });
      match self.current_view{
        View::Default => {
          ui.label("Welcome Back");
        },
        View::EnterGun => {
          let gun = &mut self.current_gun;
          ui.text_edit_singleline(&mut gun.name);
          containers::ComboBox::from_label("")
          .selected_text(gun.gun_type.as_text())
          .show_ui(ui,|ui| {
            ui.selectable_value(&mut gun.gun_type, GunType::Pistol,         GunType::Pistol.as_text());
            ui.selectable_value(&mut gun.gun_type, GunType::Shotgun,        GunType::Shotgun.as_text());
            ui.selectable_value(&mut gun.gun_type, GunType::Sniper,         GunType::Sniper.as_text());
            ui.selectable_value(&mut gun.gun_type, GunType::AssultRifle,    GunType::AssultRifle.as_text());
            ui.selectable_value(&mut gun.gun_type, GunType::SMG,            GunType::SMG.as_text());
            ui.selectable_value(&mut gun.gun_type, GunType::RocketLauncher, GunType::RocketLauncher.as_text());
            ui.selectable_value(&mut gun.gun_type, GunType::LazerGun,       GunType::LazerGun.as_text());
          });
          pair(ui,"Damage",widgets::DragValue::new(&mut gun.damage));
          pair(ui,"Accuracy",widgets::DragValue::new(&mut gun.accuracy)
          .fixed_decimals(1)
          .speed(0.1));
          pair(ui,"Fire Rate",widgets::DragValue::new(&mut gun.fire_rate)
          .fixed_decimals(1)
          .speed(0.1));
          pair(ui,"Reload Speed",widgets::DragValue::new(&mut gun.reload_speed)
          .fixed_decimals(1)
          .speed(0.1));
          pair(ui,"Magazine Size",widgets::DragValue::new(&mut gun.magazine_size));
          ui.label("TODO ELEMENTAL");
          ui.label("TODO BONUS STATS");
          
          if ui.button("Save data").clicked(){
            let gun = self.current_gun.clone();
            self.gun_list.push(gun);
            save_gunlist(&self.gun_list)
            // println!("{:?}", self.gun_list);
          }
        },
        View::ShowDPS => {
          ui.horizontal(|ui|{
            ui.radio_value(&mut self.damage_calculation_method,DamageCalculationMethod::JustDamage,"Just Damage");
            ui.radio_value(&mut self.damage_calculation_method,DamageCalculationMethod::DamageXFireRate,"Instantanous DPS");
            ui.radio_value(&mut self.damage_calculation_method,DamageCalculationMethod::DamageXFireRateMinusReload,"Aggregate DPS (w/ reload)");
          });
          ui.label(format!("Your DPS is {:.2}",self.damage_calculation_method.calc_damage(&self.current_gun)));
        },
        View::SavedGunList =>{
          for i in &self.gun_list{
            let copy = i.clone();
            if ui.button(copy.name).clicked(){
              let copy2 = i.clone();
              self.current_gun = copy2;
            }
          }
        }
      }
      
    });
  }
}


fn main() -> Result<(),eframe::Error> {
  let native_options = eframe::NativeOptions{
    viewport: eframe::egui::ViewportBuilder::default().with_inner_size([400.,400.]),
    ..Default::default()
  };
  eframe::run_native("Borderlands Damage Calculator", 
    native_options,
    Box::new(|_| Box::<MyApp>::default())
  )
}
