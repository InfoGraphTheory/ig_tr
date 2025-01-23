
pub trait TripleStore {

    fn append_info_table(&mut self, infotable_name: &str, infotriple: &str);

    fn get_info_table(&mut self, infotable_name: &str) -> String;

    fn select_from_info_table(&mut self, infotable_name: &str, where_id: &str) -> String;

//    fn append_space_info_table(&mut self, infotable_name: &str, infotriple: &str, space_id: String);

//    fn select_from_space_info_table(&mut self, infotable_name: &str, where_id: &str) -> String;

    fn clear_infotable(&mut self, infotable_name: String);

    fn set_tmp_space_id(&mut self, space_id: String);

    fn revert_space_id(&mut self);

    fn get_space_id(&mut self) -> String;
}
