use std::error::Error;
use std::collections::HashMap;
use std::vec::IntoIter;

use crate::InfoTriple;
    
#[allow(dead_code)]
#[derive(Debug,Clone)]
pub struct InfoTable {
    pub rows: HashMap<String, (String, String)>,
}

impl InfoTable {

    // ----------- General CRUD part begin --------------------

    ///
    /// Creates a new, empty InfoTable.
    ///
    pub fn new() -> Self {
        InfoTable {
            rows: HashMap::new(),
        }
    }

    ///
    /// Private add method used by other public add methods.
    ///
    fn add(&mut self, id: &str, id1: &str, id2: &str) -> Result<String, Box<dyn Error>>{
        if let None = self.get_info_triple(id) {
            self.rows.insert(id.to_string(), (id1.to_string(), id2.to_string()));
            Ok(String::from(id))
        } else {
            Err("A triple with that id already exists in the table!".into())
        }
    }

    ///
    /// This method adds an InfoTable to the InfoTable.
    ///
    pub fn add_triple(&mut self, triple: InfoTriple) -> Result<String, Box<dyn Error>> {
        self.add(triple.id.as_str(), triple.id1.as_str(), triple.id2.as_str())
    }

    ///
    /// This methods creates a dummy triple for an ID in the InfoTable where ID get to have a
    /// relation to itself.
    /// The use case is when we want to add an ID to the InfoTable so that we can start query it, but we have not yet
    /// used it for any actual info triples.
    ///
    pub fn add_node(&mut self, id: &str) -> Result<String, Box<dyn Error>>{
        self.add(id, id, id)
    }

    ///
    /// This method adds all elements from another InfoTable into this InfoTable.
    ///
    pub fn add_info_table(&mut self, info_table: InfoTable) {
        info_table.rows
            .iter() 
            .for_each(|(id, (id1, id2))| {
                let _ = self.add(id, id1, id2);
            });
    }

    ///
    /// Returns true if no rows (triples) are in the table.
    ///
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    ///
    /// Checks to see if a vertex within the InfoTable has an edge to the candidate vertex.
    fn has_neighbor(&self, id: String, candidate: String) -> bool {
        let mut result: bool = false;
        self.get_neighbor_ids(id.clone())
            .iter()
            .for_each(|x|{
//            println!("neighbor {} to {}",x,id.clone());
            if *x == candidate {result = true;}
        });
        result
    }


    ///
    /// Returns an Option which contains an InfoTriple corresponnding to the parameter id, if such InfoTriple
    /// exists.
    ///
    pub fn get_info_triple(&mut self, id: &str) -> Option<InfoTriple> {
        let tuple: &mut (String,String) = &mut self.rows.get_mut(id)?.clone();

        Some(InfoTriple{
            id: id.to_string(),
            id1: tuple.0.as_mut_str().to_string(),
            id2: tuple.1.as_mut().to_string(),
        })
    }

    pub fn get_info_triples(&mut self) -> Vec<InfoTriple> {

        self.rows
            .clone()
            .keys()
            .map(|id|self.get_info_triple(id).unwrap())
            .collect()
    }


    ///
    /// This function returns the list of triples as and InfoTable where ID1 or ID2 is refering to a
    /// certain ID. In other words the subset of the current InfoTable for all relations a
    /// certain ID has.
    ///
    pub fn get_neighbors_as_triples(&self, id: String) -> InfoTable {
        let mut refs = InfoTable::new();
        self
            .rows   
            .iter()
            .filter(|(_x,(y,z))|{ *y == id || *z == id })
            .for_each(|(x,(y,z))|{
                let _ = refs.add(x,y,z);
            });
        refs
    }




    ///
    /// Remove row with id as triple id from the InfoTable.
    ///
    pub fn remove(&mut self, id: &str) {
        self.rows.remove(id);
    }

    ///
    /// Remove row containing triple from the InfoTable.
    ///
    pub fn remove_triple(&mut self, triple: &InfoTriple) {
        self.remove(&triple.id)
    }

    // ----------- General CRUD part end --------------------


    // ----------- Flatten part begin --------------------

