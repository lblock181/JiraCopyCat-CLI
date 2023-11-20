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

fn create_epic_prompt() -> Epic {
    print_separator_line();
    println!("Epic Name:");
    let epic_name = get_user_input().trim().to_owned();
    println!("Epic description:");
    let epic_descr = get_user_input().trim().to_owned();
    Epic::new(epic_name, epic_descr)
}

fn create_story_prompt() -> Story {
    print_separator_line();
    println!("Story Name:");
    let story_name = get_user_input().trim().to_owned();
    println!("Story description:");
    let story_descr = get_user_input().trim().to_owned();
    Story::new(story_name, story_descr)
}

fn delete_epic_prompt() -> bool {
    print_separator_line();
    println!("Are you sure you want to delete the epic? All associated stories will also be deleted? Y/N: ");
    match get_user_input().trim().to_ascii_uppercase().as_str() {
        "Y" => true,
        _ => false,
    }
}

fn delete_story_prompt() -> bool {
    print_separator_line();
    println!("Are you sure you want to delete this story? Y/N: ");
    match get_user_input().trim().to_ascii_uppercase().as_str() {
        "Y" => true,
        _ => false,
    }
}

fn update_status_prompt() -> Option<Status> {
    print_separator_line();
    println!("Update status");
    println!("New Status (1-OPEN, 2-IN PROGRESS, 3-RESOLVED, 4-CLOSED");
    let status_resp = get_user_input().trim().parse::<u8>();
    if let Ok(status_resp) = status_resp {
        match status_resp {
            1 => {return Some(Status::Open);},
            2 => {return Some(Status::InProgress);},
            3 => {return Some(Status::Resolved);},
            4 => {return Some(Status::Closed);},
            _ => {return None;},
        }
    }
    None
}

fn print_separator_line() {
    println!("----------------------------\n");
}