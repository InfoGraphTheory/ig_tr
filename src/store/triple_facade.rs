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