    ///
    /// Returns a Vec containing all values in all rows in the InfoTable.
    /// Example:
    /// 
    /// InfoTable
    /// ["id3 id1 id2",
    /// "id6 id4 id5",]
    ///
    /// becomes 
    /// [id3,id1,id2,id6,id4,id5]
    ///
    pub fn flatten_triples(&self) -> Vec<String> {
        self.rows
            .iter()
            .flat_map(|(k, (v0,v1))|{[k.clone(),v0.clone(),v1.clone()]})
            .collect()            
    }

    ///
    /// Like flatten_triples() but leaving out the triple ID and returns a vector of all ID1s and
    /// ID2s. That way we only return the IDs that are referring to another ID (the refs). 
    ///
    pub fn flatten_id1_y_id2_only(&self) -> Vec<String> {
        self.rows
            .iter()
            .flat_map(|(_,(v0,v1))|[v0.clone(),v1.clone()])
            .collect()            
    }

    ///
    /// Like flatten_id1_y_id2_only() but removes the value given in the parameter except.
    /// This method is useful if you have an InfoTable containing only triples referring to a specific id and you want
    /// those referring ids without the triple ids and without the referred id itself.
    ///
    pub fn flatten_id1_y_id2_only_except(&self, except: String) -> Vec<String> {
        self.flatten_id1_y_id2_only()
            .iter()
            .filter(|x|x.to_string()!=except)
            .map(|x|x.to_string())
            .collect()
    }
    
    ///
    /// Returns a Vec containing all triple ids (ID3) of an InfoTable.
    /// That is, the key of each row, leaving out ID1 and ID2.
    ///
    pub fn flatten_triples_ids_only(&self) -> Vec<String> {
        self.rows
            .keys()
            .map(|x|x.to_string())
            .collect()
    }

    // ----------- Flatten part end --------------------


    // ----------- Query VECTORS part begin --------------------

    ///
    /// This method returns the triple IDs of the relationships a reffered_id has.
    ///
    pub fn get_neighbor_triple_ids_only(&self, reffered_id: String) -> Vec<String> {
        self.get_neighbors_as_triples(reffered_id)
            .flatten_triples_ids_only()
    }
    

    /// 
    /// TODO: Method has no unit test yet!
    /// Returns a HashMap with an element for all pairings an id is part of.
    /// The paired id itself has been left out and the key is the triple_id and the value is the
    /// neighbor_id. 
    /// This method is practical when you want to use an IDs pairings and want to check for
    /// decorations for possible filters for example.
    ///
    pub fn get_neighbor_ids_and_triple_ids(&mut self, reffered_id: String) -> HashMap<String,String> {
        let mut result: HashMap<String,String> = HashMap::new();
        self.get_neighbor_triple_ids_only(reffered_id.clone())
            .iter()
            .for_each(|id3|{
                let it = self.get_info_triple(id3).unwrap();
                result.insert(id3.to_string(), it.other_half(reffered_id.clone()).unwrap());
            });
    
            result    
    }


    ///
    /// Returns a Vec containing the all the IDs that the reffered_id has been paired with so
    /// neighbors in a graph context.
    ///
    pub fn get_neighbor_ids(&self, reffered_id: String) -> Vec<String> {
        self.get_neighbors_as_triples(reffered_id.clone())
            .flatten_id1_y_id2_only_except(reffered_id)
    }

    ///
    /// Returns all refs to the specified id except for the one specified as parameter.
    /// 
    pub fn get_neighbor_ids_except(&self, id: String, except: String) -> Vec<String>{
        self.get_neighbor_ids(id)
            .iter()
            .filter(|x|{**x!=except})
            .map(|x|{x.to_string()})
            .collect()
    }


    /// 
    /// Returns all neighbors to the specified ID where the neighbors have a neighbors_neighbor_id as
    /// a neighbor.
    /// TODO: Make a new function for decorated triples (decorated_triples(id,decoration)) that returns an InfoTable where you give an ID, a
    /// neighbor_id and a
    /// decoration_id to match.
    ///
    pub fn get_neighbors_w_neighbor(&self, id: String, neighbors_neighbor_id: String) -> Vec<String>{
        self.get_neighbor_ids(id)
            .iter()
            .filter(|x|{self.has_neighbor(x.to_string(), neighbors_neighbor_id.clone())})
        //TODO::probably worng because decoration is on the triple id and hence we cannot use vecs,
        //but infotables.
            .map(|x|x.to_string())
            .collect()
    }

