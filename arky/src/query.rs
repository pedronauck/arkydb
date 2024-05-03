use crate::db::DB;
use arkycore::types::{Data, DynResult, NodeID};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryOperation {
    ByID(NodeID),
    ByIndex(String, Data),
    ByEntityName(String),
    ByEdge(NodeID, NodeID),
    ByEdgeLabel(String),
    ByEdgeData(Data),
    ByEdgeFrom(NodeID),
    ByEdgeTo(NodeID),
    // Filter(Box<dyn Fn(&dyn Node) -> bool>),
    FilterByProp(String, Data),
    ShortPath(NodeID, NodeID),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryBuilder<'a, D: DB> {
    db: &'a D,
}
impl<'a, D: DB> QueryBuilder<'a, D> {
    pub fn new(db: &'a D) -> Self {
        Self { db }
    }
    pub fn by_id(&self, id: &NodeID) -> &Self {
        todo!()
    }
    pub fn by_index<C>(&self, index: &str, value: C) -> &Self {
        todo!()
    }
    pub fn by_entity_name(&self, entity_name: String) -> &Self {
        todo!()
    }
    pub fn by_edge(&self, from: &NodeID, to: &NodeID) -> &Self {
        todo!()
    }
    pub fn by_edge_label(&self, edge_type: &str) -> &Self {
        todo!()
    }
    pub fn by_edge_data(&self, data: &Data) -> &Self {
        todo!()
    }
    pub fn by_edge_from(&self, from: &NodeID) -> &Self {
        todo!()
    }
    pub fn by_edge_to(&self, to: &NodeID) -> &Self {
        todo!()
    }
    pub fn filter<T>(&self, cb: &impl Fn(&T) -> bool) -> &Self {
        todo!()
    }
    pub fn filter_by_prop<C>(&self, prop: &str, value: C) -> &Self {
        todo!()
    }
    pub fn short_path(&self, from: &NodeID, to: &NodeID) -> &Self {
        todo!()
    }
    pub fn build(&self) -> DynResult<QueryExecutor<&'a D>> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QueryExecutor<'a, D> {
    db: &'a D,
    operations: Vec<QueryOperation>,
    result: Vec<Data>,
}
impl<'a, D: DB> QueryExecutor<'a, D> {
    pub fn sort_by_prop(&self, prop: &str) -> &Self {
        todo!()
    }
    pub fn limit(&self, limit: usize) -> &Self {
        todo!()
    }
    pub fn skip(&self, skip: usize) -> &Self {
        todo!()
    }
    pub fn count(&self) -> DynResult<usize> {
        todo!()
    }
    // pub fn sort(&self, cb: &impl Fn(&dyn Node, &dyn Node) -> bool) -> &Self {
    //     todo!()
    // }
    // pub fn update(&self, cb: &impl Fn<T: Node>(&mut T) -> impl Node) -> &Self {
    //     todo!()
    // }
    pub fn exec<R>(&self) -> DynResult<R> {
        todo!()
    }
}
