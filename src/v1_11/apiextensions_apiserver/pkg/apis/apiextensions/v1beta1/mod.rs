
mod custom_resource_column_definition;
pub use self::custom_resource_column_definition::CustomResourceColumnDefinition;

mod custom_resource_definition;
pub use self::custom_resource_definition::CustomResourceDefinition;
#[cfg(feature = "api")] pub use self::custom_resource_definition::{CreateCustomResourceDefinitionOptional, CreateCustomResourceDefinitionResponse};
#[cfg(feature = "api")] pub use self::custom_resource_definition::DeleteCollectionCustomResourceDefinitionResponse;
#[cfg(feature = "api")] pub use self::custom_resource_definition::DeleteCustomResourceDefinitionResponse;
#[cfg(feature = "api")] pub use self::custom_resource_definition::ListCustomResourceDefinitionResponse;
#[cfg(feature = "api")] pub use self::custom_resource_definition::PatchCustomResourceDefinitionResponse;
#[cfg(feature = "api")] pub use self::custom_resource_definition::PatchCustomResourceDefinitionStatusResponse;
#[cfg(feature = "api")] pub use self::custom_resource_definition::{ReadCustomResourceDefinitionOptional, ReadCustomResourceDefinitionResponse};
#[cfg(feature = "api")] pub use self::custom_resource_definition::{ReadCustomResourceDefinitionStatusOptional, ReadCustomResourceDefinitionStatusResponse};
#[cfg(feature = "api")] pub use self::custom_resource_definition::{ReplaceCustomResourceDefinitionOptional, ReplaceCustomResourceDefinitionResponse};
#[cfg(feature = "api")] pub use self::custom_resource_definition::{ReplaceCustomResourceDefinitionStatusOptional, ReplaceCustomResourceDefinitionStatusResponse};
#[cfg(feature = "api")] pub use self::custom_resource_definition::WatchCustomResourceDefinitionResponse;

mod custom_resource_definition_condition;
pub use self::custom_resource_definition_condition::CustomResourceDefinitionCondition;

mod custom_resource_definition_names;
pub use self::custom_resource_definition_names::CustomResourceDefinitionNames;

mod custom_resource_definition_spec;
pub use self::custom_resource_definition_spec::CustomResourceDefinitionSpec;

mod custom_resource_definition_status;
pub use self::custom_resource_definition_status::CustomResourceDefinitionStatus;

mod custom_resource_definition_version;
pub use self::custom_resource_definition_version::CustomResourceDefinitionVersion;

mod custom_resource_subresource_scale;
pub use self::custom_resource_subresource_scale::CustomResourceSubresourceScale;

mod custom_resource_subresource_status;
pub use self::custom_resource_subresource_status::CustomResourceSubresourceStatus;

mod custom_resource_subresources;
pub use self::custom_resource_subresources::CustomResourceSubresources;

mod custom_resource_validation;
pub use self::custom_resource_validation::CustomResourceValidation;

mod external_documentation;
pub use self::external_documentation::ExternalDocumentation;

mod json;
pub use self::json::JSON;

mod json_schema_props;
pub use self::json_schema_props::JSONSchemaProps;

mod json_schema_props_or_array;
pub use self::json_schema_props_or_array::JSONSchemaPropsOrArray;

mod json_schema_props_or_bool;
pub use self::json_schema_props_or_bool::JSONSchemaPropsOrBool;

mod json_schema_props_or_string_array;
pub use self::json_schema_props_or_string_array::JSONSchemaPropsOrStringArray;
