use chrono::{DateTime, Utc};

struct Client {
    name: String,
    last_action_time: DateTime<Utc>,
}


#[cfg(feature = "test-client")]
fn get_client_name(client: &mut Client) -> &String {
    client.last_action_time = Utc::now();
    &client.name
}

#[cfg(not(feature = "test-client"))]
fn get_client_name(client: &Client) -> &String {
    &client.name
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "test-client")]
    fn test_get_client_name_feature_enabled() {
        let mut client = Client {
            name: "John".to_string(),
            last_action_time: Utc::now(),
        };
        let last_action_time = client.last_action_time;
        assert_eq!(get_client_name(&mut client), "John");
        assert_ne!(last_action_time, client.last_action_time);
    }

    #[test]
    #[cfg(not(feature = "test-client"))]
    fn test_get_client_name_feature_not_enabled() {
        let client = Client {
            name: "John".to_string(),
            last_action_time: Utc::now(),
        };
        let last_action_time = client.last_action_time;
        assert_eq!(get_client_name(&client), "John");
        assert_eq!(last_action_time, client.last_action_time);
    }
}


