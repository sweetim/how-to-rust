use serde::{Deserialize, Serialize};

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserIdentity {
    name: String,
    email: String,
}

#[cfg(test)]
mod tests {
    use fake::Fake;
    use rand::SeedableRng;

    use super::UserIdentity;

    fn get_samples() -> Vec<UserIdentity> {
        vec![
            UserIdentity {
                name: String::from("Rosalee Dare"),
                email: String::from("eli@example.com"),
            },
            UserIdentity {
                name: String::from("Cecilia Konopelski"),
                email: String::from("caesar@example.net"),
            },
            UserIdentity {
                name: String::from("Merritt Rohan"),
                email: String::from("monty@example.net"),
            },
            UserIdentity {
                name: String::from("Lily Heathcote"),
                email: String::from("yolanda@example.org"),
            },
            UserIdentity {
                name: String::from("Nestor McCullough"),
                email: String::from("jade@example.com"),
            },
            UserIdentity {
                name: String::from("Libbie Hahn"),
                email: String::from("ron@example.com"),
            },
            UserIdentity {
                name: String::from("Devyn Kuhlman"),
                email: String::from("julie@example.org"),
            },
            UserIdentity {
                name: String::from("Madonna Doyle"),
                email: String::from("alanna@example.com"),
            },
            UserIdentity {
                name: String::from("Frederik Goyette"),
                email: String::from("candida@example.com"),
            },
            UserIdentity {
                name: String::from("Noemie Crist"),
                email: String::from("corine@example.com"),
            },
        ]
    }

    #[test]
    fn hardcode_large_test_sample_in_code() {
        assert_eq!(get_samples().len(), 10);
    }

    fn get_test_sample_from_file_at_runtime() -> Vec<UserIdentity> {
        let file_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("resources")
            .join("sample_data.json");

        let content = std::fs::read_to_string(file_path).unwrap();
        serde_json::from_str(&content).unwrap()
    }

    #[test]
    fn store_test_sample_in_file_and_import_when_needed_at_runtime() {
        assert_eq!(get_test_sample_from_file_at_runtime(), get_samples());
    }

    fn get_test_sample_from_file_at_compile_time() -> Vec<UserIdentity> {
        let content = include_str!("../../resources/sample_data.json");
        serde_json::from_str(&content).unwrap()
    }

    #[test]
    fn store_test_sample_in_file_and_import_when_needed_at_compile_time() {
        assert_eq!(get_test_sample_from_file_at_compile_time(), get_samples());
    }

    fn generate_fake_sample_data() -> Vec<UserIdentity> {
        let seed: [u8; 32] = [1; 32];
        let ref mut rng = rand::rngs::StdRng::from_seed(seed);

        (0..10)
            .into_iter()
            .map(|_| UserIdentity {
                name: fake::faker::name::en::Name().fake_with_rng(rng),
                email: fake::faker::internet::en::SafeEmail().fake_with_rng(rng),
            })
            .collect()
    }

    #[test]
    fn generate_fake_test_sample() {
        assert_eq!(generate_fake_sample_data(), get_samples());
    }
}
