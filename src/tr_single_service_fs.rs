
use crate::tr_service_fs::TrServiceFS;
use crate::{InfoTriple, InfoTable};

    pub fn get_all_info_triples_from_info_table(space_id: String, infotable_name: String) -> Vec<InfoTriple> {
        TrServiceFS::new(space_id).director.get_all_info_triples_from_info_table(infotable_name).clone()
    }

    pub fn get_all_info_triples_from_space_info_table(space_id: String, table_name: String) -> Vec<InfoTriple> {
        TrServiceFS::new(space_id).director.get_all_info_triples_from_info_table(table_name)
    }

    pub fn get_n_flatten_all_from_table_list(space_id: String, table_list: Vec<String>) -> InfoTable {
        TrServiceFS::new(space_id).director.get_n_flatten_all_from_table_list(table_list)
    }

    pub fn create_infotriple(space_id: String, id1: String, id2: String) {
        TrServiceFS::new(space_id).director.create_triple(id1, id2);
    }

    pub fn clear_infotable(space_id: String, infotable_name: String) {
        TrServiceFS::new(space_id).director.clear_infotable(infotable_name);
    }

