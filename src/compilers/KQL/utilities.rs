use crate::core_structures::row::Cell;

/// A general structure matching a Query after it has been compiled
pub struct Query {
    /// Any recursive queries which are referenced and must be completed beforehand
    pub recursive: Option<Query>,

    /// The filter for returned data
    pub filter: Option<Filter>,

    /// The database the query references
    pub database: Option<String>,

    /// The table the query references
    pub table: Option<String>,

    /// Any variables produced by said query
    pub variables: Option<Vec<Variable>>,

    /// The operation the query has requested
    pub operation: Option<Operation>,

    /// The fields the query wishes to return
    pub fields: Option<Vec<String>>,

    /// The values the query is to insert into the table
    pub values: Option<Vec<Cell>>,

}

/// The structure used for filtering data from a query where possible
pub struct Filter {
    pub subs: Vec<SubFilter>,
}

/// A sub-structure used for defining filters on content
pub struct SubFilter {
    /// The (optional) field which the right assignment is being compared against
    pub field: Option<String>,

    /// The (optional) value that the right assignment is being compared against
    pub left: Option<Cell>,

    /// The (required) value that the field or left assignment is being compared against.
    pub right: Option<Cell>,
}

/// A structure defining a variable
pub struct Variable {
    /// The type of variable this is
    pub var_type: VariableType,

    /// The visibility state of the variable
    pub privacy: VariablePrivacy,

    /// The name of the variable
    pub name: String,

    /// The value which it stores
    pub value: Option<Cell>,
}

/// An enumerator defining the types of variables possible
pub enum VariableType {
    /// A type of variable which cannot be changed, making it a constant
    Constant,

    /// A variable which is not constant, meaning it can be be lost at any time (during program shutdown), making it ephemeral
    Ephemeral,
}

/// An enumerator defining the privacy types of variables
pub enum VariablePrivacy {
    /// Can be seen across all connections (Always used for constant values)
    Public,

    /// Can only be seen by the connection which created it
    Private,
}

/// An enumerator defining the different operations allowed to queries
pub enum Operation {
    /// Used on queries which return data
    Get,

    /// Used on queries which provide new data
    Insert,

    /// Used on queries which update existing data
    Modify,

    /// Used on queries which remove existing data
    Prune,
}