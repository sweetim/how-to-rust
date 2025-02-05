#[cfg(test)]
mod tests {
    use std::fmt;

    #[derive(Debug)]
    struct DefaultDisplay {
        x: u32,
        y: u32,
    }

    #[test]
    fn it_shows_different_format_specifier_output_using_default_display_and_debug_trait() {
        let item = DefaultDisplay {
            x: 1,
            y: 2,
        };

        let display_trait = format!("{} {}", item.x, item.y);
        let debug_trait = format!("{:?}", item);
        let pretty_display_trait = format!("{:#?}", item);

        assert_eq!(display_trait, "1 2");
        assert_eq!(debug_trait, "DefaultDisplay { x: 1, y: 2 }");
        assert_eq!(pretty_display_trait, "DefaultDisplay {\n    x: 1,\n    y: 2,\n}");
    }

    struct CustomDisplay {
        x: u32,
        y: u32,
    }

    impl fmt::Display for CustomDisplay {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl fmt::Debug for CustomDisplay {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if f.alternate() {
                write!(f, "Pretty (x = {}, y = {})", self.x, self.y)
            } else {
                write!(f, "(x = {}, y = {})", self.x, self.y)
            }
        }
    }

    #[test]
    fn it_shows_different_format_specifier_output_using_custom_display_and_debug_trait() {
        let item = CustomDisplay {
            x: 1,
            y: 2,
        };

        let display_trait = format!("{}", item);
        let debug_trait = format!("{:?}", item);
        let pretty_display_trait = format!("{:#?}", item);

        assert_eq!(display_trait, "(1, 2)");
        assert_eq!(debug_trait, "(x = 1, y = 2)");
        assert_eq!(pretty_display_trait, "Pretty (x = 1, y = 2)");
    }
}
