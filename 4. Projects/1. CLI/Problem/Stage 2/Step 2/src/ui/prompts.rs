use crate::{
    io_utils::get_user_input,
    models::{Epic, Status, Story},
};

pub struct Prompts {
    pub create_epic: Box<dyn Fn() -> Epic>,
    pub create_story: Box<dyn Fn() -> Story>,
    pub delete_epic: Box<dyn Fn() -> bool>,
    pub delete_story: Box<dyn Fn() -> bool>,
    pub update_status: Box<dyn Fn() -> Option<Status>>,
}

impl Prompts {
    pub fn new() -> Self {
        Self {
            create_epic: Box::new(create_epic_prompt),
            create_story: Box::new(create_story_prompt),
            delete_epic: Box::new(delete_epic_prompt),
            delete_story: Box::new(delete_story_prompt),
            update_status: Box::new(update_status_prompt),
        }
    }
}

fn create_epic_prompt() -> Epic {
    println!("Create epic:");
    let name = get_user_input();
    println!("Epic description:");
    let description = get_user_input();
    Epic::new(name, description)
}

fn create_story_prompt() -> Story {
    println!("Create story:");
    let name = get_user_input().trim().to_owned();
    println!("Story description:");
    let description = get_user_input().trim().to_owned();
    Story::new(name, description)
}

fn delete_epic_prompt() -> bool {
    println!("Are you sure you want to delete this epic? All stories in this epic will also be deleted [Y/n]:");
    match get_user_input().trim() {
        "Y" => true,
        "N" => false,
        _ => {
            println!("Wrong input, try again...");
            delete_epic_prompt()
        }
    }
}

fn delete_story_prompt() -> bool {
    println!("Are you sure you want to delete this story? [Y/n]: Y");
    match get_user_input().trim() {
        "Y" => true,
        "N" => false,
        _ => {
            println!("Wrong input, try again...");
            delete_epic_prompt()
        }
    }
}

fn update_status_prompt() -> Option<Status> {
    println!("New Status (1 - OPEN, 2 - IN-PROGRESS, 3 - RESOLVED, 4 - CLOSED)");
    let input = get_user_input().parse::<u8>().ok()?;
    match input {
        1 => Some(Status::Open),
        2 => Some(Status::InProgress),
        3 => Some(Status::Resolved),
        4 => Some(Status::Closed),
        _ => {
            println!("Wrong input.");
            None
        }
    }
}
