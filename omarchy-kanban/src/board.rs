use serde::{Deserialize, Serialize};
use ratatui::style::Color;

//simple task with title and tag
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub title: String,
    pub tags: Vec<String>,
}

impl Task {
    //Create task
    pub fn new(title: String) -> Self {
        Self {
            title,
            tags: Vec::new(),
        }
    }

    //add tags to the task
    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    //return color based on tags
    pub fn get_color(&self) -> Color {
        if self.tags.contains(&"urgent".to_string()) {
            Color::Red
        } else if self.tags.contains(&"bug".to_string()) {
            Color::Yellow
        } else if self.tags.contains(&"feature".to_string()) {
            Color::Green
        } else {
            Color::White
        }
    }
}

// kanban board with three coloms: todo, in_progress, done
#[derive(Serialize, Deserialize, Debug)]
pub struct Board {
    pub todo: Vec<Task>,
    pub in_progress: Vec<Task>,
    pub done: Vec<Task>,
}

impl Board {
    //Create new empty board
    pub fn new() -> Self {
        Self {
            todo: Vec::new(),
            in_progress: Vec::new(),
            done: Vec::new(),
        }
    }

    // get column based on index
    pub fn get_column_mut(&mut self, column: Column) -> &mut Vec<Task> {
        match column {
            Column::Todo => &mut self.todo,
            Column::InProgress => &mut self.in_progress,
            Column::Done => &mut self.done,
        }
    }

    //get column ((Rread only))
    pub fn get_column(&self, column: Column) -> &Vec<Task> {
        match column {
            Column::Todo => &self.todo,
            Column::InProgress => &self.in_progress,
            Column::Done => &self.done,
        }
    }
}


    // enum to indicate which column we're working with
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Column {
    Todo,
    InProgress,
    Done,
}


impl Column {
    // move to next column (right)
    pub fn next(self) -> Option<Self> {
        match self {
            Column::Todo => Some(Column::InProgress),
            Column::InProgress => Some(Column::Done),
            Column::Done => None,
        }
    }

    // move to previous column (left)
    pub fn prev(self) -> Option<Self> {
        match self {
            Column::Todo => None,
            Column::InProgress => Some(Column::Todo),
            Column::Done => Some(Column::InProgress),
        }
    }

    //return column name
    pub fn name(self) -> &str {
        match self {
            Column::Todo => "To Do",
            Column::InProgress => "In Progress",
            Column::Done => "Done",
        }
    }
}