    pub fn get_neighbors_except_decorated_and_not(&mut self, id: String, except_decoration: String, not: String) -> InfoTable{

        self.get_neighbors_except_decorated(id, except_decoration)
            .into_iter()
            .filter(|x|{!x.is_paired_with(not.clone())})
            .collect()
    }

    ///
    /// Returns all neighbors to the specified id except for the ones where the pairing triple is having the decoration specified in
    /// the method parameter. So when talking about neighbors we are talking about IDs being paired
    /// with the ID in focus. When we talk about decorations we are talking about the neighbors of the
    /// triple IDs of the triples containing the pairings of the ID in focus.
    ///
    pub fn get_neighbors_except_decorated(&mut self, id: String, except_decoration: String) -> InfoTable{

        self.get_neighbor_triple_ids_only(id.clone())
            .iter()
            .filter(|x|{!self.clone().has_neighbor(x.to_string(), except_decoration.to_string())})
            .map(|x| {
//                println!("id {} has filtered nabour {}",id.clone(), x.clone());
                self.clone()
                    .get_info_triple(x)
                    .unwrap()
            })
            .collect()
    }


    // ----------- Query VECTORS part end --------------------


    //TODO: future method that can take into account that say canceled triples can also be canceled and
    // those can be canceled and so on, so we need to go recursive....
}

///
/// Turn the InfoTable into an iterator of InfoTriple objects.
///
impl IntoIterator for InfoTable {
    type Item = InfoTriple;
    type IntoIter = IntoIter<InfoTriple>;

    fn into_iter(mut self) -> IntoIter<InfoTriple> {
    
        let mut vec = self.get_info_triples();
        vec.sort();
        vec.into_iter()
    }
}

///
/// Constructs an InfoTable from an iterator of InfoTriple objects.
///
impl FromIterator<InfoTriple> for InfoTable {
    
    fn from_iter<T: IntoIterator<Item = InfoTriple>>(iter: T) -> Self {
        
        let mut rows: HashMap<String,(String,String)> = HashMap::new();

        iter.into_iter()
            .for_each(|t|{rows.insert(t.id, (t.id1,t.id2));});

        InfoTable{
            rows: Into::into(rows),    
        }
    }
}


impl Default for InfoTable {
    fn default() -> Self {
        InfoTable::new()            
    }        
}



// ----------- CRUD TEST part begin --------------------
#[test]
fn remove_test(){
    let mut it = InfoTable::new();
    let _ = it.add("id-a", "id1-a", "id2-a");
    let _ = it.add("id-b", "id1-b", "id2-b");

    let mut iter = it.rows.clone().into_keys().collect::<Vec<String>>();
    iter.sort();
    let mut iter = iter.iter();

    assert_eq!(
        it.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-a"), String::from("id2-a"))).as_ref()
    );
    assert_eq!(
        it.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-b"), String::from("id2-b"))).as_ref()
    );

    it.remove("id-a");

    let mut iter = it.rows.keys(); 
    assert_eq!(
        it.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-b"), String::from("id2-b"))).as_ref()
    );
    assert_eq!(
        iter.next(), 
        None
    );
}

#[test]
fn remove_triple_test(){
    let mut it = InfoTable::new();
    let _ = it.add("id-a", "id1-a", "id2-a");
    let _ = it.add("id-b", "id1-b", "id2-b");

    let mut iter = it.rows.clone().into_keys().collect::<Vec<String>>();
    iter.sort();
    let mut iter = iter.iter();

    assert_eq!(
        it.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-a"), String::from("id2-a"))).as_ref()
    );
    assert_eq!(
        it.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-b"), String::from("id2-b"))).as_ref()
    );

    let triple = InfoTriple::new("id-a", "id1-a", "id2-a");
    it.remove_triple(&triple);

    let mut iter = it.rows.keys(); 
    assert_eq!(
        it.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-b"), String::from("id2-b"))).as_ref()
    );
    assert_eq!(
        iter.next(), 
        None
    );
    
    let triple = InfoTriple::new("id-b", "id1-b", "id2-b");
    it.remove_triple(&triple);

    let mut iter = it.rows.keys(); 
    assert_eq!(
        iter.next(), 
        None
    );
}

