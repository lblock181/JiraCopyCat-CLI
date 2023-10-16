use std::collections::HashMap;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Epic {
    pub name: String,
    pub description: String,
    pub status: Status,
    pub stories: Vec<u32>,
}

impl Epic {
    pub fn new(name: String, description: String) -> Self {
        Self { 
            name,
            description,
            status: Status::Open,
            stories: vec![]
        }
        }
    }


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Story {
    pub name: String,
    pub description: String,
    pub status: Status,
}

impl Story {
    pub fn new(name: String, description: String) -> Self {
        Self {
            name,
            description,
            status: Status::Open,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}