use crate::deprecation::DeprecationStrategy;
use crate::normalization::Normalization;
use proc_macro2::Ident;
use std::path::{Path, PathBuf};
use syn::{self, Visibility};

/// Which context is this code generation effort taking place.
#[derive(Debug)]
pub enum CodegenMode {
    /// The graphql-client CLI.
    Cli,
    /// The derive macro defined in graphql_query_derive.
    Derive,
}

/// Used to configure code generation.
pub struct GraphQLClientCodegenOptions {
    /// Which context is this code generation effort taking place.
    pub mode: CodegenMode,
    /// Name of the operation we want to generate code for. If it does not match, we use all queries.
    pub operation_name: Option<String>,
    /// The name of implementation target struct.
    pub struct_name: Option<String>,
    /// The struct for which we derive GraphQLQuery.
    struct_ident: Option<Ident>,
    /// Comma-separated list of additional traits we want to derive for variables.
    variables_derives: Option<String>,
    /// Comma-separated list of additional traits we want to derive for responses.
    response_derives: Option<String>,
    /// The deprecation strategy to adopt.
    deprecation_strategy: Option<DeprecationStrategy>,
    /// Target module visibility.
    module_visibility: Option<Visibility>,
    /// A path to a file to include in the module to force Cargo to take into account changes in
    /// the query files when recompiling.
    query_file: Option<PathBuf>,
    /// A path to a file to include in the module to force Cargo to take into account changes in
    /// the schema files when recompiling.
    schema_file: Option<PathBuf>,
    /// Normalization pattern for query types and names.
    normalization: Normalization,
    /// Custom scalar definitions module path
    custom_scalars_module: Option<syn::Path>,
    /// List of externally defined enum types. Type names must match those used in the schema exactly.
    extern_enums: Vec<String>,
    /// Flag to trigger generation of Other variant for fragments Enum
    fragments_other_variant: bool,
    /// Skip Serialization of None values.
    skip_serializing_none: bool,
    /// Path to the serde crate.
    serde_path: syn::Path,
}

impl GraphQLClientCodegenOptions {
    /// Creates an empty options object with default params. It probably wants to be configured.
    pub fn new(mode: CodegenMode) -> GraphQLClientCodegenOptions {
        GraphQLClientCodegenOptions {
            mode,
            variables_derives: Default::default(),
            response_derives: Default::default(),
            deprecation_strategy: Default::default(),
            module_visibility: Default::default(),
            operation_name: Default::default(),
            struct_ident: Default::default(),
            struct_name: Default::default(),
            query_file: Default::default(),
            schema_file: Default::default(),
            normalization: Normalization::None,
            custom_scalars_module: Default::default(),
            extern_enums: Default::default(),
            fragments_other_variant: Default::default(),
            skip_serializing_none: Default::default(),
            serde_path: syn::parse_quote!(::serde),
        }
    }

    /// The visibility (public/private) to apply to the target module.
    pub(crate) fn module_visibility(&self) -> &Visibility {
        self.module_visibility
            .as_ref()
            .unwrap_or(&Visibility::Inherited)
    }

    /// The deprecation strategy to adopt.
    pub(crate) fn deprecation_strategy(&self) -> DeprecationStrategy {
        self.deprecation_strategy.clone().unwrap_or_default()
    }

    /// A path to a file to include in the module to force Cargo to take into account changes in
    /// the query files when recompiling.
    pub fn set_query_file(&mut self, path: PathBuf) {
        self.query_file = Some(path);
    }

    /// Comma-separated list of additional traits we want to derive for variables.
    pub fn variables_derives(&self) -> Option<&str> {
        self.variables_derives.as_deref()
    }

    /// Comma-separated list of additional traits we want to derive for variables.
    pub fn set_variables_derives(&mut self, variables_derives: String) {
        self.variables_derives = Some(variables_derives);
    }

    /// All the variable derives to be rendered.
    pub fn all_variable_derives(&self) -> impl Iterator<Item = &str> {
        let additional = self
            .variables_derives
            .as_deref()
            .into_iter()
            .flat_map(|s| s.split(','))
            .map(|s| s.trim());

        std::iter::once("Serialize").chain(additional)
    }