#[test]
fn add_info_table_test() {
    let mut it1 = InfoTable::new();
    let _ = it1.add("id-a", "id1-a", "id2-a");
    let _ = it1.add("id-b", "id1-b", "id2-b");

    let mut it2 = InfoTable::new();
    let _ = it2.add("id-b", "id1-b", "id2-b");
    let _ = it2.add("id-c", "id1-c", "id2-c");
    let _ = it2.add("id-d", "id1-d", "id2-d");

    it1.add_info_table(it2);

    let mut iter = it1.rows.clone().into_keys().collect::<Vec<String>>();
    iter.sort();
    let mut iter = iter.iter();

    assert_eq!(
        it1.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-a"), String::from("id2-a"))).as_ref()
    );
    assert_eq!(
        it1.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-b"), String::from("id2-b"))).as_ref()
    );
    assert_eq!(
        it1.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-c"), String::from("id2-c"))).as_ref()
    );
    assert_eq!(
        it1.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-d"), String::from("id2-d"))).as_ref()
    );
    assert_eq!(
        iter.next(), 
        None
    );
}


   #[test] 
    pub fn get_info_triple_test() {

        let reffered_id: String = String::from("id-b"); 
        
        let mut it = InfoTable::new();
        let _ = it.add("id-a", "id1-a", "id2-a");
        let _ = it.add("id-b", "id1-b", "id2-b");
        let _ = it.add("id-c", "id1-c", "id1-b");
        let _ = it.add("id-d", "id1-d", "id2-d");

        let mut triple = it.get_info_triple(reffered_id.as_str());

        assert_eq!(
            triple.clone().unwrap().id,
            String::from("id-b")
        );
        assert_eq!(
            triple.clone().unwrap().id1,
            String::from("id1-b")
        );
        assert_eq!(
            triple.clone().unwrap().id2,
            String::from("id2-b")
        );

        let reffered_id: String = String::from("id1-b"); 
        let mut triple = it.get_info_triple(reffered_id.as_str());
        assert!(
            triple.is_none()
        );
    }

    ///
    /// This function returns the list of triples as and InfoTable where ID1 or ID2 is refering to a
    /// certain ID. In other words the subset of the current InfoTable for all relations a
    /// certain ID has.
    ///
#[test]    
pub fn get_neighbors_as_triples_test() {
    let reffered_id: String = String::from("id1-b"); 
        
    let mut it = InfoTable::new();
    let _ = it.add("id-a", "id1-a", "id2-a");
    let _ = it.add("id-b", "id1-b", "id2-b");
    let _ = it.add("id-c", "id1-c", "id1-b");
    let _ = it.add("id-d", "id1-d", "id2-d");

    let relations = it.get_neighbors_as_triples(reffered_id);
        
    let mut iter = relations.rows.clone().into_keys().collect::<Vec<String>>();
    iter.sort();
    let mut iter = iter.iter();

    assert_eq!(
        it.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-b"), String::from("id2-b"))).as_ref()
    );
    assert_eq!(
        it.rows.get(iter.next().unwrap()), 
        Some((String::from("id1-c"), String::from("id1-b"))).as_ref()
    );
    assert_eq!(
        iter.next(), 
        None
    );

    }


    // ----------- General CRUD TEST part end --------------------


    // ----------- Flatten part begin --------------------


#[test]
pub fn flatten_triples_test() {

    let mut it: InfoTable = InfoTable::new();
    let _ = it.add("id-a", "id1-a", "id2-a");
    let _ = it.add("id-b", "id1-b", "id2-b");

    let mut it = it.flatten_triples(); 
    it.sort();
    let mut it = it.iter();

    assert_eq!(
        **it.next().unwrap(), 
        String::from("id-a")
    );
    assert_eq!(
        **it.next().unwrap(), 
        String::from("id-b")
    );
    assert_eq!(
        **it.next().unwrap(), 
        String::from("id1-a")
    );
    assert_eq!(
        **it.next().unwrap(), 
        String::from("id1-b")
    );
    assert_eq!(
        **it.next().unwrap(), 
        String::from("id2-a")
    );
    assert_eq!(
        **it.next().unwrap(), 
        String::from("id2-b")
    );
    assert_eq!(
        it.next(), 
        None
    );

}

#[test]
pub fn flatten_id1_y_id2_only_test() {

    let mut it: InfoTable = InfoTable::new();
    let _ = it.add("id-a", "id1-a", "id2-a");
    let _ = it.add("id-b", "id1-b", "id2-b");

    let mut it = it.flatten_id1_y_id2_only(); 
    it.sort();
    let mut it = it.iter();

    assert_eq!(
        **it.next().unwrap(), 
        String::from("id1-a")
    );
    assert_eq!(
        **it.next().unwrap(), 
        String::from("id1-b")
    );
    assert_eq!(
        **it.next().unwrap(), 
        String::from("id2-a")
    );
    assert_eq!(
        **it.next().unwrap(), 
        String::from("id2-b")
    );
    assert_eq!(
        it.next(), 
        None
    );

}

