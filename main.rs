


//our todo must be able to add_item, delete_item, show completed task, .....

//defining the todo item struct
 struct TodoItem {
    id: u32, //this represents the unique identifier for every unique item
    name: String, //this contains the contains/name of the todo items
    completed: bool, // this indicates whether the todo item is complted or not 
 }

 // function to update the todo item
 fn completed_item(item: &mut TodoItem) {
    item.completed = true;
 }

 //next we need to display our todo items
 fn display_items(items:&Vec<TodoItem>) { //we need to make a vector of TodoItems
           for item in items {
            println!("{} - {} ({})", item.id, item.name, item.completed);
           }
 }

 //next, we write function to accept user input
 use std::io;

 fn main(){
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop{
        println!("What is your plan for today");
        println!("1. Add a todo item");
        println!("2. Complete a to-do item");
        println!("3. Display to-do items");
        println!("4. Quit");

        //let there be a choice from the user, and let it be mutable
        let mut choice = String::new(); //let it be a new string, changeable
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        
        let choice = choice.trim().parse::<u32>().expect("Invalid Input");

        match choice {
            //match chioces, if 1, if 2, if 3, ......if 4 do something
            1 => {
                println!("Enter the name of the todo item:");
                let mut name = String::new();
                //then we read from user
                io::stdin().read_line(&mut name).expect("Failed to read line");
                let name = name.trim().to_string();
                //the id of the name 
                let id = todo_list.len() as u32 + 1;

                let item = TodoItem {
                    id,
                    name,
                    completed: false
                };
                todo_list.push(item);
            },

            2 => { 
                println!("Enter the ID of the To-do item you want to complete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id = id.trim().parse::<u32>().expect("Invalid Input");

                let item = todo_list.iter_mut().find(|i| i.id == id).unwrap();
                completed_item(item);
            },

            3 => {
                display_items(&todo_list);
            },

            4 => {
                println!("Goodbye!");
                return;
            },
            _=> {
                println!("Invalid input");
            },
        }
        
    }
 }

