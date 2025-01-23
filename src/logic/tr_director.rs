
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
