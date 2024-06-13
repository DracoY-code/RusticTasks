use rustic_tasks::*;

fn main() -> Result<(), RusticError> {
    let exit_code = App::build();
    println!("{:?}", exit_code);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    
    use regex::Regex;
    
    use app::todo_list::author::utils::{
        is_valid_email,
        verify_password,
    };
    use app::todo_list::task::utils::parse_due_date;

    use super::*;

    #[test]
    fn print_dummy_todo_list_and_write_to_csv() -> Result<(), RusticError> {
        let author = Author::build(
            "John",
            "john@example.com",
            "password123",
        )?;

        let file_path = Path::new("./data/author_test.csv");
        if let Err(e) = author.write_to_csv(file_path) {
            eprintln!("{e}");
        } else {
            println!("Written author to: {}", file_path.display());
        }

        let mut dummy_list = ToDoList::build(author)?;

        let dummy_task_1 = Task::new(
            "Practice the piano",
            None,
            None,
            3,
            vec![ "Music", "Piano" ],
            dummy_list.get_author(),
        )?;

        let dummy_task_2 = Task::new(
            "Write a paper",
            None,
            Some("2024-06-10T16:00:00+05:30"),
            1,
            vec![ "Thesis" ],
            dummy_list.get_author(),
        )?;

        dummy_list.add(dummy_task_1);
        dummy_list.add(dummy_task_2);
        
        println!("{:#?}", dummy_list);
        
        let file_path = Path::new("./data/tasks_test.csv");
        if let Err(e) = dummy_list.write_to_csv(file_path) {
            eprintln!("{e}");
        } else {
            println!("Written tasks to: {}", file_path.display());
        };

        Ok(())
    }
    
    #[test]
    fn check_email_validity() -> Result<(), RusticError> {
        let invalid_email = "john@example@com";

        let flag = is_valid_email(&invalid_email)?;
        println!("Is the email valid?: {flag}");

        Ok(())
    }

    #[test]
    fn cause_rustic_regex_error() -> Result<(), RusticError> {
        fn is_valid_pattern(pattern: &str) -> Result<(), RusticError> {
            Regex::new(pattern)?;
            Ok(())
        }
        
        if let Err(e) = is_valid_pattern(r"{\d+") {
            eprintln!("{e}");
        }
        
        Ok(())
    }

    #[test]
    fn parse_date_string() {
        assert!(parse_due_date("2024-06-10T16:00:00+05:30").is_ok());
    }
    
    #[test]
    fn validate_author() -> Result<(), RusticError> {
        let dummy_author = Author::build(
            "John",
            "john@example.com",
            "password123",
        )?;

        verify_password(
            "password123", 
            dummy_author.get_password(),
        )?;

        Ok(())
    }

    #[test]
    fn update_task_details() -> Result<(), RusticError> {
        let dummy_author = Author::build(
            "John",
            "john@example.com",
            "password123",
        )?;

        let mut dummy_task = Task::new(
            "Prepare a presentation",
            None,
            None,
            3,
            vec![ "Work" ],
            &dummy_author,
        )?;
        
        dummy_task.update_description(
            "The presentation should be pretty!"
        );

        dummy_task.change_priority(1);

        dummy_task.update_status(Status::InProgress);

        dummy_task.update_labels(vec![ "Presentation", "Work" ]);

        dummy_task.mark_completed();

        println!("{:#?}", dummy_task);

        Ok(())
    }

    #[test]
    fn build_gtk_app() {
        let exit_code = App::build();
        println!("{:?}", exit_code);
    }
}
