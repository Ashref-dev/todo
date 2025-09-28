use crate::task::{load_tasks, save_tasks, Task, Priority};
use chrono::prelude::*;
use chrono_english::{parse_date_string, Dialect};
use ratatui::widgets::ListState;
use regex::Regex;

pub enum AppMode {
    Normal,
    Insert,
    DateInput,
    Search,
}

pub struct App {
    pub tasks: Vec<Task>,
    pub state: ListState,
    pub mode: AppMode,
    pub input: String,
    pub date_input: String,
    pub search_input: String,
    pub margin: u16,
    pub adding_subtask: bool,
}

impl App {
    pub fn new() -> App {
        let mut state = ListState::default();
        if !load_tasks("tasks.json")
            .unwrap_or_else(|_| Vec::new())
            .is_empty()
        {
            state.select(Some(0));
        }
        App {
            tasks: load_tasks("tasks.json").unwrap_or_else(|_| Vec::new()),
            state,
            mode: AppMode::Normal,
            input: String::new(),
            date_input: String::new(),
            search_input: String::new(),
            margin: 1,
            adding_subtask: false,
        }
    }

    pub fn zoom_in(&mut self) {
        self.margin = self.margin.saturating_sub(1);
    }

    pub fn zoom_out(&mut self) {
        self.margin = self.margin.saturating_add(1);
    }


    pub fn next(&mut self) {
        let displayed_tasks = self.get_displayed_tasks();
        if displayed_tasks.is_empty() {
            return;
        }
        let i = self.state.selected().map_or(0, |i| (i + 1) % displayed_tasks.len());
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let displayed_tasks = self.get_displayed_tasks();
        if displayed_tasks.is_empty() {
            return;
        }
        let i = self.state.selected().map_or(0, |i| (i + displayed_tasks.len() - 1) % displayed_tasks.len());
        self.state.select(Some(i));
    }

    pub fn toggle_completed(&mut self) {
        if let Some(selected_index) = self.state.selected() {
            let displayed_tasks = self.get_displayed_tasks();
            if let Some(selected_task) = displayed_tasks.get(selected_index) {
                // Find the task in the main tasks vector by ID
                if let Some(main_task) = self.tasks.iter_mut().find(|t| t.id == selected_task.id) {
                    main_task.completed = !main_task.completed;
                }
            }
        }
    }

    pub fn cycle_priority(&mut self) {
        if let Some(selected_index) = self.state.selected() {
            let displayed_tasks = self.get_displayed_tasks();
            if let Some(selected_task) = displayed_tasks.get(selected_index) {
                // Find the task in the main tasks vector by ID
                if let Some(main_task) = self.tasks.iter_mut().find(|t| t.id == selected_task.id) {
                    main_task.priority = match main_task.priority {
                        Priority::Low => Priority::Medium,
                        Priority::Medium => Priority::High,
                        Priority::High => Priority::Low,
                    };
                }
            }
        }
    }

    pub fn save(&self) {
        save_tasks("tasks.json", &self.tasks).unwrap_or_else(|_| {});
    }

    fn extract_date_and_clean_description(&self, input: &str) -> (String, Option<String>) {
        let now = Local::now();
        let mut cleaned_description = input.to_string();
        
        // First try chrono-english for full natural language parsing
        if let Ok(parsed_date) = parse_date_string(input, now, Dialect::Us) {
            // If chrono-english parsed it successfully, trust its result
            let due_date = if parsed_date.time().hour() != 0 || parsed_date.time().minute() != 0 {
                parsed_date.format("%Y-%m-%d %H:%M").to_string()
            } else {
                parsed_date.format("%Y-%m-%d").to_string()
            };
            return (input.to_string(), Some(due_date));
        }

        // If chrono-english fails, use our smart context-aware parsing
        let time_result = self.parse_time_with_context(input, now);
        
        if let Some((parsed_datetime, matched_text)) = time_result {
            // Remove the matched time/date text from description
            cleaned_description = input.replace(&matched_text, "")
                .trim()
                .replace("  ", " ") // Clean up double spaces
                .to_string();
            
            let due_date = if parsed_datetime.time().hour() != 0 || parsed_datetime.time().minute() != 0 {
                parsed_datetime.format("%Y-%m-%d %H:%M").to_string()
            } else {
                parsed_datetime.format("%Y-%m-%d").to_string()
            };
            
            (cleaned_description, Some(due_date))
        } else {
            // No time/date found, return as-is
            (input.to_string(), None)
        }
    }

