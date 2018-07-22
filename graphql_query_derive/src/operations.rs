use selection::Selection;

pub enum OperationType {
    Query,
    Mutation,
    Subscription,
}

#[derive(Debug)]
pub struct Operation {
    pub name: String,
    pub selection: Selection,
}

pub struct Operations(Vec<Operation>);

impl Operations {
    fn from_document(doc: ::graphql_parser::query::Document) -> Result<Self, ::failure::Error> {
        unimplemented!()
    }
}
