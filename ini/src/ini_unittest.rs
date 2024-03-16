use chrono::Local;
use std::collections::HashMap;
extern crate ini;
use ini::Ini;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_ini() {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string();
        println!("Current timestamp: {}", timestamp);

        let mut map = HashMap::new();
        map.insert("username".to_string(), "admin".to_string());
        map.insert("password".to_string(), "123456".to_string());
        map.insert("schema".to_string(), "http".to_string());
        map.insert("host".to_string(), "localhost".to_string());
        map.insert("port".to_string(), "8080".to_string());
        map.insert("path".to_string(), "/".to_string());
        map.insert("timestamp".to_string(), timestamp);

        let curren_dir = std::env::current_dir().unwrap();
        let ini_file_path = curren_dir.join("test.ini");
        let mut write_conf = if ini_file_path.exists() {
            Ini::load_from_file(ini_file_path.to_str().unwrap()).unwrap()
        } else {
            Ini::new()
        };
        for (k, v) in &map {
            write_conf.with_section(None::<String>).set(k, v);
        }
        write_conf
            .write_to_file(ini_file_path.to_str().unwrap())
            .unwrap();

        let read_conf = Ini::load_from_file(ini_file_path.to_str().unwrap()).unwrap();
        let read_section = read_conf.section(None::<String>).unwrap();
        for (k, v) in &map {
            assert_eq!(read_section.get(k).unwrap(), v);
        }
    }
}