    fn parse_time_with_context(&self, input: &str, now: DateTime<Local>) -> Option<(DateTime<Local>, String)> {
        let input_lower = input.to_lowercase();
        
        // Explicit date keywords
        if input_lower.contains("today") {
            if let Some((time, matched)) = self.extract_time_from_text(&input_lower) {
                return Some((now.date_naive().and_time(time).and_local_timezone(Local).unwrap(), matched));
            }
            return Some((now.date_naive().and_hms_opt(23, 59, 0).unwrap().and_local_timezone(Local).unwrap(), "today".to_string()));
        }
        
        if input_lower.contains("tomorrow") {
            let tomorrow = now + chrono::Duration::days(1);
            if let Some((time, matched)) = self.extract_time_from_text(&input_lower) {
                return Some((tomorrow.date_naive().and_time(time).and_local_timezone(Local).unwrap(), matched));
            }
            return Some((tomorrow.date_naive().and_hms_opt(9, 0, 0).unwrap().and_local_timezone(Local).unwrap(), "tomorrow".to_string()));
        }

        // Smart time parsing with context awareness
        if let Some((parsed_time, matched_text)) = self.extract_time_from_text(input) {
            let target_date = if parsed_time <= now.time() {
                // If the time has passed today, schedule for tomorrow
                now + chrono::Duration::days(1)
            } else {
                // Time hasn't passed today, schedule for today
                now
            };
            
            let target_datetime = target_date.date_naive()
                .and_time(parsed_time)
                .and_local_timezone(Local)
                .unwrap();
                
            return Some((target_datetime, matched_text));
        }

        // Day of week parsing
        let weekdays = ["monday", "tuesday", "wednesday", "thursday", "friday", "saturday", "sunday"];
        for (i, day) in weekdays.iter().enumerate() {
            if input_lower.contains(day) {
                let target_date = self.get_next_weekday(now, i);
                let time = if let Some((time, _)) = self.extract_time_from_text(input) {
                    time
                } else {
                    chrono::NaiveTime::from_hms_opt(9, 0, 0).unwrap() // Default to 9 AM
                };
                
                return Some((target_date.and_time(time).and_local_timezone(Local).unwrap(), day.to_string()));
            }
        }

        None
    }

    fn extract_time_from_text(&self, text: &str) -> Option<(chrono::NaiveTime, String)> {
        // Pattern for times like "10 PM", "10PM", "10:30 PM", "22:30"
        let time_patterns = [
            // 12-hour format with AM/PM
            (r"\b(\d{1,2}):(\d{2})\s*(AM|PM|am|pm)\b", true),
            (r"\b(\d{1,2})\s*(AM|PM|am|pm)\b", false),
            (r"at\s+(\d{1,2}):(\d{2})\s*(AM|PM|am|pm)\b", true),
            (r"at\s+(\d{1,2})\s*(AM|PM|am|pm)\b", false),
            // 24-hour format
            (r"\b(\d{1,2}):(\d{2})\b", true),
            (r"\b(\d{1,2})h\b", false),
        ];

        for (pattern, has_minutes) in time_patterns {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(captures) = re.captures(text) {
                    let matched_text = captures.get(0).unwrap().as_str().to_string();
                    
                    let hour: u32 = captures.get(1).unwrap().as_str().parse().ok()?;
                    let minute: u32 = if has_minutes {
                        captures.get(2).unwrap().as_str().parse().ok()?
                    } else {
                        0
                    };
                    
                    let is_pm = if has_minutes {
                        captures.get(3).map_or(false, |m| m.as_str().to_lowercase().contains('p'))
                    } else {
                        captures.get(2).map_or(false, |m| m.as_str().to_lowercase().contains('p'))
                    };
                    
                    // Convert to 24-hour format
                    let hour_24 = if is_pm && hour != 12 {
                        hour + 12
                    } else if !is_pm && hour == 12 {
                        0
                    } else {
                        hour
                    };
                    
                    if let Some(time) = chrono::NaiveTime::from_hms_opt(hour_24, minute, 0) {
                        return Some((time, matched_text));
                    }
                }
            }
        }
        
