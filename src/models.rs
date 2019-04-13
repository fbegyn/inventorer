use super::schema::items;

#[derive(Queryable)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub team: String,
    pub amount: Option<i32>,
    pub barcode: Option<i32>,
    pub for_rent: bool,
}

#[derive(Insertable)]
#[table_name="items"]
pub struct NewItem<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub team: Option<&'a str>,
    pub amount: Option<i32>,
    pub barcode: Option<i32>,
}
