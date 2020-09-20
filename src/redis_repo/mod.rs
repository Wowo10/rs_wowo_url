extern crate redis;

use std::collections::HashMap;

pub struct RedisRepo{
    client: redis::Client,
}

impl RedisRepo{
    pub fn create(address: &str) -> Self{
        RedisRepo{
            client: redis::Client::open(address).unwrap()
        }
    }

    pub fn save(&self, key: &str, value: &str) -> redis::RedisResult<()> {
        let mut connection = self.client.get_connection().unwrap();

        let _ : () = redis::cmd("SET").arg(key).arg(value).query(&mut connection)?;
        Ok(())
    }
    
    pub fn read(&self, key: &str) -> redis::RedisResult<String> {
        let mut connection = self.client.get_connection().unwrap();

        let value: String = redis::cmd("GET").arg(key).query(&mut connection)?;
        Ok(value)
    }

    fn list_keys(&self) -> redis::RedisResult<Vec<String>>{
        let mut connection = self.client.get_connection().unwrap();

        let value: Vec<String> = redis::cmd("KEYS").arg("*").query(&mut connection)?;
        Ok(value)
    }

    pub fn list_all(&self) -> HashMap<String, String>{
        let keys = self.list_keys().unwrap();

        let mut map = HashMap::new();

        for key in keys{
            let value = self.read(&key).unwrap();

            map.insert(key, value);
        }

        map
    }
}