        None
    }

    fn get_next_weekday(&self, now: DateTime<Local>, target_weekday: usize) -> chrono::NaiveDate {
        let current_weekday = now.weekday().num_days_from_monday() as usize;
        let days_until_target = if target_weekday > current_weekday {
            target_weekday - current_weekday
        } else {
            7 - (current_weekday - target_weekday)
        };
        
        (now + chrono::Duration::days(days_until_target as i64)).date_naive()
    }

    pub fn add_task(&mut self) {
        if self.adding_subtask {
            self.add_sub_task();
            self.adding_subtask = false;
        } else {
            let new_id = self.tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
            let (cleaned_description, due_date) = self.extract_date_and_clean_description(&self.input);
            let tags = self
                .input
                .split_whitespace()
                .filter(|word| word.starts_with('#'))
                .map(|word| word.to_string())
                .collect();

            let new_task = Task {
                id: new_id,
                description: if cleaned_description.trim().is_empty() {
                    self.input.clone()
                } else {
                    cleaned_description
                },
                completed: false,
                priority: Priority::Medium,
                due_date,
                sub_tasks: Box::new(Vec::new()),
                tags,
            };
            self.tasks.push(new_task);
        }
        self.input.clear();
        self.mode = AppMode::Normal;
    }

    pub fn add_sub_task(&mut self) {
        if let Some(selected_index) = self.state.selected() {
            let displayed_tasks = self.get_displayed_tasks();
            if let Some(selected_task) = displayed_tasks.get(selected_index) {
                let selected_task_id = selected_task.id;
                
                // Extract data before getting mutable reference
                let (cleaned_description, due_date) = self.extract_date_and_clean_description(&self.input);
                let tags = self
                    .input
                    .split_whitespace()
                    .filter(|word| word.starts_with('#'))
                    .map(|word| word.to_string())
                    .collect();

                // Find the task in the main tasks vector by ID
                if let Some(main_task) = self.tasks.iter_mut().find(|t| t.id == selected_task_id) {
                    let new_id = main_task.sub_tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;

                    let new_task = Task {
                        id: new_id,
                        description: if cleaned_description.trim().is_empty() {
                            self.input.clone()
                        } else {
                            cleaned_description
                        },
                        completed: false,
                        priority: Priority::Medium,
                        due_date,
                        sub_tasks: Box::new(Vec::new()),
                        tags,
                    };
                    main_task.sub_tasks.push(new_task);
                }
            }
        }
        self.input.clear();
        self.mode = AppMode::Normal;
    }

    pub fn set_due_date(&mut self) {
        if let Some(selected_index) = self.state.selected() {
            let displayed_tasks = self.get_displayed_tasks();
            if let Some(selected_task) = displayed_tasks.get(selected_index) {
                // Find the task in the main tasks vector by ID
                if let Some(main_task) = self.tasks.iter_mut().find(|t| t.id == selected_task.id) {
                    main_task.due_date = Some(self.date_input.drain(..).collect());
                }
            }
        }
        self.mode = AppMode::Normal;
    }

    pub fn delete_task(&mut self) {
        if let Some(selected_index) = self.state.selected() {
            let displayed_tasks = self.get_displayed_tasks();
            if let Some(selected_task) = displayed_tasks.get(selected_index) {
                // Find and remove the task in the main tasks vector by ID
                if let Some(main_index) = self.tasks.iter().position(|t| t.id == selected_task.id) {
                    self.tasks.remove(main_index);
                }
                
                // Update selection
                let new_displayed_tasks = self.get_displayed_tasks();
                if !new_displayed_tasks.is_empty() {
                    self.state.select(Some(selected_index.min(new_displayed_tasks.len() - 1)));
                } else {
                    self.state.select(None);
                }
            }
        }
    }

    pub fn filter_tasks(&self) -> Vec<Task> {
        if self.search_input.is_empty() {
            return self.tasks.clone();
        }

        let search_lower = self.search_input.to_lowercase();
        self.tasks
            .iter()
            .filter(|task| {
                // Filter by description (case-insensitive)
                task.description.to_lowercase().contains(&search_lower)
                    // Filter by tags (case-insensitive)
                    || task.tags.iter().any(|tag| tag.to_lowercase().contains(&search_lower))
                    // Filter by priority
                    || match search_lower.as_str() {
                        "high" | "h" => matches!(task.priority, Priority::High),
                        "medium" | "med" | "m" => matches!(task.priority, Priority::Medium),
                        "low" | "l" => matches!(task.priority, Priority::Low),
                        _ => false,
                    }
                    // Filter by completion status
                    || match search_lower.as_str() {
                        "completed" | "done" | "finished" => task.completed,
                        "incomplete" | "pending" | "todo" => !task.completed,
                        _ => false,
                    }
                    // Filter by due date (if it exists)
                    || task.due_date.as_ref().map_or(false, |date| date.contains(&search_lower))
                    // Filter by subtasks content
                    || task.sub_tasks.iter().any(|subtask| {
                        subtask.description.to_lowercase().contains(&search_lower)
                            || subtask.tags.iter().any(|tag| tag.to_lowercase().contains(&search_lower))
                    })
            })
            .cloned()
            .collect()
    }

    pub fn get_displayed_tasks(&self) -> Vec<Task> {
        match self.mode {
            AppMode::Search if !self.search_input.is_empty() => self.filter_tasks(),
            _ => self.tasks.clone(),
        }
    }
}
