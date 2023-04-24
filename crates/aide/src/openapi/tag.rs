use crate::openapi::*;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Adds metadata to a single tag that is used by the
/// Operation Object. It is not mandatory to have a
/// Tag Object per tag defined in the Operation Object instances.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, schemars::JsonSchema)]
pub struct Tag {
    /// REQUIRED. The name of the tag.
    pub name: String,
    /// A description for the tag.
    /// CommonMark syntax MAY be used for rich text representation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Additional external documentation for this tag.
    #[serde(rename = "externalDocs", skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentation>,
    /// Inline extensions to this object.
    #[serde(flatten, deserialize_with = "crate::util::deserialize_extensions")]
    pub extensions: IndexMap<String, serde_json::Value>,
}

impl Tag {
    /// Creates new named `Tag`
    /// ```
    /// # use aide::openapi::Tag;
    ///
    /// let tag = Tag::new("pet");
    /// ```
    pub fn new(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            name,
            ..Self::default()
        }
    }

    /// Sets a description for this `Tag`
    /// ```
    /// # use aide::openapi::Tag;
    ///
    /// let tag = Tag::new("pet").description("Pets operations");
    /// ```
    pub fn description(self, description: impl Into<String>) -> Self {
        let description = Some(description.into());
        Self {
            description,
            ..self
        }
    }

    /// Sets external documentation for this `Tag`
    /// ```
    /// # use aide::openapi::Tag;
    /// # use aide::openapi::ExternalDocumentation;
    ///
    /// let docs = ExternalDocumentation { url: "https://example.com".into(), ..Default::default() };
    /// let tag = Tag::new("pet").external_docs(docs);
    /// ```
    pub fn external_docs(self, external_docs: ExternalDocumentation) -> Self {
        Self {
            external_docs: Some(external_docs),
            ..self
        }
    }

    /// Sets/adds extensions to this `Tag`
    /// ```
    /// # use aide::openapi::Tag;
    ///
    /// let x_internal_id = serde_json::to_value("0").unwrap();
    /// let tag = Tag::new("foo").extensions([("x-internal-id", x_internal_id)]);
    /// ```

    pub fn extensions(
        self,
        extensions: impl IntoIterator<Item = (impl Into<String>, serde_json::Value)>,
    ) -> Self {
        let more_extensions = extensions
            .into_iter()
            .map(|(key, value)| (key.into(), value));
        let mut extensions = self.extensions;
        extensions.extend(more_extensions);
        Self { extensions, ..self }
    }
}
