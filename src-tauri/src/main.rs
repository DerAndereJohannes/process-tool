#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}, path::Path, hash::Hash};
use pmrs::objects::{ocel::importer::import_ocel, ocdg::{Ocdg, generate_ocdg, Relations}};
use serde::{Serialize, Deserialize};
use strum::{IntoEnumIterator, EnumIter};
use pmrs::objects::ocel::Ocel;
use std::str::FromStr;
use serde_json::{Value, Map, json};
use std::sync::Mutex;
use std::fs;
use chrono::Local;

static COUNTER: AtomicUsize = AtomicUsize::new(1);

fn get_new_id() -> usize {
    COUNTER.fetch_add(1, Ordering::Relaxed)
}


enum Entity {
    Ocel(OcelEntity),
    Ocdg(OcdgEntity)
}

#[derive(Debug, EnumIter)]
enum Plugins {
    GenerateOcdg,
    ApplyToOcel
}

#[derive(Serialize, Deserialize)]
struct Plugin {
    name: String,
    #[serde(alias = "type", rename(serialize = "type"))]
    plugin_type: String,
    input: HashMap<String, Value>,
    output: HashMap<String, Value>,
    parameters: Vec<HashMap<String, Value>>

}

#[tauri::command]
fn activate_plugin(plugin: Plugins, inputs: HashMap<String, Value>, entitystate: tauri::State<EntityState>) -> Result<String, String> {
    match plugin {
        Plugins::GenerateOcdg => {
            if inputs.contains_key(&"Ocel".to_string()) && inputs.contains_key(&"Relations".to_string()) {
                let iocdg: usize = inputs[&"Ocel".to_string()].as_u64().unwrap().try_into().unwrap();
                
                let mut state = entitystate.0.lock().unwrap();
                if let Entity::Ocel(ent) = &state[&iocdg] {
                    let log = &ent.object;
                    let relations: Vec<Relations> = inputs[&"Relations".to_string()].as_array().unwrap().iter().map(|i| Relations::from_str(&i.to_string()).unwrap()).collect();
                    let ocdg: Ocdg = generate_ocdg(log, &relations);
                    let id = get_new_id();
                    let mut metadata = Map::<String, Value>::new();
                    let mut instancedata = Map::<String, Value>::new();
                    metadata.entry("rust-id".to_string()).or_insert(Value::String(id.to_string()));
                    metadata.entry("name".to_string()).or_insert(Value::String(format!("Ocdg {:?}", &id).to_string()));
                    metadata.entry("time-imported".to_string()).or_insert(Value::String(Local::now().to_string()));
                    let new_ocdg = OcdgEntity {id, object: ocdg, metadata, instancedata};
                    state.entry(id).or_insert(Entity::Ocdg(new_ocdg)); 

                }
                
            }
            Ok("".to_string())
        },
        _ => Err("plugin does not exist".to_string())
    }
}


fn get_plugin_info(plugin:Plugins) -> Option<Plugin> {
    match plugin {
        Plugins::GenerateOcdg => {
            let plug = r#"{
                "name": "Generate Ocdg",
                "type": "Generation",
                "input": {"ocel": 1},
                "output": {"ocdg": 1},
                "parameters": []
            }"#;
            let mut gen_ocdg: Plugin = serde_json::from_str(plug).unwrap();
            let parameters: HashMap<String, Value> = HashMap::from([("multichoice:Relations".to_string(), serde_json::to_value(Relations::iter().map(|rel| format!("{:?}", rel)).collect::<Vec<String>>()).unwrap()), ("header".to_string(), Value::String("General".to_string()))]);
            gen_ocdg.parameters.push(parameters);
            return Some(gen_ocdg);
        },
        _ => None
    }

}

impl Entity {
    fn get_info(&self) -> HashMap<String, Value> {
        let mut instance = HashMap::<String, Value>::new();

        match self {
            Entity::Ocel(ent) => {
                instance.entry("metadata".to_string()).or_insert(serde_json::Value::Object(ent.metadata.clone()));
                instance.entry("instancedata".to_string()).or_insert(Value::Object(ent.instancedata.clone()));
            },
            Entity::Ocdg(ent) => {
                instance.entry("metadata".to_string()).or_insert(serde_json::Value::Object(ent.metadata.clone()));
                instance.entry("instancedata".to_string()).or_insert(Value::Object(ent.instancedata.clone()));
            }
        }
            
        instance

    }
}


struct EntityState(Mutex<HashMap<usize, Entity>>);

#[derive(Debug)]
struct OcelEntity {
    id: usize,
    object: Ocel,
    metadata: Map<String, Value>,
    instancedata: Map<String, Value>
}

#[derive(Debug)]
struct OcdgEntity {
    id: usize,
    object: Ocdg,
    metadata: Map<String, Value>,
    instancedata: Map<String, Value>
}


#[tauri::command]
fn import_entity(filepath: &str, entitystate: tauri::State<EntityState>) -> Result<String, String> {
    let path_instance = Path::new(filepath);
    let ext = path_instance.extension();
    let name = path_instance.file_stem();

    match ext {
        Some(e) => {
            let id = get_new_id();
            let mut metadata = Map::<String, Value>::new();
            let mut instancedata = Map::<String, Value>::new();
            let fsmeta: fs::Metadata = fs::metadata(&filepath).unwrap();
            metadata.entry("rust-id".to_string()).or_insert(Value::String(id.to_string()));
            metadata.entry("name".to_string()).or_insert(Value::String(name.unwrap().to_str().unwrap().to_string()));
            metadata.entry("time-imported".to_string()).or_insert(Value::String(format!("{:?}", fsmeta.accessed().unwrap())));
            metadata.entry("file-size".to_string()).or_insert(Value::String(fsmeta.len().to_string()));

            match e.to_str().unwrap() {
                "jsonocel" => {
                   match import_ocel(&filepath) {
                       Ok(ocel) => {
                        let new_ocel = ocel;
                        metadata.entry("type".to_string()).or_insert(Value::String("ocel".to_string()));
                        metadata.entry("type-long".to_string()).or_insert(Value::String("Object-Centric Event Log".to_string()));
                        metadata.entry("file-type".to_string()).or_insert(Value::String("jsonocel".to_string()));

                        let ocel_entity = OcelEntity {id, object: new_ocel, metadata, instancedata};

                        let mut state = entitystate.0.lock().unwrap();
                        
                        state.entry(id).or_insert(Entity::Ocel(ocel_entity));
                        Ok(id.to_string())
                       },
                       Err(e) => {
                            Err(format!("{:?} -> {:?}", "File Import Fail", e).to_string())
                       }

                   }
                },
                "gexf" => {
                    todo!()
                }
                _ => {Err("File Extension Fail.".to_string())},
            }
        },
        None => {Err("File Extension Fail.".to_string())}
    }
    
}

#[tauri::command]
fn get_instance_info(instance_id: usize, entitystate: tauri::State<EntityState>) -> Result<HashMap<String, Value>, String> {
    let state = entitystate.0.lock().unwrap();
    let entity = state.get(&instance_id).unwrap();
    Ok(entity.get_info())
}

#[tauri::command]
fn get_plugins() -> Vec<Plugin> {
    let mut plugvec = vec![];
    for plugin in Plugins::iter() {
        if let Some(plug) = get_plugin_info(plugin) {
            plugvec.push(plug)
        }
    };

    plugvec
}



fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .manage(EntityState(Default::default()))
    .invoke_handler(tauri::generate_handler![import_entity, get_instance_info, get_plugins])
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
