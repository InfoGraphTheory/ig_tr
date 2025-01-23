use std::{fs, path::Path};
use super::triple_store::TripleStore; 
use ig_tools::file_tools;

#[derive(Clone)]
pub struct TripleStoreFS {
    space_folder: String,
    table_folder_name: String,
    org_space: String,
    tmp_space: Option<String>,
}

impl TripleStoreFS {
    pub fn new(space_id: String) -> Self {
        let table_folder = "infospace/files/info_tables/";
        let space_folder = "infospace/files/spaces/";
        let table_folder_name = "info_tables";
        
        let _ = fs::create_dir_all(table_folder);
        let _ = fs::create_dir_all(space_folder);
        TripleStoreFS {
            space_folder: space_folder.to_string(),
            table_folder_name: table_folder_name.to_string(),
            org_space: space_id.clone(),
            tmp_space: Option::None,
        }
    }

    pub fn create_tablefile_if_not_there(&mut self, table_name: String) {
        let mut space_dir = self.space_folder.clone();
        space_dir.push_str(&self.get_space_id());
        space_dir.push('/');
        space_dir.push_str(&self.table_folder_name);
        let _ = fs::create_dir_all(space_dir);

        let path = self.get_table_path(table_name);

        if !Path::new(&path).is_file() {
            println!("Creating file {}", path);
            let _ = fs::write(path, "");
        }
    }
   
    pub fn get_table_path(&mut self, table_name: String) -> String {
        
        let mut path = self.space_folder.clone();
        path.push_str(self.get_space_id().trim());
        path.push('/');
        path.push_str(&self.table_folder_name.clone());
        path.push('/');
        path.push_str(table_name.trim());
        path
    }

}

impl TripleStore for TripleStoreFS {

    //temporary space handling begin
    fn set_tmp_space_id(&mut self, space_id: String) {
       self.tmp_space = Option::Some(space_id);
    }

    fn revert_space_id(&mut self) {
        self.tmp_space = Option::Some(self.org_space.clone());
    }

    fn get_space_id(&mut self) -> String {
        if self.tmp_space.is_none() || self.org_space == self.tmp_space.clone().unwrap_or("".to_string()) {
            return self.org_space.clone();
        }
        self.tmp_space.clone().unwrap()
    }
    //temporary space handling end


    fn append_info_table(&mut self, infotable_name: &str, infotriple: &str) {
        
        //create if not exists
        self.create_tablefile_if_not_there(infotable_name.to_string());
        file_tools::append_ln_to_file(infotriple, &self.get_table_path(infotable_name.to_string()));
    }

    fn get_info_table(&mut self, infotable_name: &str) -> String {
        self.create_tablefile_if_not_there(infotable_name.to_string());

        fs::read_to_string(
        self.get_table_path(infotable_name.to_string()))
        .expect("something wetn wrong reading info table text file")
    }

    fn select_from_info_table(&mut self, infotable_name: &str, where_id: &str) -> String {
        
        self.create_tablefile_if_not_there(infotable_name.to_string());

        fs::read_to_string(self.get_table_path(infotable_name.to_string()))    
            .expect("something wetn wrong reading info table text file")
            .lines()
            .filter(|x| { 
                #[cfg(test)]
                println!("new{}",x);
                x.split_once(' ')
                .unwrap_or(("",""))
                .1
                .contains(where_id)
            })
            .fold("".to_string(),
                |acc, y|
                format!( "{}{}{}", acc, y, "\n"))
    }

    fn clear_infotable(&mut self, infotable_name: String) {

        let path = self.get_table_path(infotable_name);
        if Path::new(&path).is_file() {
            println!("Clearing file {}", path);
            let _ = fs::write(path, "");
        } else {
            println!("Cannot find file to clear {}", path);
        }
    }
}

//TODO: some day figure out a general way to setup test data files to ensure sameness and/or independence of test
//data files.
#[test]
fn select_from_space_info_table_test() {
    
    let infotable_name = String::from("main_table");
    let where_id = String::from("5fe9374c5e9e27ebb78f8bf7cd78bbb23ee51e672dc54c603ec1c5b3eef33feb");
    
    let mut ts = TripleStoreFS::new("84468de1424085f127ebd262b1d25df0c8196077ebfe76bb02c9deaf09687dee".to_string());
    let result_fn = ts.select_from_info_table(&infotable_name, &where_id);
    let mut result_goal: String = 
"cfe94de90878b723efdb311090465f2158798c3e9c21e82ac1582190290f756e 38a24bbddca56373b49d0452f5e485c11048c729d9230beb2fa41044a36e9791 5fe9374c5e9e27ebb78f8bf7cd78bbb23ee51e672dc54c603ec1c5b3eef33feb".to_string();
    result_goal.push('\n');
    result_goal.push_str("004580ab93a9c16797e1a6ede5b0ab1949c253fb9fb7374d5e6088adf9b18f2d 2bac5b3da5fb1ecde9e3b433a91515e2efbc908010f461d0fc9014edd74b29f4 5fe9374c5e9e27ebb78f8bf7cd78bbb23ee51e672dc54c603ec1c5b3eef33feb");
    result_goal.push('\n');
    result_goal.push_str("aa7a71807a173ef427c94f84cc78aa3e3b636841fbaee18c329682f9e2939185 5fe9374c5e9e27ebb78f8bf7cd78bbb23ee51e672dc54c603ec1c5b3eef33feb 687a862f09e4721d70108ae9e2dd7c595ade9f40e9297a29f479073cf97301a2");
    result_goal.push('\n');
//    println!("result_fn:{}", result_fn);
//    println!("result_goal:{}", result_goal);
     assert_eq!(result_fn,result_goal);   
}


