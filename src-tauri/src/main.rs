#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{collections::HashMap, sync::atomic::{AtomicUsize, Ordering}, path::Path, fs::OpenOptions, error::Error};
use pmrs::{objects::{ocel::{importer::import_ocel, exporter::{export_ocel_pretty, generate_ocel_external_repr}, OcelSerde}, ocdg::{Ocdg, generate_ocdg, Relations, importer::import_ocdg, exporter::export_ocdg}}, algo::transformation::ocel::features::object_point::{object_point_features, ObjectPointConfig, ObjectPoint}};
use polars::{prelude::{Series, DataFrame, NamedFrom, CsvWriter}, io::SerWriter};
use serde::{Serialize, Deserialize};
use strum::{IntoEnumIterator, EnumIter, EnumString};
use pmrs::objects::ocel::Ocel;
use pmrs::objects::ocel::validator::validate_ocel_verbose;
use pmrs::objects::ocdg::exporter::generate_ocdg_string;
use tauri::Manager;
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
    Ocdg(OcdgEntity),
    Table(TableEntity)
}

enum EntityPrimitive<'a> {
    Ocel(&'a Ocel),
    Ocdg(&'a Ocdg),
    Table(&'a DataFrame)
}

#[derive(Debug, EnumIter, EnumString, Clone)]
enum Plugins {
    GenerateOcdg,
    ValidateOcel,
    MergeFeaturesIntoOcel,
    AllObjectPointFeatures,
    UiDemo
}

#[derive(Serialize, Deserialize)]
struct Plugin {
    id: usize,
    name: String,
    description: String,
    total_steps: u8,
    enumid: String,
    #[serde(alias = "type", rename(serialize = "type"))]
    plugin_type: String,
    input: HashMap<String, Value>,
    output: HashMap<String, Value>,
    parameters: Vec<HashMap<String, Value>>

}

#[derive(Serialize, Deserialize)]
struct PluginParameters {
    enumid: String,
    inputs: HashMap<String, Vec<String>>,
    parameters: Vec<HashMap<String, Value>>

}

#[derive(Serialize, Clone)]
struct ProgressEmitter<'a>{
    current_task: &'a str,
    current_step: u8,
    total_steps: u8
}

fn share_progress(current_task: &str, current_step: &mut u8, total_steps: u8, handler: &tauri::AppHandle) { 
    handler.emit_all("progress", ProgressEmitter {current_task, current_step: *current_step, total_steps}).unwrap();
    *current_step = *current_step + 1;
}

#[tauri::command]
fn activate_plugin(params: PluginParameters, entitystate: tauri::State<EntityState>, handler: tauri::AppHandle) -> Result<String, String> {
    let plugin: Plugins = Plugins::from_str(&params.enumid).unwrap();
    let plugin_info: Plugin = get_plugin_info(&plugin).expect("Plugin was not implemented properly.");
    let mut curr_step: u8 = 1;
    let total_steps: u8 = plugin_info.total_steps + 2;
    share_progress(format!("Starting Plugin: {}", plugin_info.name.as_str()).as_str(), &mut curr_step, total_steps, &handler);
    let id = get_new_id();
    let mut metadata = Map::<String, Value>::new();
    let mut instancedata = Map::<String, Value>::new();
    metadata.entry("rust-id".to_string()).or_insert(Value::String(id.to_string()));
    metadata.entry("time-created".to_string()).or_insert(Value::String(Local::now().to_string()));

    match plugin {
        Plugins::GenerateOcdg => {
                // get the first ocel log in inputs
                let iocel: usize = params.inputs[&"ocel".to_string()][0].parse().unwrap();
                
                let mut state = entitystate.0.lock().unwrap();
                if let Entity::Ocel(ent) = &state[&iocel] {
                    let log = &ent.object;
                    let relation_array = &params.parameters[0]["multichoice:Relations"];
                    let relations: Vec<Relations> = relation_array.as_array().unwrap().iter().map(|i| Relations::from_str(i.as_str().unwrap()).unwrap()).collect();
                    share_progress("Generating OCDG", &mut curr_step, total_steps, &handler);
                    let ocdg: Ocdg = generate_ocdg(log, &relations);
                    instancedata.entry("Relations".to_string()).or_insert(relation_array.to_owned());
                    instancedata.extend(generate_default_instance_data(EntityPrimitive::Ocdg(&ocdg)));
                    metadata.entry("name".to_string()).or_insert(Value::String(format!("Ocdg {:?}", &id).to_string()));
                    metadata.entry("type".to_string()).or_insert(Value::String("ocdg".to_string()));
                    metadata.entry("type-long".to_string()).or_insert(Value::String("Object-Centric Directed Graph".to_string()));
                    share_progress("Storing OCDG", &mut curr_step, total_steps, &handler);
                    let new_ocdg = OcdgEntity {id, object: ocdg, metadata, instancedata};
                    state.entry(id).or_insert(Entity::Ocdg(new_ocdg)); 

                }
        },
        Plugins::ValidateOcel => {
            let mut state = entitystate.0.lock().unwrap();
            let path: &str = params.parameters[0]["file:ValidationFile"].as_str().unwrap();
            let df: DataFrame;
            
            metadata.entry("name".to_string()).or_insert(Value::String(format!("Ocel Validation {:?}", &id).to_string()));
            metadata.entry("type".to_string()).or_insert(Value::String("table".to_string()));
            metadata.entry("type-long".to_string()).or_insert(Value::String("DataFrame".to_string()));


            share_progress("Validating OCEL", &mut curr_step, total_steps, &handler);
            match validate_ocel_verbose(path) {
                Ok(a) => {
                    let mut err_reason: Vec<&str> = vec![];
                    let mut err_location: Vec<&str> = vec![];
                    share_progress("Storing Validation Result", &mut curr_step, total_steps, &handler);


                    a.iter().for_each(|(reason, location)| {err_reason.push(reason); err_location.push(location)});

                    df = DataFrame::new(vec![Series::new("Error Reason", err_reason), Series::new("Error Location", err_location)]).expect("Data Table Creation went wrong");

                    instancedata.extend(generate_default_instance_data(EntityPrimitive::Table(&df)));
                    let new_table = TableEntity{id, object: df, metadata, instancedata};
                    state.entry(id).or_insert(Entity::Table(new_table));
                },
                Err(error) => {
                    return Err(error.to_string());
                }
            }

        },
        Plugins::AllObjectPointFeatures => {
            let iocel: usize = params.inputs[&"ocel".to_string()][0].parse().unwrap();
            let iocdg: usize = params.inputs[&"ocdg".to_string()][0].parse().unwrap();
            let mut state = entitystate.0.lock().unwrap();
            if let Entity::Ocel(ocel) = &state[&iocel] {
                if let Entity::Ocdg(ocdg) = &state[&iocdg] {
                    let params: HashMap<ObjectPoint, Option<Value>> = HashMap::from_iter([(ObjectPoint::UniqueNeighborCount, None), (ObjectPoint::ActivityExistenceCount, None) , (ObjectPoint::ObjectLifetime, None), (ObjectPoint::ObjectEventInteractionOperator, None), (ObjectPoint::ObjectUnitSetRatio, None)]);
                    let feature_config: ObjectPointConfig = ObjectPointConfig { ocel: &ocel.object, ocdg: &ocdg.object, params: &params };
                    share_progress("Extracting Object Point Features", &mut curr_step, total_steps, &handler);
                    let df: DataFrame = object_point_features(feature_config);
                    share_progress("Storing Result as DataFrame", &mut curr_step, total_steps, &handler);
                    metadata.entry("name".to_string()).or_insert(json!(format!("Object Point Features {:?}", &id)));
                    metadata.entry("type".to_string()).or_insert(json!("table"));
                    metadata.entry("type-long".to_string()).or_insert(json!("DataFrame"));
                    instancedata.entry("ocel-used".to_string()).or_insert(json!(ocel.metadata["name"]));
                    instancedata.entry("ocdg-used".to_string()).or_insert(json!(ocdg.metadata["name"]));

                    instancedata.extend(generate_default_instance_data(EntityPrimitive::Table(&df)));
                    let new_table = TableEntity {id, object: df, metadata, instancedata};
                    state.entry(id).or_insert(Entity::Table(new_table));
                }
            }
        }, 
        Plugins::MergeFeaturesIntoOcel => {
            let iocel: usize = params.inputs[&"ocel".to_string()][0].parse().unwrap();
            let itable: usize = params.inputs[&"table".to_string()][0].parse().unwrap();
            let mut state = entitystate.0.lock().unwrap();
            if let Entity::Ocel(ocel) = &state[&iocel] {
                if let Entity::Table(table) = &state[&itable] {
                    share_progress("Merging DataFrame into OCEL", &mut curr_step, total_steps, &handler);
                    let mut new_ocel = ocel.object.clone();
                    let df: DataFrame = table.object.clone();
                    let cols = df.get_columns();
                    let oids: Vec<&str> = cols[0].utf8().unwrap().into_no_null_iter().collect();
                    for col_id in 1..cols.len() {
                        let curr_series = &cols[col_id];
                        let curr_name = curr_series.name();
                        curr_series.iter().enumerate().for_each(|(index, value)| {
                            let oid = ocel.object.object_map.get_by_left(oids[index]).unwrap();
                            let number_value: Value = (json!(value)).as_object().expect("This can't fail").values().next().expect("This can't fail").to_owned();
                            new_ocel.objects.get_mut(oid).expect("This can't fail").ovmap.entry(curr_name.to_string()).or_insert(number_value);
                        });
                    }

                    metadata.entry("type".to_string()).or_insert(Value::String("ocel".to_string()));
                    metadata.entry("type-long".to_string()).or_insert(Value::String("Object-Centric Event Log".to_string()));
                    metadata.entry("file-type".to_string()).or_insert(Value::String("jsonocel".to_string()));
                    instancedata.extend(generate_default_instance_data(EntityPrimitive::Ocel(&new_ocel)));

                    let new_ocel: OcelEntity = OcelEntity { id, object: new_ocel, metadata, instancedata };
                    state.entry(id).or_insert(Entity::Ocel(new_ocel));

                }
            }
            share_progress("Storing new OCEL log", &mut curr_step, total_steps, &handler);

            if let Value::Bool(consume) = params.parameters[0]["bool:ConsumeEntities"] {
                if consume {
                    todo!()
                }
            }

        },
        Plugins::UiDemo => {

        },
    }

    share_progress(format!("Finished Plugin: {}", plugin_info.name.as_str()).as_str(), &mut curr_step, total_steps, &handler);
    Ok(id.to_string())
}

fn get_plugin_info(plugin: &Plugins) -> Option<Plugin> {
    match plugin {
        Plugins::GenerateOcdg => {
            let plug = r#"{
                "id": 1,
                "name": "Generate Ocdg",
                "total_steps": 2,
                "enumid": "GenerateOcdg",
                "description": "Generate an Object-Centric Directed Graph with specified relations.",
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
        Plugins::ValidateOcel => {
            let plug = r#"{
                "id": 2,
                "name": "Validate Ocel",
                "total_steps": 2,
                "enumid": "ValidateOcel",
                "description": "Validates the OCEL input file and returns all errors that exist with the document",
                "type": "Validation",
                "input": {},
                "output": {"table": 1},
                "parameters": [{"header": "General", "file:ValidationFile": ""}]
            }"#;

            let val_ocel: Plugin = serde_json::from_str(plug).expect("This should never crash");
            return Some(val_ocel);
        },
        Plugins::AllObjectPointFeatures => {
        let plug = r#"{
                "id": 3,
                "name": "Generate all Object Point Features (oid intersection)",
                "total_steps": 2,
                "enumid": "AllObjectPointFeatures",
                "description": "Generate all object features based on default values. This plugin only returns features of objects that are in both the ocel and ocdg.",
                "type": "Feature Extraction",
                "input": {"ocel": 1, "ocdg": 1},
                "output": {"table": 1},
                "parameters": []
            }"#;

            let val_ocel: Plugin = serde_json::from_str(plug).expect("This should never crash");
            return Some(val_ocel);
        },
        Plugins::MergeFeaturesIntoOcel => {
        let plug = r#"{
                "id": 4,
                "name": "Merge Feature Table into Ocel log",
                "total_steps": 2,
                "enumid": "MergeFeaturesIntoOcel",
                "description": "Merge a feature table into an ocel log. If ConsumeEntities is true, the input objects are consumed to create the merged log. If false, the result is the result of cloning the data.",
                "type": "Combination",
                "input": {"ocel": 1, "table": 1},
                "output": {"ocel": 1},
                "parameters": [{"header": "General", "bool:ConsumeEntities": true}]
            }"#;

            let val_ocel: Plugin = serde_json::from_str(plug).expect("This should never crash");
            return Some(val_ocel);

            
        },
        Plugins::UiDemo => {
        let plug = r#"{
                "id": 5,
                "name": "UI Demo",
                "total_steps": 0,
                "enumid": "UiDemo",
                "description": "Plugin to showcase all the user interface options that are available.",
                "type": "Demo",
                "input": {},
                "output": {},
                "parameters": [{"header": "General", "string:Normal String": "default input", "number: Number input": 123.321, "multichoice:Multiple Choice!!": ["multiple", "selection", "options"], "bool:boolean selection": false, "file:Select a file!": "", "dropdown:Drop down selection!": ["only", "select", "one"], "slider:slider min,max,step,initial": [0.0, 1.0, 0.01, 0.5]}]
            }"#;

            let val_ocel: Plugin = serde_json::from_str(plug).expect("This should never crash");
            return Some(val_ocel);
        },
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
            },
            Entity::Table(ent) => {
                instance.entry("metadata".to_string()).or_insert(serde_json::Value::Object(ent.metadata.clone()));
                instance.entry("instancedata".to_string()).or_insert(Value::Object(ent.instancedata.clone()));
            }
        }
            
        instance

    }

    fn get_analysis_view(&self) -> Result<String, Box<dyn Error>> {
        match self {
            Entity::Ocel(ent) => {
                if ent.object.object_map.len() < 10 && ent.object.event_map.len() < 100 {
                    let ocel_repr: OcelSerde = generate_ocel_external_repr(&ent.object);
                    match serde_json::to_string(&ocel_repr) {
                        Ok(ent_str) => {return Ok(ent_str);},
                        Err(e) => {return Err(e.into());}
                    }
                }
            },
            Entity::Ocdg(ent) => {
                if ent.object.object_map.len() < 20 { 
                    return generate_ocdg_string(&ent.object);
                }
            },
            Entity::Table(ent) => {
                if ent.object.shape().0 < 500 {
                    match serde_json::to_string(&ent.object) {
                        Ok(ent_str) => {return Ok(ent_str);},
                        Err(e) => {return Err(e.into());}
                    }
                }
            }
        }

        Ok("na".to_string())
    }
}


