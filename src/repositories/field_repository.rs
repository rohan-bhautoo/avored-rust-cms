use crate::error::{Error, Result};
use crate::models::field_model::{CreatableFieldDataModel, CreatableFieldModel, FieldModel, UpdatableFieldDataModel, UpdatableFieldModel};
use surrealdb::dbs::Session;
use surrealdb::kvs::Datastore;

use super::into_iter_objects;

#[derive(Clone)]
pub struct FieldRepository {}

impl FieldRepository {
    pub fn new() -> Self {
        FieldRepository {}
    }

    // pub async fn paginate(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     start: i64,
    // ) -> Result<Vec<FieldModel>> {
    //     let sql = "SELECT * FROM fields LIMIT $limit START $start;";
    //     let vars = BTreeMap::from([
    //         ("limit".into(), PER_PAGE.into()),
    //         ("start".into(), start.into()),
    //     ]);
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
    //
    //     let mut field_list: Vec<FieldModel> = Vec::new();
    //
    //     for object in into_iter_objects(responses)? {
    //         let field_object = object?;
    //
    //         let field_model: Result<FieldModel> = field_object.try_into();
    //         field_list.push(field_model?);
    //     }
    //     Ok(field_list)
    // }

    pub async fn create_field(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        creatable_field_model: CreatableFieldModel,
    ) -> Result<FieldModel> {

        let mut field_data_sql = String::from("");
        // field_data_sql.push_str("[");
        let create_field_data_model: Vec<CreatableFieldDataModel> = creatable_field_model.field_data.unwrap_or_else(|| vec![]);

        for creatable_field_data in create_field_data_model {
            field_data_sql.push_str(&format!(
                    "{open_brace} \
                    label: '{label}', \
                    value: '{value}'  \
                    {close_brace}\
                    ,",
                    label = creatable_field_data.label,
                    value = creatable_field_data.value,
                    open_brace = String::from("{"),
                    close_brace = String::from("}")
                ));
        }
        // field_data_sql.pop();
        // field_data_sql.push_str("]");

        let sql = format!("
                CREATE fields CONTENT {open_brace}
                    name: '{name}',
                    identifier: '{identifier}',
                    field_type: '{field_type}',
                    field_data: [{field_data_sql}],
                    created_by: '{logged_in_user_email}',
                    updated_by: '{logged_in_user_email}',
                    created_at: time::now(),
                    updated_at: time::now(),
                {close_brace};
            ",
            name = creatable_field_model.name,
            identifier = creatable_field_model.identifier,
            field_type = creatable_field_model.field_type,
            field_data_sql = field_data_sql,
            logged_in_user_email = creatable_field_model.logged_in_username,
            open_brace = String::from("{"),
            close_brace = String::from("}")
        );

        let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("fields record can't be created".to_string())),
        };
        let field_model: Result<FieldModel> = result_object?.try_into();

        field_model
    }

    // pub async fn find_by_id(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     field_id: String,
    // ) -> Result<FieldModel> {
    //     let sql = "SELECT * FROM type::thing($table, $id);";
    //     let vars: BTreeMap<String, Value> = [
    //         ("id".into(), field_id.into()),
    //         ("table".into(), "fields".into()),
    //     ]
    //     .into();
    //
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
    //
    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found")),
    //     };
    //     let field_model: Result<FieldModel> = result_object?.try_into();
    //
    //     field_model
    // }
    //
    pub async fn update_field(
        &self,
        datastore: &Datastore,
        database_session: &Session,
        updatable_field_model: UpdatableFieldModel,
    ) -> Result<FieldModel> {
        let mut field_data_sql = String::from("");

        let update_field_data_model: Vec<UpdatableFieldDataModel> = updatable_field_model.field_data.unwrap_or_else(|| vec![]);

        for creatable_field_data in update_field_data_model {
            field_data_sql.push_str(&format!(
                "{open_brace} \
                    label: '{label}', \
                    value: '{value}'  \
                    {close_brace}\
                    ,",
                label = creatable_field_data.label,
                value = creatable_field_data.value,
                open_brace = String::from("{"),
                close_brace = String::from("}")
            ));
        }
        // field_data_sql.pop();
        // field_data_sql.push_str("]");

        let sql = format!("
                UPDATE fields:{field_id} MERGE {open_brace}
                    name: '{name}',
                    identifier: '{identifier}',
                    field_type: '{field_type}',
                    field_data: [{field_data_sql}],
                    created_by: '{logged_in_user_email}',
                    updated_by: '{logged_in_user_email}',
                    created_at: time::now(),
                    updated_at: time::now(),
                {close_brace};
            ",
            field_id = updatable_field_model.id,
            name = updatable_field_model.name,
            identifier = updatable_field_model.identifier,
            field_type = updatable_field_model.field_type,
            field_data_sql = field_data_sql,
            logged_in_user_email = updatable_field_model.logged_in_username,
            open_brace = String::from("{"),
            close_brace = String::from("}")
        );

        let responses = datastore.execute(&sql, database_session, None).await?;

        let result_object_option = into_iter_objects(responses)?.next();
        let result_object = match result_object_option {
            Some(object) => object,
            None => Err(Error::Generic("no record found".to_string())),
        };
        let field_model: Result<FieldModel> = result_object?.try_into();

        field_model
    }
    //
    // pub async fn delete_field(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    //     field_id: String,
    // ) -> Result<bool> {
    //     let sql = "
    //         DELETE type::thing($table, $id);";
    //
    //     let vars: BTreeMap<String, Value> = [
    //         ("id".into(), field_id.into()),
    //         ("table".into(), "fields".into()),
    //     ]
    //     .into();
    //
    //     let responses = datastore.execute(sql, database_session, Some(vars)).await?;
    //     let response = responses.into_iter().next().map(|rp| rp.result).transpose();
    //     if response.is_ok() {
    //         return Ok(true);
    //     }
    //
    //     Ok(false)
    // }

    // pub async fn get_total_count(
    //     &self,
    //     datastore: &Datastore,
    //     database_session: &Session,
    // ) -> Result<ModelCount> {
    //     let sql = "SELECT count() FROM fields GROUP ALL;";
    //     let responses = datastore.execute(sql, database_session, None).await?;
    //
    //     let result_object_option = into_iter_objects(responses)?.next();
    //     let result_object = match result_object_option {
    //         Some(object) => object,
    //         None => Err(Error::Generic("no record found")),
    //     };
    //
    //     let total_count = match result_object {
    //         Ok(obj) => obj.try_into(),
    //         Err(_) => Ok(ModelCount::default()),
    //     };
    //
    //     total_count
    // }
}
