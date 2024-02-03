pub mod todo{

    use std::process::exit;

    use rusqlite::Connection;

    pub fn list()->Result<Vec<Todo>, String>{
        let connection = connect_db();

        match connection {
            Ok(con) => {
                let result = con.prepare("SELECT * from todo");
                match result {
                    Ok(mut stmt) => {
                        let result =  stmt.query_map([], |f|{
                            Ok(Todo{
                                id: f.get(0)?,
                                content: f.get(1)?,
                                marked: f.get(2)?                                       
                            })
                        });
                        if let Ok(todos) = result {
                            let todo_list: Vec<Todo> = todos
                                .filter(|f| f.is_ok())
                                .map(|f|f.unwrap()).collect();

                            Ok(todo_list)
                                                      
                        }else{
                            Err("Error creating Todo objects".to_string())
                        }
                    },
                    Err(e)=>{
                        Err(format!("Error getting todo from database table.\n{}",e))
                    }
                }
            },
            Err(e)=>{
                Err(e)
            }
        }
        
    }
    pub fn connect_db() -> Result<Connection, String>{
        let app_folder = simple_home_dir::home_dir().unwrap().join(".todo_rs");
        let db_location = app_folder.join("todo.db");

        if !app_folder.exists(){
            if let Err(e) = std::fs::create_dir(app_folder){
                return Err(format!("There was an error when trying to create the database directory\n{}",e.to_string()));
            }
        }
        if !db_location.exists(){
            if let Err(e) =  std::fs::File::create(&db_location){
                return Err(format!("There was an error when trying to create the database file\n{}",e.to_string()));  
            }
        }
        
        match rusqlite::Connection::open(db_location) {
            Ok(con)=>{
                match con.execute("CREATE TABLE IF NOT EXISTS todo(id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT, marked BOOL)", []) {
                    Ok(_) => Ok(con),
                    Err(e) => Err(format!("Error creating table.\n{}",e))
                }                        
            }
            Err(e) => Err(format!("Error opening db.\n{}",e))
        }
    }

    pub fn execute_command(command: TodoCommand) -> Result<(), String>{
        let connection = connect_db();

        match connection {
            Ok(con)=>{
                match command {
                    TodoCommand::Add(todo)=>{
                        let result = con.execute(format!("INSERT INTO todo (content, marked) values ('{}', false)", todo).as_str(), []);
                        if let Err(e) = result{
                            return Err(format!("Error adding new todo.\n{}",e));
                        }
                    },
                    TodoCommand::Remove(id)=>{
                        let result = con.execute(format!("DELETE from todo WHERE id = {}", id).as_str(), []);
    
                        if let Err(e) = result{
                            return Err(format!("Error removing {} todo.\n{}",id,e));
                        }
                    },
                    TodoCommand::Mark(id)=>{
                        let result = con.execute(format!("UPDATE todo SET marked=true WHERE id = {}", id).as_str(), []);
                        if let Err(e) = result{
                            return Err(format!("Error marking {} as done.\n{}",id,e));
                        }

                    },
                    TodoCommand::Unmark(id)=>{
                        let result = con.execute(format!("UPDATE todo SET marked=false WHERE id = {}",id).as_str(), []);
                        if let Err(e) = result{
                            return Err(format!("Error unmarking {}.\n{}",id,e));
                        }
                    },
                    TodoCommand::Toggle((id,marked))=>{

                        let marked_value = if marked{"false"}else{"true"};

                        let result = con.execute(format!("UPDATE todo SET marked={} WHERE id = {}",marked_value,id).as_str(), []);
                        if let Err(e) = result{
                            return Err(format!("Error unmarking {}.\n{}",id,e));
                        }

                    },
                } 
            },
            Err(e)=>{
                return Err(e);
            }
        }

        Ok(())


    }

    pub enum TodoCommand<'a>{
        Add(&'a str),
        Remove(usize),
        Mark(usize),
        Unmark(usize),
        Toggle((usize, bool))
    }

    pub struct Todo{
        pub id: usize,
        pub content: String,
        pub marked: bool
    }

    impl std::fmt::Display for Todo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.marked{
                write!(f, "{} [x]", self.content)
            }else{
                write!(f, "{} []", self.content)
            }
        }
    }

}