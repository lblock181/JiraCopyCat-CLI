use std::{collections::HashMap, fmt::Display};
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum Status {
    Open,
    InProgress,
    Resolved,
    Closed
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Status::Open => write!(f, "OPEN"),
            Status::InProgress => write!(f, "IN PROGRESS"),
            Status::Resolved => write!(f, "RESOLVED"),
            Status::Closed => write!(f, "Closed"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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


#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct DBState {
    pub last_item_id: u32,
    pub epics: HashMap<u32, Epic>,
    pub stories: HashMap<u32, Story>,
}