#[test]
pub fn flatten_id1_y_id2_only_except_test() {
    let mut it: InfoTable = InfoTable::new();
    let _ = it.add("id-a", "id1-a", "id2-a");
    let _ = it.add("id-b", "id1-b", "id2-b");

    let mut it = it.flatten_id1_y_id2_only_except("id1-b".to_string()); 
    it.sort();
    let mut it = it.iter();

    assert_eq!(
        **it.next().unwrap(), 
        String::from("id1-a")
    );
    assert_eq!(
        **it.next().unwrap(), 
        String::from("id2-a")
    );
    assert_eq!(
        **it.next().unwrap(), 
        String::from("id2-b")
    );
    assert_eq!(
        it.next(), 
        None
    );

}

#[test]
pub fn flatten_triples_ids_only_test() {

    let mut it: InfoTable = InfoTable::new();
    let _ = it.add("id-a", "id1-a", "id2-a");
    let _ = it.add("id-b", "id1-b", "id2-b");

    let mut it = it.flatten_triples_ids_only(); 
    it.sort();
    let mut it = it.iter();

    assert_eq!(
        **it.next().unwrap(), 
        String::from("id-a")
    );
    assert_eq!(
        **it.next().unwrap(), 
        String::from("id-b")
    );
    assert_eq!(
        it.next(), 
        None
    );

}


    // ----------- Flatten TEST part end --------------------


    // ----------- Query Vectors TEST part begin --------------------

    ///
    /// This method returns the triple IDs of the relationships a reffered_id has.
    ///
    #[test]
    pub fn get_neighbor_triple_ids_only_test() { 
        let reffered_id: String = String::from("id1-b"); 
        
        let mut it = InfoTable::new();
        let _ = it.add("id-a", "id1-a", "id2-a");
        let _ = it.add("id-b", "id1-b", "id2-b");
        let _ = it.add("id-c", "id1-c", "id1-b");
        let _ = it.add("id-d", "id1-d", "id2-d");

        let mut triple_ids = it.get_neighbor_triple_ids_only(reffered_id);
        triple_ids.sort();
        let mut it = triple_ids.iter();

        assert_eq!(
            **it.next().unwrap(), 
            String::from("id-b")
        );
        assert_eq!(
            **it.next().unwrap(), 
            String::from("id-c")
        );
        assert_eq!(
            it.next(), 
            None
        );
        
    }
    

    //
    // Returns a Vec containing the all the IDs that the reffered_id has been paired with.
    //
    #[test]
    pub fn get_neighbor_ids_test() {

        let reffered_id: String = String::from("id1-b"); 
        
        let mut it = InfoTable::new();
        let _ = it.add("id-a", "id1-a", "id2-a");
        let _ = it.add("id-b", "id1-b", "id2-b");
        let _ = it.add("id-c", "id1-c", "id1-b");
        let _ = it.add("id-d", "id1-d", "id2-d");

        let mut triple_ids = it.get_neighbor_ids(reffered_id);
        triple_ids.sort();
        let mut it = triple_ids.iter();

        assert_eq!(
            **it.next().unwrap(), 
            String::from("id1-c")
        );
        assert_eq!(
            **it.next().unwrap(), 
            String::from("id2-b")
        );
        assert_eq!(
            it.next(), 
            None
        );

    }

    #[test]
    pub fn get_neighbor_ids_except_test() {

        let reffered_id: String = String::from("id1-b"); 
        let except_id: String = String::from("id2-b"); 
        
        let mut it = InfoTable::new();
        let _ = it.add("id-a", "id1-a", "id2-a");
        let _ = it.add("id-b", "id1-b", "id2-b");
        let _ = it.add("id-c", "id1-c", "id1-b");
        let _ = it.add("id-d", "id1-d", "id2-d");

        let mut triple_ids = it.get_neighbor_ids_except(reffered_id, except_id);
        triple_ids.sort();
        let mut it = triple_ids.iter();

        assert_eq!(
            **it.next().unwrap(), 
            String::from("id1-c")
        );
        assert_eq!(
            it.next(), 
            None
        );

    }

