mod low_level_parsers;
mod specs;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{multispace0, space0, space1};
use nom::combinator::opt;
use nom::IResult;

use low_level_parsers::parse_identifier;
use nom::multi::{fold_many1, many0};
use specs::{FieldSpecification, TargetClassSpecification};

use crate::low_level_parsers::parse_string;

fn main() {
    // Make all functions appear used in VS Code.
    let _x = parse_target_specs("");
}

/// Extracts the name of the C# class.
fn parse_class_name(input: &str) -> IResult<&str, String> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("class")(input)?;
    let (input, _) = space0(input)?;
    let (input, class_name) = parse_identifier(input)?;

    Ok((input, class_name.to_string()))
}

/// Parses the Oracle record type name, e.g. "MYSCHEMA.objRecord", which immediately
/// follows the class name and is mandatory.
fn parse_oracle_record_type_name(input: &str) -> IResult<&str, String> {
    let (input, _) = space0(input)?;
    let (input, record_type_name) = parse_identifier(input)?;
    Ok((input, record_type_name.to_string()))
}

/// Parses the Oracle collection type name, e.g. "MYSCHEMA.tblRecord", which immediately
/// follows the Oracle record type name and is optional.
fn parse_oracle_collection_type_name(input: &str) -> IResult<&str, String> {
    fn parse_collection_type_name_inner(input: &str) -> IResult<&str, String> {
        let (input, _) = space0(input)?;
        let (input, collection_type_name) = parse_identifier(input)?;
        Ok((input, collection_type_name.to_string()))
    }

    let (input, ctn) = opt(parse_collection_type_name_inner)(input)?;
    Ok((input, ctn.unwrap_or_default()))
}

/// Eats whitespace, an optional comma, then more whitespace. This is used
/// to consume the whitespace at the end of lines such as the 'class' stanza
/// and takes us up to the very start of the next line (i.e. it eats all the
/// intermediate whitespace.)
fn parse_line_end_with_optional_comma(input: &str) -> IResult<&str, ()> {
    let (input, _) = multispace0(input)?;
    let (input, _) = opt(tag(","))(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, ()))
}

/// Parses the Namespace specification. This is optional, but if present occurs on a line
/// by itself after the class declaration.
fn parse_namespace(input: &str) -> IResult<&str, String> {
    fn parse_namespace_inner(input: &str) -> IResult<&str, String> {
        let (input, _) = parse_line_end_with_optional_comma(input)?;
        let (input, _) = tag("Namespace")(input)?;
        let (input, _) = space0(input)?;
        let (input, ns) = parse_identifier(input)?;
        Ok((input, ns.to_string()))
    }

    let (input, ns) = opt(parse_namespace_inner)(input)?;
    Ok((input, ns.unwrap_or_default()))
}

/// Parses the DebuggerDisplay specification. This is optional, but if present occurs on a line
/// by itself after the namespace declaration.
fn parse_debugger_display_format(input: &str) -> IResult<&str, String> {
    fn parse_debugger_display_inner(input: &str) -> IResult<&str, String> {
        let (input, _) = parse_line_end_with_optional_comma(input)?;
        let (input, _) = tag("DebuggerDisplay")(input)?;
        let (input, _) = space0(input)?;
        let (input, dd) = parse_string(input)?;
        Ok((input, dd.to_string()))
    }

    let (input, dd) = opt(parse_debugger_display_inner)(input)?;
    Ok((input, dd.unwrap_or_default()))
}

fn parse_to_string_format(input: &str) -> IResult<&str, String> {
    fn parse_to_string_inner(input: &str) -> IResult<&str, String> {
        let (input, _) = parse_line_end_with_optional_comma(input)?;
        let (input, _) = tag("ToString")(input)?;
        let (input, _) = space0(input)?;
        let (input, ts) = parse_string(input)?;
        Ok((input, ts.to_string()))
    }

    let (input, ts) = opt(parse_to_string_inner)(input)?;
    Ok((input, ts.unwrap_or_default()))
}

fn parse_field_list_start(input: &str) -> IResult<&str, ()> {
    let (input, _) = parse_line_end_with_optional_comma(input)?;
    let (input, _) = tag("Fields")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("[")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, ()))
}

