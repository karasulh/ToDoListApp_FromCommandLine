use todoapp::ToDo;

fn main() {
    
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3{
        panic!("Too less arguments passed!");
    }
    let action = args[1].clone();
    let task= args[2].clone();

    let mut todolist = ToDo::new().expect("Error in creating a TodoApp");
    
    if action == "add" {
        todolist.insert(task);
        match todolist.save(){
            Ok(_) => println!("New task is added to ToDo List"),
            Err(e) => println!("Error: {}",e)
        }
    }
    else if action == "start"{
        match todolist.start(task){
            None => println!("There is no such task in ToDo List"),
            Some(_) => {
                match todolist.save(){
                    Ok(_) => println!("Task is in progress."),
                    Err(e) => println!("Error: {}",e),
                }
            }
        }
    }
    else if action == "done"{
        match todolist.done(task){
            None => println!("There is no such task in ToDo List"),
            Some(_) => {
                match todolist.save(){
                    Ok(_) => println!("Task is done."),
                    Err(e) => println!("Error: {}",e),
                }
            }
        }
    }
    else{
        println!("Wrong actions are given");
    }

}
