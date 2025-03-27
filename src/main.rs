use std::io;
#[derive(Debug)]
enum Status{
    Open,
    Pospond(String),
    Completed
}

struct Task{
    id: u32,
    name: String,
    date: String,
    time: String,
    status: Status
}

impl Task{

    fn update_task(&mut self){
        println!("Enter the new task name:");
        let mut name=String::new();
        io::stdin().read_line(&mut name).expect("Failed to read Message");
        self.name=name;
    }

    fn update_status(&mut self){
        println!("Enter 1 to set the task to complete");
        println!("Enter 0 to set the task to open");
        println!("Enter 2 to set the task to Posponed");
        let mut sop=String::new();
        io::stdin().read_line(&mut sop).expect("Failed to read Message");
        let op:u32=sop.trim().parse().expect("Failed to convert");
        match op{
            0=>self.status=Status::Open,
            1=>self.status=Status::Completed,
            2=>{
                println!("Enter the new date to posponed to");
                let mut date=String::new();
                io::stdin().read_line(&mut date).expect("Failed to read Message");
                self.date=date.clone();
                self.status=Status::Pospond(date);

                println!("Enter the new time to posponed to");
                let mut time=String::new();
                io::stdin().read_line(&mut time).expect("Failed to read Message");
                self.time=time.clone();
            },
            _=>println!("Invalid option")
        };
    }

    fn create_task(count:&mut u32,name:String,date:String,time:String)->Task{
        *count+=1;
        return Task{
            id:*count,
            name,
            date,
            time,
            status: Status::Open
        }
    }

    fn show_task(&self ){
        println!("--------------------------------------");
        println!("id: {}", self.id);
        println!("Name: {}", self.name);
        println!("Date: {}", self.date);
        println!("Time: {}", self.time);
        match &self.status {
            Status::Open=>println!("Status: Open"),
            Status::Completed=>println!("Status: Completed"),
            Status::Pospond(date)=>println!("Status: Posponed to {}",date),
        }
        println!("--------------------------------------");
    }

}



fn add_tasks(tasks:&mut Vec<Task>,count:&mut u32){
    println!("Enter the task name");
    let mut name=String::new();
    io::stdin().read_line(&mut name).expect("Failed to read Message");
    println!("Enter the task date");
    let mut date=String::new();
    io::stdin().read_line(&mut date).expect("Failed to read Message");
    println!("Enter the task time");
    let mut time=String::new();
    io::stdin().read_line(&mut time).expect("Failed to read Message");

    tasks.push(Task::create_task(count, name, date, time));
}

fn remove_tasks(tasks:&mut Vec<Task>,count:&mut u32){
    println!("Enter the id of the task");
    let mut id=String::new();
    io::stdin().read_line(&mut id).expect("Failed to read Message");
    let id:u32=id.trim().parse().expect("Failed to convert");
    let mut index = 0;
    while index < tasks.len() {
        if tasks[index].id == id {
            *count -= 1;
            tasks.remove(index); 
        } else {
            index += 1; // Only increment if not removed
        }
    }
}

fn update_tasks(tasks:&mut Vec<Task>,count:& u32){
    println!("Enter the id of the task");
    let mut id=String::new();
    io::stdin().read_line(&mut id).expect("Failed to read Message");
    let id:u32=id.trim().parse().expect("Failed to convert");
    if id>*count{
        println!("Invalid id. Try again");
    }
    else{
        let mut index = 0;

        while index < tasks.len() {
            if tasks[index].id == id {
                println!("Task found");
                break;
            } else {
                index += 1; 
            }
        }

        println!("Enter 1 for updating the task details and 2 for updating the staus");
        let mut op: String=String::new();
        io::stdin().read_line(&mut op).expect("Failed to read Message");
        let op:u32=op.trim().parse().expect("Failed to convert");

        match op {
            1=>tasks[index].update_task(),
            2=>tasks[index].update_status(),
            _=>println!("Invalid"),
        }
       
    }
    
}

fn see_all_tasks(tasks:& Vec<Task>){
    for i in tasks{
        i.show_task();
    }
}

fn see_pend_tasks(tasks:& Vec<Task>){
    for i in tasks{
        if let Status::Completed = i.status{
            
        }
        else{
            i.show_task();
        }
    }
}

fn see_comp_tasks(tasks:& Vec<Task>){
    for i in tasks{
        if let Status::Completed = i.status{
            i.show_task();
        }
    }
}

fn main(){
    println!("Welcome to the terminal Task manager");
    let mut tasks:Vec<Task>=Vec::new();
    let mut count=0;

    loop{
        println!("Enter the number of the option that you want to use");
        println!("1. Add task");
        println!("2. Remove task");
        println!("3. Update task");
        println!("4. See all tasks");
        println!("5. See pending tasks");
        println!("6. See completed tasks");
        println!("10. Exit");
        
        let mut sop=String::new();
        io::stdin().read_line(&mut sop).expect("Failed to read Message");
        let op:u32=sop.trim().parse().expect("Failed to convert");

        match op{
            1=>add_tasks(&mut tasks,&mut count),
            2=>remove_tasks(&mut tasks,&mut count),
            3=>update_tasks(&mut tasks,&count),
            4=>see_all_tasks(&tasks),
            5=>see_pend_tasks(&tasks),
            6=>see_comp_tasks(&tasks),
            10=>break,
            _=>println!("Invalid"),
        }

    }
}