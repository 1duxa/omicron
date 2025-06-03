#[allow(unused)]
pub mod types {
    use serde::Deserialize;

    pub trait HasStyleGroup {
        fn style(&self) -> Option<&String>;
        fn has_style(&self, group: &str) -> bool {
            self.style().map(|s| s == group).unwrap_or(false)
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct Entry {
        pub title: Title,
        pub app: App,
    }

    #[derive(Debug, Deserialize)]
    pub struct Title {
        #[serde(rename = "$text")]
        pub content: String,
    }

    #[derive(Debug, Deserialize)]
    pub struct App {
        #[serde(rename = "@style")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub style: Option<String>,
        pub div: Div,
    }

    #[derive(Debug, Deserialize)]
    pub struct Div {
        #[serde(rename = "@style")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub style: Option<String>,
        pub h1: Vec<H1>,
    }

    #[derive(Debug, Deserialize)]
    pub struct H1 {
        #[serde(rename = "@style")]
        #[serde(skip_serializing_if = "Option::is_none")]
        pub style: Option<String>,
        #[serde(rename = "$text")]
        pub content: String,
    }
    impl HasStyleGroup for App {
        fn style(&self) -> Option<&String> {
            self.style.as_ref()
        }
    }

    impl HasStyleGroup for Div {
        fn style(&self) -> Option<&String> {
            self.style.as_ref()
        }
    }

    impl HasStyleGroup for H1 {
        fn style(&self) -> Option<&String> {
            self.style.as_ref()
        }
    }
}
