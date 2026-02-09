//! Goals Plugin
//! 
//! Manages goals and goal tracking

use time_tracker_plugin_sdk::{Plugin, PluginInfo, PluginAPIInterface, EntityType, SchemaChange};
use serde_json;

pub struct GoalsPlugin {
    info: PluginInfo,
}

impl GoalsPlugin {
    pub fn new() -> Self {
        Self {
            info: PluginInfo {
                id: "goals-plugin".to_string(),
                name: "Goals".to_string(),
                version: "1.0.0".to_string(),
                description: Some("Goal tracking".to_string()),
                is_builtin: false,
            },
        }
    }
}

impl Plugin for GoalsPlugin {
    fn info(&self) -> &PluginInfo {
        &self.info
    }
    
    fn initialize(&mut self, api: &dyn PluginAPIInterface) -> Result<(), String> {
        // Register schema extension for goals table
        use time_tracker_plugin_sdk::extensions::{TableColumn, ForeignKey};
        
        api.register_schema_extension(
            EntityType::Activity, // Using Activity as entity type, though goals is separate
            vec![
                SchemaChange::CreateTable {
                    table: "goals".to_string(),
                    columns: vec![
                        TableColumn {
                            name: "id".to_string(),
                            column_type: "INTEGER PRIMARY KEY AUTOINCREMENT".to_string(),
                            default: None,
                            foreign_key: None,
                            nullable: false,
                            primary_key: true,
                        },
                        TableColumn {
                            name: "goal_type".to_string(),
                            column_type: "TEXT NOT NULL".to_string(),
                            default: None,
                            foreign_key: None,
                            nullable: false,
                            primary_key: false,
                        },
                        TableColumn {
                            name: "target_seconds".to_string(),
                            column_type: "INTEGER NOT NULL".to_string(),
                            default: None,
                            foreign_key: None,
                            nullable: false,
                            primary_key: false,
                        },
                        TableColumn {
                            name: "category_id".to_string(),
                            column_type: "INTEGER".to_string(),
                            default: None,
                            foreign_key: Some(ForeignKey {
                                table: "categories".to_string(),
                                column: "id".to_string(),
                            }),
                            nullable: true,
                            primary_key: false,
                        },
                        TableColumn {
                            name: "project_id".to_string(),
                            column_type: "INTEGER".to_string(),
                            default: None,
                            foreign_key: Some(ForeignKey {
                                table: "projects".to_string(),
                                column: "id".to_string(),
                            }),
                            nullable: true,
                            primary_key: false,
                        },
                        TableColumn {
                            name: "start_date".to_string(),
                            column_type: "INTEGER NOT NULL".to_string(),
                            default: None,
                            foreign_key: None,
                            nullable: false,
                            primary_key: false,
                        },
                        TableColumn {
                            name: "end_date".to_string(),
                            column_type: "INTEGER".to_string(),
                            default: None,
                            foreign_key: None,
                            nullable: true,
                            primary_key: false,
                        },
                        TableColumn {
                            name: "created_at".to_string(),
                            column_type: "INTEGER NOT NULL".to_string(),
                            default: None,
                            foreign_key: None,
                            nullable: false,
                            primary_key: false,
                        },
                        TableColumn {
                            name: "active".to_string(),
                            column_type: "BOOLEAN DEFAULT TRUE".to_string(),
                            default: Some("TRUE".to_string()),
                            foreign_key: None,
                            nullable: true,
                            primary_key: false,
                        },
                        TableColumn {
                            name: "name".to_string(),
                            column_type: "TEXT".to_string(),
                            default: None,
                            foreign_key: None,
                            nullable: true,
                            primary_key: false,
                        },
                    ],
                },
            ],
        )?;
        
        Ok(())
    }
    
    fn invoke_command(&self, command: &str, params: serde_json::Value, api: &dyn PluginAPIInterface) -> Result<serde_json::Value, String> {
        match command {
            "create_goal" => api.call_db_method("create_goal", params),
            "get_goals" => api.call_db_method("get_goals", params),
            "update_goal" => api.call_db_method("update_goal", params),
            "delete_goal" => api.call_db_method("delete_goal", params),
            "get_goal_progress" => api.call_db_method("get_goal_progress", params),
            "check_goal_alerts" => api.call_db_method("check_goal_alerts", params),
            _ => Err(format!("Unknown command: {}", command)),
        }
    }
    
    fn shutdown(&self) -> Result<(), String> {
        Ok(())
    }
}

// FFI exports for dynamic loading
#[no_mangle]
pub extern "C" fn _plugin_create() -> *mut dyn Plugin {
    Box::into_raw(Box::new(GoalsPlugin::new()))
}

#[no_mangle]
pub extern "C" fn _plugin_destroy(plugin: *mut dyn Plugin) {
    unsafe {
        let _ = Box::from_raw(plugin);
    }
}
