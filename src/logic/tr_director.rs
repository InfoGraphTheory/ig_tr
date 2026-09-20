
use ig_tools::hashing_tools;
use crate::{triple_facade::TripleFacade, triple_store::TripleStore, InfoTriple, InfoTable};

#[derive(Clone)]
pub struct TrDirector<S:TripleStore> {
   pub triples: TripleFacade<S>, 
}

impl <S:TripleStore> TrDirector<S>{

    pub fn new(triples: TripleFacade<S>) -> TrDirector<S> {
        TrDirector{triples}
    } 

    pub fn get_all_info_triples_from_info_table(&mut self, infotable_name: String) -> Vec<InfoTriple>{
        self.triples.get_all_info_triples_from_info_table(infotable_name.as_str()).clone()
    }
    
    //TODO:: may be put in triple_store for possible performance optimizations or on facade for
    //reuse in other directors. 
    pub fn get_n_flatten_all_from_table_list(&mut self, table_list: Vec<String>) -> InfoTable {

        table_list
            .iter()
            .flat_map(|t|{
                self.get_all_info_triples_from_info_table(t.to_string())
            })
            .collect()
    }

    pub fn create_triple(&mut self, id1: String, id2: String) -> InfoTriple {
        let it = InfoTriple::from(hashing_tools::concat_n_hash(id1.as_str(), id2.as_str()));
        self.triples.add_to_infotable("main_table".to_string(),it.clone());
        it
    }

    pub fn clear_infotable(&mut self, infotable_name: String) {
        self.triples.clear_infotable(infotable_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::triple_store_fs::TripleStoreFS;

    fn new_director(space_id: &str) -> TrDirector<TripleStoreFS> {
        TrDirector::new(TripleFacade::new(TripleStoreFS::new(space_id.to_string())))
    }

    #[test]
    fn create_triple_hashes_id1_and_id2_and_adds_it_to_main_table() {
        let space_id = format!("ig_tr_test_director_create_{}", std::process::id());
        let mut director = new_director(&space_id);
        director.clear_infotable("main_table".to_string());

        let id1 = "a".repeat(64);
        let id2 = "b".repeat(64);
        // id1 and id2 are already in sorted order, so create_triple's internal sort-before-hash
        // (ig_tools::hashing_tools::concat_n_hash) is a no-op here and id1/id2 come back as-given.
        let triple = director.create_triple(id1.clone(), id2.clone());

        let stored = director.get_all_info_triples_from_info_table("main_table".to_string());
        assert!(stored.iter().any(|t| t.id == triple.id));
        assert_eq!(triple.id1, id1);
        assert_eq!(triple.id2, id2);
    }

    #[test]
    fn create_triple_is_order_independent_because_inputs_get_sorted_before_hashing() {
        // Vital property: create_triple hashes id1/id2 via concat_n_hash, which sorts the pair
        // lexicographically before hashing. So calling it with the pair already in sorted order
        // and calling it with the pair reversed must produce the exact same triple (same id, and
        // id1/id2 normalized to the same sorted order) - otherwise the same unordered relationship
        // could be recorded twice under two different triple ids depending on argument order.
        let space_id = format!("ig_tr_test_director_order_independence_{}", std::process::id());
        let mut director = new_director(&space_id);
        director.clear_infotable("main_table".to_string());

        let id_a = "a".repeat(64);
        let id_b = "b".repeat(64);
        let already_ordered = director.create_triple(id_a.clone(), id_b.clone());
        let needs_ordering = director.create_triple(id_b.clone(), id_a.clone());

        assert_eq!(already_ordered.id, needs_ordering.id);
        assert_eq!(already_ordered.id1, needs_ordering.id1);
        assert_eq!(already_ordered.id2, needs_ordering.id2);
        assert_eq!(already_ordered.id1, id_a);
        assert_eq!(already_ordered.id2, id_b);
    }

    #[test]
    fn get_n_flatten_all_from_table_list_merges_multiple_tables() {
        let space_id = format!("ig_tr_test_director_flatten_{}", std::process::id());
        let mut director = new_director(&space_id);
        director.clear_infotable("table_a".to_string());
        director.clear_infotable("table_b".to_string());

        director.triples.add_to_infotable("table_a".to_string(), InfoTriple::new("t1", "a", "b"));
        director.triples.add_to_infotable("table_b".to_string(), InfoTriple::new("t2", "c", "d"));

        let flattened = director.get_n_flatten_all_from_table_list(vec!["table_a".to_string(), "table_b".to_string()]);

        assert!(flattened.get_info_triple("t1").is_some());
        assert!(flattened.get_info_triple("t2").is_some());
    }

    #[test]
    fn clear_infotable_empties_the_table() {
        let space_id = format!("ig_tr_test_director_clear_{}", std::process::id());
        let mut director = new_director(&space_id);
        director.triples.add_to_infotable("main_table".to_string(), InfoTriple::new("t1", "a", "b"));

        director.clear_infotable("main_table".to_string());

        assert!(director.get_all_info_triples_from_info_table("main_table".to_string()).is_empty());
    }
}