    /// Traits we want to derive for responses.
    pub fn all_response_derives(&self) -> impl Iterator<Item = &str> {
        let base_derives = std::iter::once("Deserialize");

        base_derives.chain(
            self.additional_response_derives()
                .filter(|additional| additional != &"Deserialize"),
        )
    }

    /// Additional traits we want to derive for responses.
    pub fn additional_response_derives(&self) -> impl Iterator<Item = &str> {
        self.response_derives
            .as_deref()
            .into_iter()
            .flat_map(|s| s.split(','))
            .map(|s| s.trim())
    }

    /// Comma-separated list of additional traits we want to derive for responses.
    pub fn set_response_derives(&mut self, response_derives: String) {
        self.response_derives = Some(response_derives);
    }

    /// The deprecation strategy to adopt.
    pub fn set_deprecation_strategy(&mut self, deprecation_strategy: DeprecationStrategy) {
        self.deprecation_strategy = Some(deprecation_strategy);
    }

    /// Target module visibility.
    pub fn set_module_visibility(&mut self, visibility: Visibility) {
        self.module_visibility = Some(visibility);
    }

    /// The name of implementation target struct.
    pub fn set_struct_name(&mut self, struct_name: String) {
        self.struct_name = Some(struct_name);
    }

    /// Name of the operation we want to generate code for. If none is selected, it means all
    /// operations.
    pub fn set_operation_name(&mut self, operation_name: String) {
        self.operation_name = Some(operation_name);
    }

    /// A path to a file to include in the module to force Cargo to take into account changes in
    /// the schema files when recompiling.
    pub fn schema_file(&self) -> Option<&Path> {
        self.schema_file.as_deref()
    }

    /// A path to a file to include in the module to force Cargo to take into account changes in
    /// the query files when recompiling.
    pub fn query_file(&self) -> Option<&Path> {
        self.query_file.as_deref()
    }

    /// The identifier to use when referring to the struct implementing GraphQLQuery, if any.
    pub fn set_struct_ident(&mut self, ident: Ident) {
        self.struct_ident = Some(ident);
    }

    /// The identifier to use when referring to the struct implementing GraphQLQuery, if any.
    pub fn struct_ident(&self) -> Option<&proc_macro2::Ident> {
        self.struct_ident.as_ref()
    }

    /// Set the normalization mode for the generated code.
    pub fn set_normalization(&mut self, norm: Normalization) {
        self.normalization = norm;
    }

    /// The normalization mode for the generated code.
    pub fn normalization(&self) -> &Normalization {
        &self.normalization
    }

    /// Get the custom scalar definitions module
    pub fn custom_scalars_module(&self) -> Option<&syn::Path> {
        self.custom_scalars_module.as_ref()
    }

    /// Set the custom scalar definitions module
    pub fn set_custom_scalars_module(&mut self, module: syn::Path) {
        self.custom_scalars_module = Some(module)
    }

    /// Get the externally defined enums type names
    pub fn extern_enums(&self) -> &[String] {
        &self.extern_enums
    }

    /// Set the externally defined enums type names
    pub fn set_extern_enums(&mut self, enums: Vec<String>) {
        self.extern_enums = enums;
    }

    /// Set the graphql client codegen options's fragments other variant.
    pub fn set_fragments_other_variant(&mut self, fragments_other_variant: bool) {
        self.fragments_other_variant = fragments_other_variant;
    }

    /// Get a reference to the graphql client codegen options's fragments other variant.
    pub fn fragments_other_variant(&self) -> &bool {
        &self.fragments_other_variant
    }

    /// Set the graphql client codegen option's skip none value.
    pub fn set_skip_serializing_none(&mut self, skip_serializing_none: bool) {
        self.skip_serializing_none = skip_serializing_none
    }

    /// Get a reference to the graphql client codegen option's skip none value.
    pub fn skip_serializing_none(&self) -> &bool {
        &self.skip_serializing_none
    }

    /// Set the path to used to resolve serde traits.
    pub fn set_serde_path(&mut self, path: syn::Path) {
        self.serde_path = path;
    }

    /// Get a reference to the path used to resolve serde traits.
    pub fn serde_path(&self) -> &syn::Path {
        &self.serde_path
    }
}
