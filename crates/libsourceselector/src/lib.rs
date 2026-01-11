#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum Source {
    Monitor {
        monitor_name: String,
    },
    Window {
        window_id: u64,
        window_name: String,
        icon_path: Option<String>,
    },
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Sources(pub Vec<Source>);

impl Sources {
    pub fn iter(&self) -> impl Iterator<Item = &Source> {
        self.0.iter()
    }
}

impl From<Vec<Source>> for Sources {
    fn from(value: Vec<Source>) -> Self {
        Self(value)
    }
}

impl From<Sources> for Vec<Source> {
    fn from(value: Sources) -> Self {
        value.0
    }
}

pub trait SerdeJson<'de>: serde::Serialize + serde::Deserialize<'de> {
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }

    fn from_json(s: &'de str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }
}

impl<'de> SerdeJson<'de> for Source {}
impl<'de> SerdeJson<'de> for Sources {}