struct EntityState(Mutex<HashMap<usize, Entity>>);

pub struct OcelEntity {
    pub id: usize,
    pub object: Ocel,
    metadata: Map<String, Value>,
    instancedata: Map<String, Value>
}

pub struct OcdgEntity {
    pub id: usize,
    pub object: Ocdg,
    metadata: Map<String, Value>,
    instancedata: Map<String, Value>
}

pub struct TableEntity {
    pub id: usize,
    pub object: DataFrame,
    metadata: Map<String, Value>,
    instancedata: Map<String, Value>
}


#[tauri::command]
fn get_analysis_view(rust_id: usize, entitystate: tauri::State<EntityState>) -> Result<String, String> {
    let rust_objs = entitystate.0.lock().unwrap();
    if let Some(entity) = rust_objs.get(&rust_id) {
        match entity.get_analysis_view() {
            Ok(ent_str) => {return Ok(ent_str);},
            Err(e) => {return Err(e.to_string());}
        }
    }
    Err(format!("The rust id {} could not be found", rust_id).to_string())
}

#[tauri::command]
fn export_entity(rust_id: usize, filepath: &str, entitystate: tauri::State<EntityState>) -> Result<String, String> {
    let mut rust_objs = entitystate.0.lock().unwrap();

    if let Some(entity) = rust_objs.get_mut(&rust_id) {
        match entity {
            Entity::Ocel(ocel) => {
                match export_ocel_pretty(&ocel.object, filepath) {
                    Ok(_) => {return Ok(filepath.to_string())},
                    Err(e) => {return Err(e.to_string())}
                }
            },
            Entity::Ocdg(ocdg) => {
                match export_ocdg(&ocdg.object, filepath) {
                    Ok(_) => {return Ok(filepath.to_string())},
                    Err(e) => {return Err(e.to_string())}
                }


            },
            Entity::Table(table) => {
                match OpenOptions::new().create(true).write(true).truncate(true).open(filepath) {
                    Ok(output_file) => {
                        CsvWriter::new(output_file)
                            .has_header(true)
                            .with_delimiter(b',')
                            .finish(&mut table.object).unwrap();

                            return Ok(filepath.to_string());
                    },
                    Err(e) => {return Err(e.to_string())}
                }
            }
        }
    }

    Err("Rust ID could not be found".to_string())


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
            metadata.entry("time-imported".to_string()).or_insert(Value::String(format!("{}", Local::now())));
            metadata.entry("file-size".to_string()).or_insert(Value::String(fsmeta.len().to_string()));

            match e.to_str().unwrap() {
                "jsonocel" => {
                   match import_ocel(&filepath) {
                       Ok(ocel) => {
                        metadata.entry("type".to_string()).or_insert(Value::String("ocel".to_string()));
                        metadata.entry("type-long".to_string()).or_insert(Value::String("Object-Centric Event Log".to_string()));
                        metadata.entry("file-type".to_string()).or_insert(Value::String("jsonocel".to_string()));
                        instancedata.extend(generate_default_instance_data(EntityPrimitive::Ocel(&ocel)));

                        let ocel_entity = OcelEntity {id, object: ocel, metadata, instancedata};

                        let mut state = entitystate.0.lock().unwrap();
                        
                        state.entry(id).or_insert(Entity::Ocel(ocel_entity));
                        Ok(id.to_string())
                       },
                       Err(e) => {
                            Err(format!("{:?} -> {:?}", "File Import Fail", e).to_string())
                       }

                   }
                },
                "gexfocdg"|"gexf" => {
                    match import_ocdg(&filepath) {
                        Ok(ocdg) => {
                            metadata.entry("type".to_string()).or_insert(Value::String("ocdg".to_string())); 
                            metadata.entry("type-long".to_string()).or_insert(Value::String("Object-Centric Directed Graph".to_string()));
                            metadata.entry("file-type".to_string()).or_insert(Value::String("gexfocdg".to_string()));
                            instancedata.extend(generate_default_instance_data(EntityPrimitive::Ocdg(&ocdg)));

                            let ocdg_entity = OcdgEntity {id, object: ocdg, metadata, instancedata};

                            let mut state = entitystate.0.lock().unwrap();

                            state.entry(id).or_insert(Entity::Ocdg(ocdg_entity));
                            Ok(id.to_string())
                        },
                        Err(e) => {
                            Err(format!("{:?} -> {:?}", "File Import Fail", e).to_string())
                            
                        }
                    }
                }
                _ => {Err("File Extension Fail.".to_string())},
            }
        },
        None => {Err("File Extension Fail.".to_string())}
    }
    
}