fn parse_field_list_end(input: &str) -> IResult<&str, ()> {
    let (input, _) = multispace0(input)?;
    let (input, _) = tag("]")(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, ()))
}

fn parse_one_part_field(input: &str) -> IResult<&str, FieldSpecification> {
    let (input, property_name) = parse_identifier(input)?;
    let field = FieldSpecification {
        property_name: property_name.to_string(),
        ..Default::default()
    };

    Ok((input, field))
}

fn parse_two_part_field(input: &str) -> IResult<&str, FieldSpecification> {
    let (input, dotnet_data_type_name) = parse_identifier(input)?;
    let (input, _) = space1(input)?;
    let (input, property_name) = parse_identifier(input)?;

    let field = FieldSpecification {
        property_name: property_name.to_string(),
        dotnet_data_type_name: dotnet_data_type_name.to_string(),
        ..Default::default()
    };

    Ok((input, field))
}

fn parse_three_part_field(input: &str) -> IResult<&str, FieldSpecification> {
    let (input, dotnet_data_type_name) = parse_identifier(input)?;
    let (input, _) = space1(input)?;
    let (input, property_name) = parse_identifier(input)?;
    let (input, _) = space1(input)?;
    let (input, oracle_field_name) = parse_identifier(input)?;

    let field = FieldSpecification {
        property_name: property_name.to_string(),
        dotnet_data_type_name: dotnet_data_type_name.to_string(),
        oracle_field_name: oracle_field_name.to_string(),
    };

    Ok((input, field))
}

fn parse_field(input: &str) -> IResult<&str, FieldSpecification> {
    let (input, field) = alt((
        parse_three_part_field,
        parse_two_part_field,
        parse_one_part_field,
    ))(input)?;

    // Consume all the whitespace (or comma) to the beginning of the next field.
    let (input, _) = parse_line_end_with_optional_comma(input)?;

    Ok((input, field))
}

fn parse_field_list(input: &str) -> IResult<&str, Vec<FieldSpecification>> {
    let (input, _) = parse_field_list_start(input)?;

    let (input, field_list) = fold_many1(
        // Parser function to apply - parses a single field.
        parse_field,
        // Initial state - empty vector.
        Vec::new,
        // Folding function - collect the fields.
        |mut fields, field| {
            fields.push(field);
            fields
        },
    )(input)?;

    let (input, _) = parse_field_list_end(input)?;
    Ok((input, field_list))
}

fn parse_one_complete_spec(input: &str) -> IResult<&str, TargetClassSpecification> {
    let (input, class_name) = parse_class_name(input)?;
    let (input, oracle_record_type_name) = parse_oracle_record_type_name(input)?;
    let (input, oracle_collection_type_name) = parse_oracle_collection_type_name(input)?;
    let (input, namespace) = parse_namespace(input)?;
    let (input, debugger_display_format) = parse_debugger_display_format(input)?;
    let (input, to_string_format) = parse_to_string_format(input)?;
    let (input, fields) = parse_field_list(input)?;

    let spec = TargetClassSpecification {
        class_name,
        debugger_display_format,
        namespace,
        oracle_record_type_name,
        oracle_collection_type_name,
        to_string_format,
        fields,
    };

    Ok((input, spec))
}

