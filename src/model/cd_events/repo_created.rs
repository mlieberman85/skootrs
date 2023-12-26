#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryCreatedEvent {
    pub context: RepositoryCreatedEventContext,
    #[serde(
        rename = "customData",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_data: Option<RepositoryCreatedEventCustomData>,
    #[serde(
        rename = "customDataContentType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_data_content_type: Option<String>,
    pub subject: RepositoryCreatedEventSubject,
}
impl From<&RepositoryCreatedEvent> for RepositoryCreatedEvent {
    fn from(value: &RepositoryCreatedEvent) -> Self {
        value.clone()
    }
}
impl RepositoryCreatedEvent {
    pub fn builder() -> builder::RepositoryCreatedEvent {
        builder::RepositoryCreatedEvent::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryCreatedEventContext {
    pub id: RepositoryCreatedEventContextId,
    pub source: String,
    pub timestamp: chrono::DateTime<chrono::offset::Utc>,
    #[serde(rename = "type")]
    pub type_: RepositoryCreatedEventContextType,
    pub version: RepositoryCreatedEventContextVersion,
}
impl From<&RepositoryCreatedEventContext> for RepositoryCreatedEventContext {
    fn from(value: &RepositoryCreatedEventContext) -> Self {
        value.clone()
    }
}
impl RepositoryCreatedEventContext {
    pub fn builder() -> builder::RepositoryCreatedEventContext {
        builder::RepositoryCreatedEventContext::default()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RepositoryCreatedEventContextId(String);
impl std::ops::Deref for RepositoryCreatedEventContextId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<RepositoryCreatedEventContextId> for String {
    fn from(value: RepositoryCreatedEventContextId) -> Self {
        value.0
    }
}
impl From<&RepositoryCreatedEventContextId> for RepositoryCreatedEventContextId {
    fn from(value: &RepositoryCreatedEventContextId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for RepositoryCreatedEventContextId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for RepositoryCreatedEventContextId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryCreatedEventContextId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryCreatedEventContextId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryCreatedEventContextId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RepositoryCreatedEventContextType {
    #[serde(rename = "dev.cdevents.repository.created.0.1.1")]
    DevCdeventsRepositoryCreated011,
}
impl From<&RepositoryCreatedEventContextType> for RepositoryCreatedEventContextType {
    fn from(value: &RepositoryCreatedEventContextType) -> Self {
        value.clone()
    }
}
impl ToString for RepositoryCreatedEventContextType {
    fn to_string(&self) -> String {
        match *self {
            Self::DevCdeventsRepositoryCreated011 => {
                "dev.cdevents.repository.created.0.1.1".to_string()
            }
        }
    }
}
impl std::str::FromStr for RepositoryCreatedEventContextType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "dev.cdevents.repository.created.0.1.1" => Ok(Self::DevCdeventsRepositoryCreated011),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for RepositoryCreatedEventContextType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryCreatedEventContextType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryCreatedEventContextType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for RepositoryCreatedEventContextType {
    fn default() -> Self {
        RepositoryCreatedEventContextType::DevCdeventsRepositoryCreated011
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RepositoryCreatedEventContextVersion(String);
impl std::ops::Deref for RepositoryCreatedEventContextVersion {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<RepositoryCreatedEventContextVersion> for String {
    fn from(value: RepositoryCreatedEventContextVersion) -> Self {
        value.0
    }
}
impl From<&RepositoryCreatedEventContextVersion> for RepositoryCreatedEventContextVersion {
    fn from(value: &RepositoryCreatedEventContextVersion) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for RepositoryCreatedEventContextVersion {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for RepositoryCreatedEventContextVersion {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryCreatedEventContextVersion {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryCreatedEventContextVersion {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryCreatedEventContextVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RepositoryCreatedEventCustomData {
    Variant0(std::collections::HashMap<String, serde_json::Value>),
    Variant1(String),
}
impl From<&RepositoryCreatedEventCustomData> for RepositoryCreatedEventCustomData {
    fn from(value: &RepositoryCreatedEventCustomData) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, serde_json::Value>>
    for RepositoryCreatedEventCustomData
{
    fn from(value: std::collections::HashMap<String, serde_json::Value>) -> Self {
        Self::Variant0(value)
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryCreatedEventSubject {
    pub content: RepositoryCreatedEventSubjectContent,
    pub id: RepositoryCreatedEventSubjectId,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "type")]
    pub type_: RepositoryCreatedEventSubjectType,
}
impl From<&RepositoryCreatedEventSubject> for RepositoryCreatedEventSubject {
    fn from(value: &RepositoryCreatedEventSubject) -> Self {
        value.clone()
    }
}
impl RepositoryCreatedEventSubject {
    pub fn builder() -> builder::RepositoryCreatedEventSubject {
        builder::RepositoryCreatedEventSubject::default()
    }
}
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RepositoryCreatedEventSubjectContent {
    pub name: RepositoryCreatedEventSubjectContentName,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    pub url: RepositoryCreatedEventSubjectContentUrl,
    #[serde(rename = "viewUrl", default, skip_serializing_if = "Option::is_none")]
    pub view_url: Option<String>,
}
impl From<&RepositoryCreatedEventSubjectContent> for RepositoryCreatedEventSubjectContent {
    fn from(value: &RepositoryCreatedEventSubjectContent) -> Self {
        value.clone()
    }
}
impl RepositoryCreatedEventSubjectContent {
    pub fn builder() -> builder::RepositoryCreatedEventSubjectContent {
        builder::RepositoryCreatedEventSubjectContent::default()
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RepositoryCreatedEventSubjectContentName(String);
impl std::ops::Deref for RepositoryCreatedEventSubjectContentName {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<RepositoryCreatedEventSubjectContentName> for String {
    fn from(value: RepositoryCreatedEventSubjectContentName) -> Self {
        value.0
    }
}
impl From<&RepositoryCreatedEventSubjectContentName> for RepositoryCreatedEventSubjectContentName {
    fn from(value: &RepositoryCreatedEventSubjectContentName) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for RepositoryCreatedEventSubjectContentName {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for RepositoryCreatedEventSubjectContentName {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryCreatedEventSubjectContentName {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryCreatedEventSubjectContentName {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryCreatedEventSubjectContentName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RepositoryCreatedEventSubjectContentUrl(String);
impl std::ops::Deref for RepositoryCreatedEventSubjectContentUrl {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<RepositoryCreatedEventSubjectContentUrl> for String {
    fn from(value: RepositoryCreatedEventSubjectContentUrl) -> Self {
        value.0
    }
}
impl From<&RepositoryCreatedEventSubjectContentUrl> for RepositoryCreatedEventSubjectContentUrl {
    fn from(value: &RepositoryCreatedEventSubjectContentUrl) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for RepositoryCreatedEventSubjectContentUrl {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for RepositoryCreatedEventSubjectContentUrl {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryCreatedEventSubjectContentUrl {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryCreatedEventSubjectContentUrl {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryCreatedEventSubjectContentUrl {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RepositoryCreatedEventSubjectId(String);
impl std::ops::Deref for RepositoryCreatedEventSubjectId {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}
impl From<RepositoryCreatedEventSubjectId> for String {
    fn from(value: RepositoryCreatedEventSubjectId) -> Self {
        value.0
    }
}
impl From<&RepositoryCreatedEventSubjectId> for RepositoryCreatedEventSubjectId {
    fn from(value: &RepositoryCreatedEventSubjectId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for RepositoryCreatedEventSubjectId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if value.len() < 1usize {
            return Err("shorter than 1 characters");
        }
        Ok(Self(value.to_string()))
    }
}
impl std::convert::TryFrom<&str> for RepositoryCreatedEventSubjectId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryCreatedEventSubjectId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryCreatedEventSubjectId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl<'de> serde::Deserialize<'de> for RepositoryCreatedEventSubjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        String::deserialize(deserializer)?
            .parse()
            .map_err(|e: &'static str| <D::Error as serde::de::Error>::custom(e.to_string()))
    }
}
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum RepositoryCreatedEventSubjectType {
    #[serde(rename = "repository")]
    Repository,
}
impl From<&RepositoryCreatedEventSubjectType> for RepositoryCreatedEventSubjectType {
    fn from(value: &RepositoryCreatedEventSubjectType) -> Self {
        value.clone()
    }
}
impl ToString for RepositoryCreatedEventSubjectType {
    fn to_string(&self) -> String {
        match *self {
            Self::Repository => "repository".to_string(),
        }
    }
}
impl std::str::FromStr for RepositoryCreatedEventSubjectType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "repository" => Ok(Self::Repository),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for RepositoryCreatedEventSubjectType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for RepositoryCreatedEventSubjectType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for RepositoryCreatedEventSubjectType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl Default for RepositoryCreatedEventSubjectType {
    fn default() -> Self {
        RepositoryCreatedEventSubjectType::Repository
    }
}
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct RepositoryCreatedEvent {
        context: Result<super::RepositoryCreatedEventContext, String>,
        custom_data: Result<Option<super::RepositoryCreatedEventCustomData>, String>,
        custom_data_content_type: Result<Option<String>, String>,
        subject: Result<super::RepositoryCreatedEventSubject, String>,
    }
    impl Default for RepositoryCreatedEvent {
        fn default() -> Self {
            Self {
                context: Err("no value supplied for context".to_string()),
                custom_data: Ok(Default::default()),
                custom_data_content_type: Ok(Default::default()),
                subject: Err("no value supplied for subject".to_string()),
            }
        }
    }
    impl RepositoryCreatedEvent {
        pub fn context<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RepositoryCreatedEventContext>,
            T::Error: std::fmt::Display,
        {
            self.context = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for context: {}", e));
            self
        }
        pub fn custom_data<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<super::RepositoryCreatedEventCustomData>>,
            T::Error: std::fmt::Display,
        {
            self.custom_data = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for custom_data: {}", e));
            self
        }
        pub fn custom_data_content_type<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.custom_data_content_type = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for custom_data_content_type: {}",
                    e
                )
            });
            self
        }
        pub fn subject<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RepositoryCreatedEventSubject>,
            T::Error: std::fmt::Display,
        {
            self.subject = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for subject: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RepositoryCreatedEvent> for super::RepositoryCreatedEvent {
        type Error = String;
        fn try_from(value: RepositoryCreatedEvent) -> Result<Self, String> {
            Ok(Self {
                context: value.context?,
                custom_data: value.custom_data?,
                custom_data_content_type: value.custom_data_content_type?,
                subject: value.subject?,
            })
        }
    }
    impl From<super::RepositoryCreatedEvent> for RepositoryCreatedEvent {
        fn from(value: super::RepositoryCreatedEvent) -> Self {
            Self {
                context: Ok(value.context),
                custom_data: Ok(value.custom_data),
                custom_data_content_type: Ok(value.custom_data_content_type),
                subject: Ok(value.subject),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RepositoryCreatedEventContext {
        id: Result<super::RepositoryCreatedEventContextId, String>,
        source: Result<String, String>,
        timestamp: Result<chrono::DateTime<chrono::offset::Utc>, String>,
        type_: Result<super::RepositoryCreatedEventContextType, String>,
        version: Result<super::RepositoryCreatedEventContextVersion, String>,
    }
    impl Default for RepositoryCreatedEventContext {
        fn default() -> Self {
            Self {
                id: Err("no value supplied for id".to_string()),
                source: Err("no value supplied for source".to_string()),
                timestamp: Err("no value supplied for timestamp".to_string()),
                type_: Err("no value supplied for type_".to_string()),
                version: Err("no value supplied for version".to_string()),
            }
        }
    }
    impl RepositoryCreatedEventContext {
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RepositoryCreatedEventContextId>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<String>,
            T::Error: std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn timestamp<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<chrono::DateTime<chrono::offset::Utc>>,
            T::Error: std::fmt::Display,
        {
            self.timestamp = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for timestamp: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RepositoryCreatedEventContextType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
        pub fn version<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RepositoryCreatedEventContextVersion>,
            T::Error: std::fmt::Display,
        {
            self.version = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for version: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RepositoryCreatedEventContext> for super::RepositoryCreatedEventContext {
        type Error = String;
        fn try_from(value: RepositoryCreatedEventContext) -> Result<Self, String> {
            Ok(Self {
                id: value.id?,
                source: value.source?,
                timestamp: value.timestamp?,
                type_: value.type_?,
                version: value.version?,
            })
        }
    }
    impl From<super::RepositoryCreatedEventContext> for RepositoryCreatedEventContext {
        fn from(value: super::RepositoryCreatedEventContext) -> Self {
            Self {
                id: Ok(value.id),
                source: Ok(value.source),
                timestamp: Ok(value.timestamp),
                type_: Ok(value.type_),
                version: Ok(value.version),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RepositoryCreatedEventSubject {
        content: Result<super::RepositoryCreatedEventSubjectContent, String>,
        id: Result<super::RepositoryCreatedEventSubjectId, String>,
        source: Result<Option<String>, String>,
        type_: Result<super::RepositoryCreatedEventSubjectType, String>,
    }
    impl Default for RepositoryCreatedEventSubject {
        fn default() -> Self {
            Self {
                content: Err("no value supplied for content".to_string()),
                id: Err("no value supplied for id".to_string()),
                source: Ok(Default::default()),
                type_: Err("no value supplied for type_".to_string()),
            }
        }
    }
    impl RepositoryCreatedEventSubject {
        pub fn content<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RepositoryCreatedEventSubjectContent>,
            T::Error: std::fmt::Display,
        {
            self.content = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for content: {}", e));
            self
        }
        pub fn id<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RepositoryCreatedEventSubjectId>,
            T::Error: std::fmt::Display,
        {
            self.id = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for id: {}", e));
            self
        }
        pub fn source<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.source = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for source: {}", e));
            self
        }
        pub fn type_<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RepositoryCreatedEventSubjectType>,
            T::Error: std::fmt::Display,
        {
            self.type_ = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for type_: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RepositoryCreatedEventSubject> for super::RepositoryCreatedEventSubject {
        type Error = String;
        fn try_from(value: RepositoryCreatedEventSubject) -> Result<Self, String> {
            Ok(Self {
                content: value.content?,
                id: value.id?,
                source: value.source?,
                type_: value.type_?,
            })
        }
    }
    impl From<super::RepositoryCreatedEventSubject> for RepositoryCreatedEventSubject {
        fn from(value: super::RepositoryCreatedEventSubject) -> Self {
            Self {
                content: Ok(value.content),
                id: Ok(value.id),
                source: Ok(value.source),
                type_: Ok(value.type_),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct RepositoryCreatedEventSubjectContent {
        name: Result<super::RepositoryCreatedEventSubjectContentName, String>,
        owner: Result<Option<String>, String>,
        url: Result<super::RepositoryCreatedEventSubjectContentUrl, String>,
        view_url: Result<Option<String>, String>,
    }
    impl Default for RepositoryCreatedEventSubjectContent {
        fn default() -> Self {
            Self {
                name: Err("no value supplied for name".to_string()),
                owner: Ok(Default::default()),
                url: Err("no value supplied for url".to_string()),
                view_url: Ok(Default::default()),
            }
        }
    }
    impl RepositoryCreatedEventSubjectContent {
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RepositoryCreatedEventSubjectContentName>,
            T::Error: std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {}", e));
            self
        }
        pub fn owner<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.owner = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for owner: {}", e));
            self
        }
        pub fn url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<super::RepositoryCreatedEventSubjectContentUrl>,
            T::Error: std::fmt::Display,
        {
            self.url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for url: {}", e));
            self
        }
        pub fn view_url<T>(mut self, value: T) -> Self
        where
            T: std::convert::TryInto<Option<String>>,
            T::Error: std::fmt::Display,
        {
            self.view_url = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for view_url: {}", e));
            self
        }
    }
    impl std::convert::TryFrom<RepositoryCreatedEventSubjectContent>
        for super::RepositoryCreatedEventSubjectContent
    {
        type Error = String;
        fn try_from(value: RepositoryCreatedEventSubjectContent) -> Result<Self, String> {
            Ok(Self {
                name: value.name?,
                owner: value.owner?,
                url: value.url?,
                view_url: value.view_url?,
            })
        }
    }
    impl From<super::RepositoryCreatedEventSubjectContent> for RepositoryCreatedEventSubjectContent {
        fn from(value: super::RepositoryCreatedEventSubjectContent) -> Self {
            Self {
                name: Ok(value.name),
                owner: Ok(value.owner),
                url: Ok(value.url),
                view_url: Ok(value.view_url),
            }
        }
    }
}
