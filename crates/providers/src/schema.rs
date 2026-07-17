//! Output-schema registry v0: named schemas with required top-level fields.
//! Full JSON-Schema validation arrives with the real adapters (M1); the
//! contract point — every brain output validates against a *named, versioned*
//! schema before anything downstream trusts it — is fixed now.

use std::collections::HashMap;

pub struct SchemaRegistry {
    required: HashMap<String, Vec<&'static str>>,
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        let mut required = HashMap::new();
        required.insert("phase_summary.v1".to_owned(), vec!["schema", "what"]);
        required.insert("plan.v1".to_owned(), vec!["tasks"]);
        required.insert("builder_step.v1".to_owned(), vec!["edits"]);
        required.insert(
            "review_findings.v1".to_owned(),
            vec!["findings", "disposition"],
        );
        // A model-generated engineering specification (Build Feature recipe).
        required.insert(
            "specification.v1".to_owned(),
            vec!["overview", "acceptance_criteria"],
        );
        Self { required }
    }
}

impl SchemaRegistry {
    pub fn knows(&self, schema_id: &str) -> bool {
        self.required.contains_key(schema_id)
    }

    /// Ok, or the list of missing required fields. Unknown schema ids fail
    /// closed (everything is "missing").
    pub fn validate(&self, schema_id: &str, output: &serde_json::Value) -> Result<(), Vec<String>> {
        let Some(required) = self.required.get(schema_id) else {
            return Err(vec![format!("<unknown schema id: {schema_id}>")]);
        };
        let missing: Vec<String> = required
            .iter()
            .filter(|f| output.get(**f).is_none())
            .map(|f| (*f).to_owned())
            .collect();
        if missing.is_empty() {
            Ok(())
        } else {
            Err(missing)
        }
    }
}
