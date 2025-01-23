use std::cmp::Ordering;


#[derive(Clone, Debug, Eq)]
#[allow(dead_code)]
pub struct InfoTriple {
    pub id: String, 
    pub id1: String, 
    pub id2: String, 
}

#[allow(dead_code)]
impl InfoTriple {
    pub fn new(id: &str, id1: &str, id2: &str) -> Self {
        InfoTriple {
            id: id.to_string(), id1: id1.to_string(), id2: id2.to_string(),
        }
    }

    pub fn to_tuple_string(&self) -> String {
        let mut tuple_string: String = String::new();
        tuple_string.push_str(&self.id);
        tuple_string.push(' ');
        tuple_string.push_str(&self.id1);
        tuple_string.push(' ');
        tuple_string.push_str(&self.id2);
        tuple_string
    }

    pub fn to_id_list(&self) -> [String;3] {
        [self.id.clone(), self.id1.clone(), self.id2.clone()]
    }

    pub fn other_half(&self, id_x: String) -> Result<String, String> {
        match id_x {
            x if x == self.id1 =>  Ok(self.id2.clone()),
            x if x == self.id2 =>  Ok(self.id1.clone()),
            _ => std::result::Result::Err(
                format!("id_x:{} is not id1:{} or id2:{} so we cannot find its other half!", id_x, self.id1, self.id2)
            ),
        }
    }
    
    pub fn is_paired_with(&self, id_x: String) -> bool {
        match id_x {
            x if x == self.id1 => true,
            x if x == self.id2 => true,
            _ => false,
        } 
    }
}

impl From<String> for InfoTriple {
    fn from(triple: String) -> Self {
        let mut fields = triple.splitn(3,' ');
        InfoTriple {
            id: fields.next().unwrap().to_string(),
            id1: fields.next().unwrap().to_string(),
            id2: fields.next().unwrap().to_string(),
        }
    }
}

impl From<(String,String,String)> for InfoTriple {
    fn from(triple: (String,String,String)) -> Self {
        InfoTriple {
            id: triple.0,
            id1: triple.1,
            id2: triple.2,
        }
    }
}


//used to be impl of InfoGraph's ToOneString trait, but left out to avoid the dependency (alt
//gather in a misc/tools lib)
impl InfoTriple {
    pub fn to_one_string (&self) -> String {
        let mut one_string: String = String::new();
        one_string.push_str(&self.id);
        one_string.push('\n');
        one_string.push_str(&self.id1);
        one_string.push('\n');
        one_string.push_str(&self.id2);
        one_string
    }
}

impl Ord for InfoTriple {
    fn cmp(&self, other: &Self) -> Ordering {
       self.id.cmp(&other.id)       
    }
}

impl PartialOrd for InfoTriple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl PartialEq for InfoTriple {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[test]
fn to_one_string_test(){
    let triple = InfoTriple {
        id: "line1 - id".to_string(),
        id1: "line2 - id1".to_string(),
        id2: "line3 - id2".to_string(),
    };
    let ideal: String = String::from(
"line1 - id
line2 - id1
line3 - id2");
print!("{}", triple.to_one_string());
   assert_eq!(triple.to_one_string(), ideal); 
}
