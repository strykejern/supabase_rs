use uuid::Uuid;

pub mod update;
pub mod search;


pub struct SupabaseClient {
    pub url: String,
    pub api_key: String
}


impl SupabaseClient {
    // service role and anon key will be cooked here
    pub fn new(
        supabase_url: String,
        private_key: String
    ) -> Self {
        Self {
            url: supabase_url,
            api_key: private_key
        }
    }
}

/// Generates a random UUID to be passed as `id` when desired
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
}