#[test]
fn get_neighbors_w_neighbor_test() {
    let id: String = String::from("id1-b"); 
    let neighbors_neighbor_id: String = String::from("id2-b"); 
        
    let mut it = InfoTable::new();
    let _ = it.add("id-a", "id1-a", "id2-a");
    let _ = it.add("id-b", "id1-b", "id2-b");
    let _ = it.add("id-c", "id1-c", "id1-b");
    let _ = it.add("id-d", "id1-c", "id2-b");
    let _ = it.add("id-e", "id1-d", "id2-b");
    let _ = it.add("id-f", "id1-b", "id2-a");
    let _ = it.add("id-g", "id1-d", "id2-b");
    let _ = it.add("id-h", "id2-b", "id2-d");

    let mut neighbors_w_neighbor = it.get_neighbors_w_neighbor(id, neighbors_neighbor_id);
    neighbors_w_neighbor.sort();    
    let mut iter = neighbors_w_neighbor.iter();

    assert_eq!(
        iter.next(), 
        Some(String::from("id1-c")).as_ref()
    );
    assert_eq!(
        iter.next(), 
        None
    );
}

#[test]
fn get_neighbors_except_decorated_test() {
    
    let id: String = String::from("focus_id"); 
    let except_decoration: String = String::from("exp_dec"); 
        
    let mut it = InfoTable::new();
    let _ = it.add("id-a", "id1-a", "id2-a");
    let _ = it.add("id-b", "focus_id", "exp_dec");
    let _ = it.add("id-c", "id1-c", "focus_id");
    let _ = it.add("id-d", "id-c", "exp_dec");
    let _ = it.add("id-e", "id-d", "exp_dec");
    let _ = it.add("id-f", "focus_id", "id2-a");
    let _ = it.add("id-g", "exp_dec", "id2-d");

    let mut neighbors_except_decorated = it.get_neighbors_except_decorated(id, except_decoration);
    let mut neighbors_except_decorated = neighbors_except_decorated.get_info_triples();
    neighbors_except_decorated.sort();
    let mut iter = neighbors_except_decorated.iter();

    assert_eq!(
        iter.next(), 
        Some(InfoTriple{
            id:"id-b".to_string(), id1:"focus_id".to_string(), id2:"exp_dec".to_string(),
        }).as_ref()
    );
    assert_eq!(
        iter.next(), 
        Some(InfoTriple{
            id:"id-f".to_string(), id1:"focus_id".to_string(), id2:"id2-a".to_string(),
        }).as_ref()
    );
    assert_eq!(
        iter.next(), 
        None
    );
}

    // ----------- Query Vectors TEST part end --------------------
#[test]
fn has_neighbor_test() {
    let id: String = String::from("id1-b");  
    let candidate1: String = String::from("id2-b"); 
    let candidate2: String = String::from("id1-c"); 
    let candidate3: String = String::from("id2-bb"); 

    let mut it = InfoTable::new();
    let _ = it.add("id-a", "id1-a", "id2-a");
    let _ = it.add("id-b", "id1-b", "id2-b");
    let _ = it.add("id-c", "id1-c", "id1-b");
    let _ = it.add("id-d", "id1-d", "id2-d");

    assert!(it.has_neighbor(id.clone(), candidate1));
    assert!(it.has_neighbor(id.clone(), candidate2));
    assert!(!it.has_neighbor(id.clone(), candidate3));

}

#[test]
fn get_info_triples_test() {
    let mut it = InfoTable::new();
    let _ = it.add("id-a", "id1-a", "id2-a");
    let _ = it.add("id-b", "id1-b", "id2-b");
    let mut info_triples: Vec<InfoTriple> = it.get_info_triples();
    info_triples.sort();
    let mut iter = info_triples.iter();
    
    let tr = iter.next().unwrap();
    assert_eq!(tr.id, String::from("id-a"));
    assert_eq!(tr.id1, String::from("id1-a"));
    assert_eq!(tr.id2, String::from("id2-a"));

    let tr = iter.next().unwrap();
    assert_eq!(tr.id, String::from("id-b"));
    assert_eq!(tr.id1, String::from("id1-b"));
    assert_eq!(tr.id2, String::from("id2-b"));

    assert!(iter.next().is_none());
}

