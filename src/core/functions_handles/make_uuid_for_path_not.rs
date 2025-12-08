use uuid::Uuid;

pub fn make_uuid_for_path_not(function_name: &String) -> String{
    let id = Uuid::new_v4();
    let simple = id.simple().to_string();
    format!("{}_{}", function_name, simple)
}