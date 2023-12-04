use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct UserIdentity {
    name: String,
    email: String,
}

#[cfg(test)]
mod tests {
    use super::UserIdentity;

    fn get_samples() -> Vec<UserIdentity> {
        vec![
            UserIdentity {
                name: String::from("Victor Kerr"),
                email: String::from("lorem@protonmail.net"),
            },
            UserIdentity {
                name: String::from("Irene Prince"),
                email: String::from("odio.aliquam@outlook.couk"),
            },
            UserIdentity {
                name: String::from("Cameron Blair"),
                email: String::from("amet.ante.vivamus@protonmail.org"),
            },
            UserIdentity {
                name: String::from("Finn Petersen"),
                email: String::from("dictum.magna@yahoo.com"),
            },
            UserIdentity {
                name: String::from("Miriam Christensen"),
                email: String::from("tincidunt.tempus.risus@outlook.couk"),
            },
            UserIdentity {
                name: String::from("Carlos Wilkinson"),
                email: String::from("tincidunt.adipiscing@protonmail.edu"),
            },
            UserIdentity {
                name: String::from("Yoshi Mosley"),
                email: String::from("fusce.aliquet@outlook.couk"),
            },
            UserIdentity {
                name: String::from("Fay Holden"),
                email: String::from("mauris.ipsum@yahoo.org"),
            },
            UserIdentity {
                name: String::from("Edward Todd"),
                email: String::from("elementum.lorem@aol.edu"),
            },
            UserIdentity {
                name: String::from("Axel Frost"),
                email: String::from("lorem.fringilla@icloud.edu"),
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
}
