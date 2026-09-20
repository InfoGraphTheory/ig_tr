
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
        self.clear_infotable(infotable_name);
        self.director.triples.storage.revert_space_id();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Pairing pattern: create_infotriple records a pairing between two ids, and
    // get_all_info_triples_from_info_table reads pairings back.
    #[test]
    fn create_infotriple_and_read_back_from_info_table() {
        let space_id = format!("ig_tr_test_service_create_{}", std::process::id());
        let mut service = TrServiceFS::new(space_id);
        service.clear_infotable("main_table".to_string());

        let id = "c".repeat(64);
        let cat_tag = "d".repeat(64);
        let triple = service.create_infotriple(id.clone(), cat_tag.clone());

        // create_infotriple hashes id1/id2 via ig_tools::hashing_tools::concat_n_hash, which
        // sorts the pair lexicographically before hashing - so id1/id2 on the resulting triple
        // aren't necessarily in the order passed in. Check pairing, not position.
        let triples = service.get_all_info_triples_from_info_table("main_table".to_string());
        assert!(triples.iter().any(|t| t.id == triple.id && t.is_paired_with(&id) && t.is_paired_with(&cat_tag)));
    }

    #[test]
    fn new_sets_space_id_and_org_space_id_to_the_same_value() {
        let space_id = format!("ig_tr_test_service_new_{}", std::process::id());
        let service = TrServiceFS::new(space_id.clone());
        assert_eq!(service.space_id, space_id);
        assert_eq!(service.org_space_id, space_id);
    }

    // Note: space_id/set_tmp_space_id/revert_space_id here are plain bookkeeping on TrServiceFS
    // itself - they do not affect which space storage operations actually hit. The
    // *_guest_space methods below reach into `director.triples.storage` directly instead of
    // using self.space_id. This test documents current behavior rather than a routing guarantee.
    #[test]
    fn set_tmp_space_id_and_revert_space_id_round_trip() {
        let space_id = format!("ig_tr_test_service_revert_{}", std::process::id());
        let mut service = TrServiceFS::new(space_id.clone());

        service.set_tmp_space_id("some-other-space".to_string());
        assert_eq!(service.space_id, "some-other-space");

        service.revert_space_id();
        assert_eq!(service.space_id, space_id);
    }

    #[test]
    fn guest_space_operations_do_not_leak_into_the_org_space() {
        let space_id = format!("ig_tr_test_service_guest_org_{}", std::process::id());
        let guest_space_id = format!("ig_tr_test_service_guest_other_{}", std::process::id());
        let mut service = TrServiceFS::new(space_id);
        service.clear_infotable("main_table".to_string());
        service.clear_infotable_guest_space("main_table".to_string(), guest_space_id.clone());

        service.create_infotriple_guest_space("guest-widget".to_string(), "cat-tag".to_string(), guest_space_id.clone());

        let org_triples = service.get_all_info_triples_from_info_table("main_table".to_string());
        assert!(org_triples.iter().all(|t| !t.is_paired_with("guest-widget")));

        let guest_triples = service.get_all_info_triples_from_info_table_guest_space("main_table".to_string(), guest_space_id);
        assert!(guest_triples.iter().any(|t| t.is_paired_with("guest-widget")));
    }
}
