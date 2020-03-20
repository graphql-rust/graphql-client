#[derive(Debug, Clone)]
pub(crate) enum OperationType {
    Query,
    Mutation,
    Subscription,
}
