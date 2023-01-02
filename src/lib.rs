use std::collections::HashMap;
use std::io::Read;

enum TaskState {
    ToDo,
    InProgress,
    Done
}

impl TaskState{
    fn enum_to_string(self) -> String{
        match self{
            TaskState::ToDo => String::from("To Do"),
            TaskState::InProgress => String::from("In Progress"),
            TaskState::Done => String::from("Done") 
        }
    }
}

pub struct ToDo{
    list_map: HashMap<String,String>
}

impl ToDo{
    pub fn new() -> Result<ToDo,std::io::Error>{

        let mut f = std::fs::OpenOptions::new().write(true).create(true).read(true).open("todo.db")?;//with '?', return Err if problem occurs. 
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let list_map:HashMap<String, String> = content.lines()
        .map(|line| {line.split(" : ").collect::<Vec<&str>>()})
        .map(|v| { (String::from(v[0]), String::from(v[1])) } )
        .map(|(k,v)| { (String::from(k), String::from(v))}).collect();
        Ok(ToDo{list_map})
    }

    pub fn insert(&mut self, new_task:String){
        self.list_map.insert(new_task, TaskState::ToDo.enum_to_string());
    }

    pub fn start(&mut self, task:String) -> Option<()>{
        if let Some(task_state_map) = self.list_map.get_mut(&task){
            *task_state_map = TaskState::InProgress.enum_to_string();
            Some(())
        }else{
            None
        }
    }

    pub fn done(&mut self, task:String) -> Option<()>{
        match self.list_map.get_mut(&task){
            Some(task_state_map) => { 
                *task_state_map = TaskState::Done.enum_to_string();
                Some(())
            }
            None => None
        }
    }

    pub fn save(self) -> Result<(),std::io::Error>{
        let mut contents = String::new();
        for (k,v) in self.list_map{
            let each_task_state = format!("{} : {}\n",k,v);
            contents.push_str(&each_task_state);
        }
        std::fs::write("todo.db", contents)
    }
}