fn generate_default_instance_data(entity: EntityPrimitive) -> Vec<(String, Value)> {
    let mut instancedata: Vec<(String, Value)> = vec![];
    match entity {
        EntityPrimitive::Ocel(ocel) => {
            instancedata.push(("Event #".to_string(), json!(ocel.events.len())));
            instancedata.push(("Object #".to_string(), json!(ocel.objects.len())));
            instancedata.push(("Activities".to_string(), json!(ocel.activities)));
            instancedata.push(("Object Types".to_string(), match ocel.global_log.get("ocel:object-types") {Some(v) => {v.to_owned()}, None => {json!("None?")}}));
        },
        EntityPrimitive::Ocdg(ocdg) => {
            instancedata.push(("Node #".to_string(), json!(ocdg.net.node_count()))); 
            instancedata.push(("Edge #".to_string(), json!(ocdg.net.edge_count()))); 
        },
        EntityPrimitive::Table(df) => {
            instancedata.push(("rows".to_string(), json!(df.shape().0)));
            instancedata.push(("columns".to_string(), json!(df.shape().1)));
        }
    }
    instancedata
}

#[tauri::command]
fn get_view(rust_id: usize, entitystate: tauri::State<EntityState>) -> Result<String, String> {
    let mut rust_objs = entitystate.0.lock().unwrap();

    if let Some(entity) = rust_objs.get_mut(&rust_id) {
        match entity {
            Entity::Ocel(_ocel) => {todo!()},
            Entity::Ocdg(_ocdg) => {todo!()},
            Entity::Table(table) => {
            match serde_json::to_string(&table.object) {
                Ok(v) => {return Ok(v);},
                Err(e) => {return Err(e.to_string());}
                }
            }
        }
    }
    Err("rust-id does not exist?!".to_string())
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
        if let Some(plug) = get_plugin_info(&plugin) {
            plugvec.push(plug)
        }
    };

    plugvec
}



fn main() {
  let context = tauri::generate_context!();
  tauri::Builder::default()
    .manage(EntityState(Default::default()))
    .invoke_handler(tauri::generate_handler![import_entity, export_entity, get_instance_info, get_analysis_view, get_plugins, get_view, activate_plugin])
    .menu(tauri::Menu::os_default(&context.package_info().name))
    .run(context)
    .expect("error while running tauri application");
}
