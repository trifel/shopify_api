// Field returns Cost value
// Scalar     0
// Enum       0
// Object     1
// Interface  1
// Union      1
// Mutation   10
// Connection 2 + 1 per edge (first or last argument)

#[derive(Clone, Debug)]
pub enum ShopifyGraphQLType {
    Scalar,
    Enum,
    Object,
    Interface,
    Union,
    Mutation,
    Connection,
}

pub mod product;

static SHOPIFY_GRAPHQL_COST_SCALAR: u8 = 0;
static SHOPIFY_GRAPHQL_COST_ENUM: u8 = 0;
static SHOPIFY_GRAPHQL_COST_OBJECT: u8 = 1;
static SHOPIFY_GRAPHQL_COST_INTERFACE: u8 = 1;
static SHOPIFY_GRAPHQL_COST_UNION: u8 = 1;
static SHOPIFY_GRAPHQL_COST_MUTATION: u8 = 10;
static SHOPIFY_GRAPHQL_COST_CONNECTION: u8 = 2;
static SHOPIFY_GRAPHQL_COST_CONNECTION_EDGE: u8 = 1;

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ShopifyGraphQLComposer {
    /// The generated GraphQL query
    query_str: String,

    /// The query object
    queries: HashMap<String, ShopifyGraphQLComposerQuery>,

    /// The variables listing (required or optionals)
    variables: Vec<ShopifyGraphQLComposerVariableType>,
}

impl Default for ShopifyGraphQLComposer {
    /// Create a new GraphQL composer
    /// # Example
    /// ```
    /// use shopify_api::graphql::composer::ShopifyGraphQLComposer;
    ///
    /// let composer = ShopifyGraphQLComposer::default();
    /// ```
    /// # Returns
    /// A new GraphQL composer
    fn default() -> Self {
        ShopifyGraphQLComposer {
            query_str: String::new(),
            queries: HashMap::new(),
            variables: Vec::new(),
        }
    }
}

impl ShopifyGraphQLComposer {
    /// Add a new query to the composer
    /// # Example
    /// ```
    /// use shopify_api::graphql::composer::ShopifyGraphQLComposer;
    /// use shopify_api::graphql::composer::ShopifyGraphQLComposerQuery;
    ///
    /// let mut composer = ShopifyGraphQLComposer::default();
    /// let query = composer.add_query("myquery", ShopifyGraphQLComposerQuery::Query("query { shop { name } }"));
    /// ```
    /// # Arguments
    /// * `name` - The query name
    /// * `query` - The query object
    /// # Returns
    /// The query object
    /// # Panics
    /// If the query name already exists
    pub fn add_query(
        &mut self,
        name: &str,
        query: ShopifyGraphQLComposerQuery,
    ) -> &ShopifyGraphQLComposerQuery {
        if self.queries.contains_key(name) {
            panic!("Query name already exists");
        }

        self.queries.insert(name.to_string(), query);
        self.queries.get(name).unwrap()
    }
}

/// Variable type enum
#[derive(Clone, Debug)]
pub enum ShopifyGraphQLComposerVariableType {
    Required(String),
    Optional(String),
}

/// Main Query enum for structure
#[derive(Clone, Debug)]
pub enum ShopifyGraphQLComposerQuery {
    Query(String),
    Mutation(String),
}