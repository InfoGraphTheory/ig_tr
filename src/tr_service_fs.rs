
use crate::triple_store::TripleStore;
use crate::{InfoTriple, InfoTable};
use crate::tr_director::TrDirector;
use crate::triple_facade::TripleFacade;
use crate::triple_store_fs::TripleStoreFS;

#[derive(Clone)]
pub struct TrServiceFS {
    pub director: TrDirector<TripleStoreFS>,
    pub space_id: String,
    pub org_space_id: String,
}

impl TrServiceFS {

    pub fn new(space_id: String) -> Self {

         TrServiceFS {
            director: TrDirector::new(TripleFacade{ storage: TripleStoreFS::new(space_id.clone())}),
            space_id: space_id.clone(),
            org_space_id: space_id.clone(),
        }
    }

    pub fn set_tmp_space_id(&mut self, space_id: String) {
        self.space_id = space_id;
    }

    pub fn revert_space_id(&mut self) {
        self.space_id = self.org_space_id.clone();
    }

    pub fn get_all_info_triples_from_info_table(&mut self, infotable_name: String) -> Vec<InfoTriple> {
        self.director.get_all_info_triples_from_info_table(infotable_name).clone()
    }

    pub fn get_n_flatten_all_from_table_list(&mut self, table_list: Vec<String>) -> InfoTable {
        self.director.get_n_flatten_all_from_table_list(table_list)
    }

    pub fn create_infotriple(&mut self, id1: String, id2: String) -> InfoTriple {
        self.director.create_triple(id1, id2)
    }

    pub fn clear_infotable(&mut self, infotable_name: String) {
        self.director.clear_infotable(infotable_name);
    }


    //guest functions
    pub fn get_all_info_triples_from_info_table_guest_space(&mut self, infotable_name: String, space_id: String) -> Vec<InfoTriple> {
        self.director.triples.storage.set_tmp_space_id(space_id);
        let result = self.get_all_info_triples_from_info_table(infotable_name);
        self.director.triples.storage.revert_space_id();
        result
    }

    pub fn get_all_info_triples_from_space_info_table_guest_space(&mut self, table_name: String, space_id: String) -> Vec<InfoTriple> {
        self.director.triples.storage.set_tmp_space_id(space_id);
        let result = self.get_all_info_triples_from_info_table(table_name);
        self.director.triples.storage.revert_space_id();
        result
    }

    pub fn get_n_flatten_all_from_table_list_guest_space(&mut self, table_list: Vec<String>, space_id: String) -> InfoTable {
        self.director.triples.storage.set_tmp_space_id(space_id);
        let result = self.get_n_flatten_all_from_table_list(table_list);
        self.director.triples.storage.revert_space_id();
        result
    }

    pub fn create_infotriple_guest_space(&mut self, id1: String, id2: String, space_id: String) -> InfoTriple {
        self.director.triples.storage.set_tmp_space_id(space_id);
        let result = self.create_infotriple(id1, id2);
        self.director.triples.storage.revert_space_id();
        result
    }

    pub fn clear_infotable_guest_space(&mut self, infotable_name: String, space_id: String) {
        self.director.triples.storage.set_tmp_space_id(space_id);
        let result = self.clear_infotable(infotable_name);
        self.director.triples.storage.revert_space_id();
        result
    }
}