fn parse_target_specs(input: &str) -> Vec<TargetClassSpecification> {
    let (_input, specs) = many0(parse_one_complete_spec)(input).unwrap();
    specs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn with_record_type_only() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE
            Fields [
                Dummy
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(specs[0].oracle_collection_type_name, "");
        assert_eq!(specs[0].debugger_display_format, "");
        assert_eq!(specs[0].to_string_format, "");
    }

    #[test]
    fn with_record_type_and_namespace() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE
            Namespace Some.Name.Space
            Fields [
                Dummy
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "Some.Name.Space");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(specs[0].oracle_collection_type_name, "");
        assert_eq!(specs[0].debugger_display_format, "");
        assert_eq!(specs[0].to_string_format, "");
    }

    #[test]
    fn with_record_type_and_collection_type() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE SCHEMA.COLLECTIONTYPE
            Fields [
                Dummy
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(
            specs[0].oracle_collection_type_name,
            "SCHEMA.COLLECTIONTYPE"
        );
        assert_eq!(specs[0].debugger_display_format, "");
        assert_eq!(specs[0].to_string_format, "");
    }

    #[test]
    fn with_record_type_and_collection_type_and_debug() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE SCHEMA.COLLECTIONTYPE
            DebuggerDisplay \"my_debug_spec {Sku}\"
            Fields [
                Dummy
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(
            specs[0].oracle_collection_type_name,
            "SCHEMA.COLLECTIONTYPE"
        );
        assert_eq!(specs[0].debugger_display_format, "my_debug_spec {Sku}");
        assert_eq!(specs[0].to_string_format, "");
    }

    #[test]
    fn with_record_type_only_and_namespace_and_debug_and_to_string() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE
            Namespace Some.Name.Space
            DebuggerDisplay \"my_debug_spec {Sku}\"
            ToString \"format_something\"
            Fields [
                Dummy
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "Some.Name.Space");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(specs[0].oracle_collection_type_name, "");
        assert_eq!(specs[0].debugger_display_format, "my_debug_spec {Sku}");
        assert_eq!(specs[0].to_string_format, "format_something");
    }

    #[test]
    fn with_record_type_and_namespace_and_trailing_comma() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE  ,
            Namespace Some.Name.Space   ,
            Fields [
                Dummy
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "Some.Name.Space");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(specs[0].oracle_collection_type_name, "");
        assert_eq!(specs[0].debugger_display_format, "");
        assert_eq!(specs[0].to_string_format, "");
    }

    #[test]
    fn with_record_type_only_and_debug() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE
            DebuggerDisplay \"my_debug_spec {Sku}\"
            Fields [
                Dummy
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(specs[0].oracle_collection_type_name, "");
        assert_eq!(specs[0].debugger_display_format, "my_debug_spec {Sku}");
        assert_eq!(specs[0].to_string_format, "");
    }

    #[test]
    fn with_record_type_and_collection_type_and_debugger_display_and_to_string() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE SCHEMA.COLLECTIONTYPE
            DebuggerDisplay \"my_debug_spec {Sku}\"
            ToString \"format_something\"
            Fields [
                Dummy
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(
            specs[0].oracle_collection_type_name,
            "SCHEMA.COLLECTIONTYPE"
        );
        assert_eq!(specs[0].debugger_display_format, "my_debug_spec {Sku}");
        assert_eq!(specs[0].to_string_format, "format_something");
    }

    #[test]
    fn with_record_type_and_collection_type_and_debugger_display_and_to_string_and_trailing_commas()
    {
        let input = "
        class MyClass SCHEMA.RECORDTYPE SCHEMA.COLLECTIONTYPE
            DebuggerDisplay \"my_debug_spec {Sku}\",
            ToString \"format_something\",
            Fields [
                Dummy
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(
            specs[0].oracle_collection_type_name,
            "SCHEMA.COLLECTIONTYPE"
        );
        assert_eq!(specs[0].debugger_display_format, "my_debug_spec {Sku}");
        assert_eq!(specs[0].to_string_format, "format_something");
    }

    #[test]
    fn one_simple_field() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE
            Fields [
                FirstProperty
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(specs[0].oracle_collection_type_name, "");
        assert_eq!(specs[0].debugger_display_format, "");
        assert_eq!(specs[0].to_string_format, "");
        assert_eq!(specs[0].fields.len(), 1);

        assert_eq!(specs[0].fields[0].property_name, "FirstProperty");
        assert_eq!(specs[0].fields[0].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[0].oracle_field_name, "");
    }

    #[test]
    fn three_simple_fields() {
        // Test spaces and tabs within the field list.
        let input = "
        class MyClass SCHEMA.RECORDTYPE
            Fields [
                FirstProperty
                         SecondProperty
ThirdProperty
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(specs[0].oracle_collection_type_name, "");
        assert_eq!(specs[0].debugger_display_format, "");
        assert_eq!(specs[0].to_string_format, "");
        assert_eq!(specs[0].fields.len(), 3);

        assert_eq!(specs[0].fields[0].property_name, "FirstProperty");
        assert_eq!(specs[0].fields[0].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[0].oracle_field_name, "");

        assert_eq!(specs[0].fields[1].property_name, "SecondProperty");
        assert_eq!(specs[0].fields[1].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[1].oracle_field_name, "");

        assert_eq!(specs[0].fields[2].property_name, "ThirdProperty");
        assert_eq!(specs[0].fields[2].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[2].oracle_field_name, "");
    }

    #[test]
    fn fields_are_correct() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE
            Fields [
System.Int32? NullableProp ORACLENAME
                PropertyOnly
                int PropAndType
      decimal? PropAndTypeAndOracle ORACLENAME2
                OtherPropertyOnly
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(specs[0].oracle_collection_type_name, "");
        assert_eq!(specs[0].debugger_display_format, "");
        assert_eq!(specs[0].to_string_format, "");
        assert_eq!(specs[0].fields.len(), 5);

        assert_eq!(specs[0].fields[0].property_name, "NullableProp");
        assert_eq!(specs[0].fields[0].dotnet_data_type_name, "System.Int32?");
        assert_eq!(specs[0].fields[0].oracle_field_name, "ORACLENAME");

        assert_eq!(specs[0].fields[1].property_name, "PropertyOnly");
        assert_eq!(specs[0].fields[1].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[1].oracle_field_name, "");

        assert_eq!(specs[0].fields[2].property_name, "PropAndType");
        assert_eq!(specs[0].fields[2].dotnet_data_type_name, "int");
        assert_eq!(specs[0].fields[2].oracle_field_name, "");

        assert_eq!(specs[0].fields[3].property_name, "PropAndTypeAndOracle");
        assert_eq!(specs[0].fields[3].dotnet_data_type_name, "decimal?");
        assert_eq!(specs[0].fields[3].oracle_field_name, "ORACLENAME2");

        assert_eq!(specs[0].fields[4].property_name, "OtherPropertyOnly");
        assert_eq!(specs[0].fields[4].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[4].oracle_field_name, "");
    }

    #[test]
    fn fields_are_correct_using_commas_as_field_separator() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE
            Fields [
System.Int32? NullableProp ORACLENAME ,          PropertyOnly
                int PropAndType
      decimal? PropAndTypeAndOracle ORACLENAME2,OtherPropertyOnly
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(specs[0].oracle_collection_type_name, "");
        assert_eq!(specs[0].debugger_display_format, "");
        assert_eq!(specs[0].to_string_format, "");
        assert_eq!(specs[0].fields.len(), 5);

        assert_eq!(specs[0].fields[0].property_name, "NullableProp");
        assert_eq!(specs[0].fields[0].dotnet_data_type_name, "System.Int32?");
        assert_eq!(specs[0].fields[0].oracle_field_name, "ORACLENAME");

        assert_eq!(specs[0].fields[1].property_name, "PropertyOnly");
        assert_eq!(specs[0].fields[1].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[1].oracle_field_name, "");

        assert_eq!(specs[0].fields[2].property_name, "PropAndType");
        assert_eq!(specs[0].fields[2].dotnet_data_type_name, "int");
        assert_eq!(specs[0].fields[2].oracle_field_name, "");

        assert_eq!(specs[0].fields[3].property_name, "PropAndTypeAndOracle");
        assert_eq!(specs[0].fields[3].dotnet_data_type_name, "decimal?");
        assert_eq!(specs[0].fields[3].oracle_field_name, "ORACLENAME2");

        assert_eq!(specs[0].fields[4].property_name, "OtherPropertyOnly");
        assert_eq!(specs[0].fields[4].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[4].oracle_field_name, "");
    }

    #[test]
    fn fields_are_correct_using_commas_as_field_separator_and_at_line_end() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE
            Fields [
System.Int32? NullableProp ORACLENAME    ,          PropertyOnly,
                int PropAndType   ,
      decimal? PropAndTypeAndOracle ORACLENAME2     ,OtherPropertyOnly
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 1);
        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(specs[0].oracle_collection_type_name, "");
        assert_eq!(specs[0].debugger_display_format, "");
        assert_eq!(specs[0].to_string_format, "");
        assert_eq!(specs[0].fields.len(), 5);

        assert_eq!(specs[0].fields[0].property_name, "NullableProp");
        assert_eq!(specs[0].fields[0].dotnet_data_type_name, "System.Int32?");
        assert_eq!(specs[0].fields[0].oracle_field_name, "ORACLENAME");

        assert_eq!(specs[0].fields[1].property_name, "PropertyOnly");
        assert_eq!(specs[0].fields[1].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[1].oracle_field_name, "");

        assert_eq!(specs[0].fields[2].property_name, "PropAndType");
        assert_eq!(specs[0].fields[2].dotnet_data_type_name, "int");
        assert_eq!(specs[0].fields[2].oracle_field_name, "");

        assert_eq!(specs[0].fields[3].property_name, "PropAndTypeAndOracle");
        assert_eq!(specs[0].fields[3].dotnet_data_type_name, "decimal?");
        assert_eq!(specs[0].fields[3].oracle_field_name, "ORACLENAME2");

        assert_eq!(specs[0].fields[4].property_name, "OtherPropertyOnly");
        assert_eq!(specs[0].fields[4].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[4].oracle_field_name, "");
    }

    #[test]
    fn multiple_specs_in_one_file() {
        let input = "
        class MyClass SCHEMA.RECORDTYPE
            Fields [
System.Int32? NullableProp ORACLENAME
                PropertyOnly
                int PropAndType
        decimal? PropAndTypeAndOracle ORACLENAME2
                OtherPropertyOnly
            ]

        class MyClass2 SCHEMA.RECORDTYPE2
    Namespace Some.Name.Space
        ToString \"format_something\"
            Fields [
                PropertyOnly2
                int PropAndType2
            ]
        ";

        let specs = parse_target_specs(input);
        assert_eq!(specs.len(), 2);

        assert_eq!(specs[0].namespace, "");
        assert_eq!(specs[0].class_name, "MyClass");
        assert_eq!(specs[0].oracle_record_type_name, "SCHEMA.RECORDTYPE");
        assert_eq!(specs[0].oracle_collection_type_name, "");
        assert_eq!(specs[0].debugger_display_format, "");
        assert_eq!(specs[0].to_string_format, "");
        assert_eq!(specs[0].fields.len(), 5);
        assert_eq!(specs[0].fields[0].property_name, "NullableProp");
        assert_eq!(specs[0].fields[0].dotnet_data_type_name, "System.Int32?");
        assert_eq!(specs[0].fields[0].oracle_field_name, "ORACLENAME");
        assert_eq!(specs[0].fields[1].property_name, "PropertyOnly");
        assert_eq!(specs[0].fields[1].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[1].oracle_field_name, "");
        assert_eq!(specs[0].fields[2].property_name, "PropAndType");
        assert_eq!(specs[0].fields[2].dotnet_data_type_name, "int");
        assert_eq!(specs[0].fields[2].oracle_field_name, "");
        assert_eq!(specs[0].fields[3].property_name, "PropAndTypeAndOracle");
        assert_eq!(specs[0].fields[3].dotnet_data_type_name, "decimal?");
        assert_eq!(specs[0].fields[3].oracle_field_name, "ORACLENAME2");
        assert_eq!(specs[0].fields[4].property_name, "OtherPropertyOnly");
        assert_eq!(specs[0].fields[4].dotnet_data_type_name, "");
        assert_eq!(specs[0].fields[4].oracle_field_name, "");

        assert_eq!(specs[1].namespace, "Some.Name.Space");
        assert_eq!(specs[1].class_name, "MyClass2");
        assert_eq!(specs[1].oracle_record_type_name, "SCHEMA.RECORDTYPE2");
        assert_eq!(specs[1].oracle_collection_type_name, "");
        assert_eq!(specs[1].debugger_display_format, "");
        assert_eq!(specs[1].to_string_format, "format_something");
        assert_eq!(specs[1].fields.len(), 2);
        assert_eq!(specs[1].fields[0].property_name, "PropertyOnly2");
        assert_eq!(specs[1].fields[0].dotnet_data_type_name, "");
        assert_eq!(specs[1].fields[0].oracle_field_name, "");
        assert_eq!(specs[1].fields[1].property_name, "PropAndType2");
        assert_eq!(specs[1].fields[1].dotnet_data_type_name, "int");
        assert_eq!(specs[1].fields[1].oracle_field_name, "");
    }
}
