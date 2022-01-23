#[derive(Default)]
pub struct FieldSpecification {
    pub property_name: String,
    pub dotnet_data_type_name: String,
    pub oracle_field_name: String,
}

#[derive(Default)]
pub struct TargetClassSpecification {
    pub namespace: String,
    pub class_name: String,
    pub oracle_record_type_name: String,
    pub oracle_collection_type_name: String,
    pub debugger_display_format: String,
    pub to_string_format: String,
    pub fields: Vec<FieldSpecification>,
}
