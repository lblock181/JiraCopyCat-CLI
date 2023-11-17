use crate::{models::{Epic, Story, Status, Action}, io_utils::get_user_input};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>
}

impl Prompts {
    pub fn new() -> Self {
        Self { 
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt)
        }
    }
}

enum ActionMask {
    CreateEpic,
    DeleteEpic,
    UpdateEpic,
    CreateStory,
    DeleteStory,
    UpdateStory,
}

fn create_epic_prompt() -> Epic {
    println!("Epic Name:");
    let epic_name = get_user_input();
    println!("Epic description:");
    let epic_descr = get_user_input();
    Epic::new(epic_name, epic_descr)
}

fn create_story_prompt() -> Story {
    println!("Story Name:");
    let story_name = get_user_input();
    println!("Story description:");
    let story_descr = get_user_input();
    Story::new(story_name, story_descr)
}

fn delete_epic_prompt() -> bool {
    println!("Are you sure you want to delete the epic? All associated stories will also be deleted? Y/N: ");
    match get_user_input().to_ascii_uppercase().as_str() {
        "Y" => true,
        _ => false,
    }
}

fn delete_story_prompt() -> bool {
    println!("Are you sure you want to delete this story? Y/N: ");
    match get_user_input().to_ascii_uppercase().as_str() {
        "Y" => true,
        _ => false,
    }
}

fn update_status_prompt() -> Option<Status> {
    println!("Update status");
    println!("New Status (1-OPEN, 2-IN PROGRESS, 3-RESOLVED, 4-CLOSED");
    match get_user_input().parse::<i32>().unwrap() {
        1 => Some(Status::Open),
        2 => Some(Status::InProgress),
        3 => Some(Status::Resolved),
        4 => Some(Status::Closed),
        _ => None,
    }
}

fn build_boilerplate_display(action: ActionMask) -> String {
    let mut prompt_str: String = "----------------------------\n".to_owned();
    match action {
        ActionMask::CreateEpic => prompt_str.push_str("Epic"),
        ActionMask::DeleteEpic => prompt_str.push_str("Delete Epic"),
        ActionMask::UpdateEpic => prompt_str.push_str("Update Epic"),
        ActionMask::CreateStory => prompt_str.push_str("Create Story"),
        ActionMask::DeleteStory => prompt_str.push_str("Delete Story"),
        ActionMask::UpdateStory => prompt_str.push_str("Update Story"),
    }
    prompt_str
}