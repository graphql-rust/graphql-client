use field_type::FieldType;
use graphql_parser;

#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub ty: FieldType,
    pub default: Option<graphql_parser::query::Value>,
}

impl ::std::convert::From<graphql_parser::query::VariableDefinition> for Variable {
    fn from(def: graphql_parser::query::VariableDefinition) -> Variable {
        Variable {
            name: def.name,
            ty: FieldType::from(def.var_type),
            default: def.default_value,
        }
    }
}
