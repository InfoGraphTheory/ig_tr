use std::collections::HashSet;

use crate::{InfoTriple, InfoTable};

use super::triple_store::TripleStore;


#[derive(Clone)]
pub struct TripleFacade<T:TripleStore> {
    pub storage: T, 
}

impl<T:TripleStore> TripleFacade<T> {

    pub fn new(storage: T) -> Self{
        TripleFacade {storage}
    }

    pub fn add_to_infotable(&mut self, infotable_name: String, infotriple: InfoTriple) {
        self.append_info_table(&infotable_name, &infotriple.to_tuple_string());
    }
    
    pub fn append_info_table(&mut self, infotable_name: &str, infotriple: &str) {
        self.storage.append_info_table(infotable_name, infotriple);
    }

    pub fn get_ids_from_info_table(&mut self, infotable_name: &str) -> Vec<String> {

            self.storage.get_info_table(infotable_name)
                    .lines()
                    .map(|x| {
            let (id, _) = x.split_once(' ').unwrap();
            id.to_string()
        })
        .collect()
    }

    pub fn get_all_ids_from_info_table(&mut self, infotable_name: &str) -> HashSet<String> {

        self.storage.get_info_table(infotable_name)
                    .lines()
                    .flat_map(|x| {
                        x.split(' ').map(|y| y.to_string())
                    })
                    .collect()
    }

    pub fn get_all_info_triples_from_info_table(&mut self, infotable_name: &str) -> Vec<InfoTriple> {
        self.storage.get_info_table(infotable_name)
            .lines()
            .map(|x|InfoTriple::from(x.to_string()))
            .collect()
    }
    
    pub fn get_info_table_as_info_table(&mut self, infotable_name: &str) -> InfoTable {
        self.storage.get_info_table(infotable_name)
            .lines()
            .map(|x|InfoTriple::from(x.to_string()))
            .collect()
    }

    pub fn select_from_info_table(&mut self, infotable_name: &str, where_id: &str) -> Vec<InfoTriple> {
        self.storage.select_from_info_table(infotable_name, where_id)
            .lines()
            .map(|x|InfoTriple::from(x.to_string()))
            .collect()
    }

    pub fn get_all_ids_from_info_table_select(&mut self, infotable_name: &str, where_id: &str) -> HashSet<String> {

        self.storage.select_from_info_table(infotable_name, where_id)
                    .lines()
                    .flat_map(|x| {
                        x.split_once(' ').unwrap_or(("",""))
                            .1
                            .split(' ')
                            .map(|y| y.to_string())
                            .filter(|z| z.ne(where_id))
                    })
                    .collect()
    }

    pub fn clear_infotable(&mut self, infotable_name: String) {
        self.storage.clear_infotable(infotable_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::triple_store_fs::TripleStoreFS;

    fn new_facade(space_id: &str) -> TripleFacade<TripleStoreFS> {
        TripleFacade::new(TripleStoreFS::new(space_id.to_string()))
    }

    #[test]
    fn add_to_infotable_and_get_all_info_triples_round_trips() {
        let space_id = format!("ig_tr_test_facade_round_trip_{}", std::process::id());
        let mut facade = new_facade(&space_id);
        facade.clear_infotable("main_table".to_string());

        facade.add_to_infotable("main_table".to_string(), InfoTriple::new("t1", "a", "b"));
        facade.add_to_infotable("main_table".to_string(), InfoTriple::new("t2", "c", "d"));

        let triples = facade.get_all_info_triples_from_info_table("main_table");
        assert_eq!(triples.len(), 2);
        assert!(triples.iter().any(|t| t.id == "t1" && t.id1 == "a" && t.id2 == "b"));
        assert!(triples.iter().any(|t| t.id == "t2" && t.id1 == "c" && t.id2 == "d"));
    }

    #[test]
    fn get_ids_from_info_table_returns_only_triple_ids() {
        let space_id = format!("ig_tr_test_facade_ids_{}", std::process::id());
        let mut facade = new_facade(&space_id);
        facade.clear_infotable("main_table".to_string());

        facade.add_to_infotable("main_table".to_string(), InfoTriple::new("t1", "a", "b"));
        facade.add_to_infotable("main_table".to_string(), InfoTriple::new("t2", "c", "d"));

        let mut ids = facade.get_ids_from_info_table("main_table");
        ids.sort();
        assert_eq!(ids, vec!["t1".to_string(), "t2".to_string()]);
    }

    #[test]
    fn select_from_info_table_filters_by_where_id() {
        let space_id = format!("ig_tr_test_facade_select_{}", std::process::id());
        let mut facade = new_facade(&space_id);
        facade.clear_infotable("main_table".to_string());

        facade.add_to_infotable("main_table".to_string(), InfoTriple::new("t1", "cat-tag", "a"));
        facade.add_to_infotable("main_table".to_string(), InfoTriple::new("t2", "cat-tag", "b"));
        facade.add_to_infotable("main_table".to_string(), InfoTriple::new("t3", "unrelated", "c"));

        let selected = facade.select_from_info_table("main_table", "cat-tag");
        let mut ids: Vec<String> = selected.iter().map(|t| t.id.clone()).collect();
        ids.sort();
        assert_eq!(ids, vec!["t1".to_string(), "t2".to_string()]);
    }

    #[test]
    fn get_all_ids_from_info_table_select_excludes_the_where_id() {
        let space_id = format!("ig_tr_test_facade_select_ids_{}", std::process::id());
        let mut facade = new_facade(&space_id);
        facade.clear_infotable("main_table".to_string());

        facade.add_to_infotable("main_table".to_string(), InfoTriple::new("t1", "cat-tag", "a"));
        facade.add_to_infotable("main_table".to_string(), InfoTriple::new("t2", "cat-tag", "b"));

        let ids = facade.get_all_ids_from_info_table_select("main_table", "cat-tag");
        assert!(ids.contains("a"));
        assert!(ids.contains("b"));
        assert!(!ids.contains("cat-tag"));
    }

    #[test]
    fn clear_infotable_empties_the_table() {
        let space_id = format!("ig_tr_test_facade_clear_{}", std::process::id());
        let mut facade = new_facade(&space_id);
        facade.add_to_infotable("main_table".to_string(), InfoTriple::new("t1", "a", "b"));

        facade.clear_infotable("main_table".to_string());

        assert!(facade.get_all_info_triples_from_info_table("main_table").is_empty());
    }
}

