// This file is generated by atrium-codegen. DO NOT EDIT.
#![doc = "Definitions for the `app.bsky.richtext.facet` namespace."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Main {
    pub features: Vec<MainFeaturesItem>,
    pub index: ByteSlice,
}
#[doc = "A text segment. Start is inclusive, end is exclusive. Indices are for utf8-encoded strings."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ByteSlice {
    pub byte_end: i32,
    pub byte_start: i32,
}
#[doc = "A facet feature for links."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub uri: String,
}
#[doc = "A facet feature for actor mentions."]
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Mention {
    pub did: String,
}
#[derive(serde :: Serialize, serde :: Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum MainFeaturesItem {
    #[serde(rename = "app.bsky.richtext.facet#mention")]
    Mention(Box<Mention>),
    #[serde(rename = "app.bsky.richtext.facet#link")]
    Link(Box<Link>),
    #[serde(rename = "app.pollblue.poll.facet#option")]
    Option(Box<Link>),
}
