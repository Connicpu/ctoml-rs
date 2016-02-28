#![allow(non_snake_case)]
#![allow(no_mangle_generic_items)]
#![feature(box_syntax)]

extern crate toml;
extern crate bson;
extern crate chrono;
extern crate serde_json as json;

use std::mem;
use std::collections::BTreeMap;

pub type TomlValue = toml::Value;

type JsonTable = BTreeMap<String, json::Value>;
type JsonArray = Vec<json::Value>;

#[repr(C)]
pub enum TomlType {
    String,
    Integer,
    Float,
    Boolean,
    Datetime,
    Array,
    Table,
}

////////////////////////////////////////////////////////////////////
// Value functions

#[no_mangle]
pub extern "C" fn ValueType(value: &TomlValue) -> TomlType {
    match value {
        &toml::Value::String(_) => TomlType::String,
        &toml::Value::Integer(_) => TomlType::Integer,
        &toml::Value::Float(_) => TomlType::Float,
        &toml::Value::Boolean(_) => TomlType::Boolean,
        &toml::Value::Datetime(_) => TomlType::Datetime,
        &toml::Value::Array(_) => TomlType::Array,
        &toml::Value::Table(_) => TomlType::Table,
    }
}

#[no_mangle]
pub extern "C" fn ValueLookup<'a>(value: &'a TomlValue, key: &'a str) -> Option<&'a TomlValue> {
    value.lookup(key)
}

#[no_mangle]
pub extern "C" fn FreeValue(_: Option<Box<TomlValue>>) {
    // let it drop
}

#[no_mangle]
pub extern "C" fn CreateTable() -> Box<TomlValue> {
    box toml::Value::Table(toml::Table::new())
}

#[no_mangle]
pub extern "C" fn ValueTable(value: &TomlValue) -> Option<&toml::Table> {
    match value {
        &toml::Value::Table(ref tbl) => Some(tbl),
        _ => None,
    }
}

#[no_mangle]
pub extern "C" fn ValueTableMut(value: &mut TomlValue) -> Option<&mut toml::Table> {
    match value {
        &mut toml::Value::Table(ref mut tbl) => Some(tbl),
        _ => None,
    }
}

#[no_mangle]
pub extern "C" fn CreateArray() -> Box<TomlValue> {
    box toml::Value::Array(toml::Array::new())
}

#[no_mangle]
pub extern "C" fn ValueArray(value: &TomlValue) -> Option<&toml::Array> {
    match value {
        &toml::Value::Array(ref ary) => Some(ary),
        _ => None,
    }
}

#[no_mangle]
pub extern "C" fn ValueArrayMut(value: &mut TomlValue) -> Option<&mut toml::Array> {
    match value {
        &mut toml::Value::Array(ref mut ary) => Some(ary),
        _ => None,
    }
}

#[no_mangle]
pub extern "C" fn CreateString(input: &str) -> Box<TomlValue> {
    box toml::Value::String(String::from(input))
}

#[no_mangle]
pub extern "C" fn ValueString(value: &TomlValue) -> &str {
    match value {
        &toml::Value::String(ref string) => &string,
        _ => "",
    }
}

#[no_mangle]
pub extern "C" fn CreateInteger(i: i64) -> Box<TomlValue> {
    box toml::Value::Integer(i)
}

#[no_mangle]
pub extern "C" fn ValueInteger(value: &TomlValue) -> i64 {
    match value {
        &toml::Value::Integer(i) => i,
        _ => 0,
    }
}

#[no_mangle]
pub extern "C" fn CreateFloat(f: f64) -> Box<TomlValue> {
    box toml::Value::Float(f)
}

#[no_mangle]
pub extern "C" fn ValueFloat(value: &TomlValue) -> f64 {
    match value {
        &toml::Value::Float(f) => f,
        _ => 0.0,
    }
}

#[no_mangle]
pub extern "C" fn CreateBoolean(b: bool) -> Box<TomlValue> {
    box toml::Value::Boolean(b)
}

#[no_mangle]
pub extern "C" fn ValueBoolean(value: &TomlValue) -> bool {
    match value {
        &toml::Value::Boolean(b) => b,
        _ => false,
    }
}

#[no_mangle]
pub extern "C" fn CreateDatetime(input: &str) -> Box<TomlValue> {
    box toml::Value::Datetime(String::from(input))
}

#[no_mangle]
pub extern "C" fn ValueDatetime(value: &TomlValue) -> &str {
    match value {
        &toml::Value::Datetime(ref dt) => &dt,
        _ => "",
    }
}

////////////////////////////////////////////////////////////////////
// Table functions

#[no_mangle]
pub extern "C" fn WrapTable(mut table: Box<toml::Table>) -> Box<TomlValue> {
    box toml::Value::Table(mem::replace::<toml::Table>(&mut table, toml::Table::new()))
}

#[no_mangle]
pub extern "C" fn ParseTable(input: &str, errors: Option<&mut Option<Box<TomlValue>>>) -> Option<Box<toml::Table>> {
    let mut parser = toml::Parser::new(input);
    match parser.parse() {
        Some(table) => Some(Box::new(table)),
        None => {
            if let Some(errors) = errors {
                let mut list = Vec::new();
                for err in parser.errors.iter() {
                    let pos = parser.to_linecol(err.lo);
                    list.push(format!("Line {}, Col {}: {}", pos.0, pos.1, err.desc));
                }
                *errors = Some(box toml::Value::String(format!("Parse errors: {:?}", &list)));
            }
            None
        },
    }
}

#[no_mangle]
pub extern "C" fn SerializeTable(input: &toml::Table) -> Box<TomlValue> {
    box toml::Value::String(format!("{}", toml::Value::Table(input.clone())))
}

#[no_mangle]
pub extern "C" fn FreeTable(_: Option<Box<toml::Table>>) {
    // implicit drop
}

#[no_mangle]
pub extern "C" fn TableKeyCount(table: &toml::Table) -> usize {
    table.len()
}

#[no_mangle]
pub extern "C" fn TableKeys<'a>(table: &'a toml::Table, keys: &mut [&'a str]) {
    for (s, t) in table.keys().zip(keys.iter_mut()) {
        *t = s;
    }
}

#[no_mangle]
pub extern "C" fn TableGet<'a, 'b>(table: &'a toml::Table, key: &'b str) -> Option<&'a TomlValue> {
    table.get(key)
}

#[no_mangle]
pub extern "C" fn TableGetMut<'a, 'b>(table: &'a mut toml::Table, key: &'b str) -> Option<&'a mut TomlValue> {
    table.get_mut(key)
}

#[no_mangle]
pub extern "C" fn TableSet(table: &mut toml::Table, key: &str, mut value: Box<TomlValue>) {
    let inner_value = mem::replace::<TomlValue>(&mut value, toml::Value::Boolean(false));
    table.insert(String::from(key), inner_value);
}

#[no_mangle]
pub extern "C" fn TableRemove(table: &mut toml::Table, key: &str) {
    table.remove(key);
}

////////////////////////////////////////////////////////////////////
// Array functions

#[no_mangle]
pub extern "C" fn ArrayCount(array: &toml::Array) -> usize {
    array.len()
}

#[no_mangle]
pub extern "C" fn ArrayGet(array: &toml::Array, index: usize) -> &TomlValue {
    &array[index]
}

#[no_mangle]
pub extern "C" fn ArrayGetMut(array: &mut toml::Array, index: usize) -> &mut TomlValue {
    &mut array[index]
}

#[no_mangle]
pub extern "C" fn ArrayPush(array: &mut toml::Array, mut value: Box<TomlValue>) {
    let inner_value = mem::replace::<TomlValue>(&mut value, toml::Value::Boolean(false));
    array.push(inner_value);
}

#[no_mangle]
pub extern "C" fn ArrayPop(array: &mut toml::Array) {
    array.pop();
}

//////////////////////////////////////////////////////
// JSON Functions

fn table_to_json(table: &toml::Table) -> JsonTable {
    let mut json = JsonTable::new();
    for (key, value) in table {
        json.insert(key.clone(), toml_to_json(value));
    }
    json
}

fn array_to_json(array: &toml::Array) -> JsonArray {
    let mut json = JsonArray::new();
    
    for value in array {
        json.push(toml_to_json(value));
    }
    
    json
}

fn toml_to_json(value: &toml::Value) -> json::Value {
    use toml::Value::*;
    use json::Value;
    
    match *value {
        String(ref s) => Value::String(s.clone()),
        Integer(i) => Value::I64(i),
        Float(f) => Value::F64(f),
        Boolean(b) => Value::Bool(b),
        Datetime(ref d) => Value::String(d.clone()),
        Array(ref ary) => Value::Array(array_to_json(ary)),
        Table(ref table) => Value::Object(table_to_json(table)),
    }
}

fn json_to_table(json: &JsonTable) -> toml::Table {
    let mut table = toml::Table::new();
    
    for (key, value) in json {
        match json_to_toml(value) {
            Some(value) => { table.insert(key.clone(), value); }
            None => {}
        }
    }
    
    table
}

fn json_to_array(json: &JsonArray) -> toml::Array {
    let mut ary = toml::Array::new();
    
    for value in json {
        match json_to_toml(value) {
            Some(value) => { ary.push(value); }
            None => {}
        }
    }
    
    ary
}

fn json_to_toml(json: &json::Value) -> Option<toml::Value> {
    use json::Value::*;
    use toml::Value;
    
    match *json {
        Null => None,
        Bool(b) => Some(Value::Boolean(b)),
        I64(i) => Some(Value::Integer(i)),
        U64(u) => if u < std::i64::MAX as u64 {
            Some(Value::Integer(u as i64))
        } else {
            Some(Value::Float(u as f64))
        },
        F64(f) => Some(Value::Float(f)),
        String(ref s) => Some(Value::String(s.clone())),
        Array(ref json) => Some(Value::Array(json_to_array(json))),
        Object(ref json) => Some(Value::Table(json_to_table(json))),
    }
}

#[no_mangle]
pub extern "C" fn ParseTableJSON(input: &[u8], errors: Option<&mut Option<Box<TomlValue>>>) -> Option<Box<toml::Table>> {
    match json::de::from_slice::<json::Value>(input) {
        Ok(json) => match json_to_toml(&json) {
            Some(toml::Value::Table(table)) => Some(box table),
            _ => {
                if let Some(errors) = errors {
                    let error = "Json document was not an object at its root";
                    *errors = Some(box toml::Value::String(error.into()));
                }
                None
            },
        },
        Err(err) => {
            if let Some(errors) = errors {
                *errors = Some(box toml::Value::String(format!("{}", err)));
            }
            None
        }
    }
}

#[no_mangle]
pub extern "C" fn SerializeTableJSON(table: &toml::Table) -> Option<Box<[u8]>> {
    let json = json::Value::Object(table_to_json(table));
    
    match json::ser::to_vec(&json) {
        Ok(data) => Some(data.into_boxed_slice()),
        Err(_) => None,
    }
}

#[no_mangle]
pub extern "C" fn FreeJSONData(_: Option<Box<[u8]>>) {
    // Let it die
}

//////////////////////////////////////////////////////
// CSON Functions

//////////////////////////////////////////////////////
// BSON Functions

fn table_to_bson(table: &toml::Table) -> bson::Document {
    let mut doc = bson::Document::new();

    for (k, v) in table {
        doc.insert(k.clone(), toml_to_bson(v));
    }

    doc
}

fn array_to_bson(array: &toml::Array) -> bson::Array {
    let mut ary = bson::Array::new();

    for v in array {
        ary.push(toml_to_bson(v));
    }

    ary
}

fn toml_to_bson(value: &toml::Value) -> bson::Bson {
    use toml::Value::*;
    use bson::Bson;
    use std::str::FromStr;

    match value {
        &String(ref s) => Bson::String(s.clone()),
        &Integer(i) => Bson::I64(i),
        &Float(f) => Bson::FloatingPoint(f),
        &Boolean(b) => Bson::Boolean(b),
        &Datetime(ref s) => match FromStr::from_str(s) {
            Ok(dt) => Bson::UtcDatetime(dt),
            _ => Bson::Null
        },
        &Array(ref a) => Bson::Array(array_to_bson(a)),
        &Table(ref t) => Bson::Document(table_to_bson(t)),
    }
}

fn bson_to_table(table: &bson::Document) -> toml::Table {
    let mut tab = toml::Table::new();

    for (k, v) in table.iter() {
        match bson_to_toml(v) {
            Some(v) => { tab.insert(k.clone(), v); },
            None => {}
        }
    }

    tab
}

fn bson_to_array(array: &bson::Array) -> toml::Array {
    let mut ary = toml::Array::new();

    for v in array.iter() {
        match bson_to_toml(v) {
            Some(v) => { ary.push(v); },
            None => {}
        }
    }

    ary
}

fn bson_to_toml(value: &bson::Bson) -> Option<toml::Value> {
    use bson::Bson::*;

    match value {
        &Document(ref doc) => Some(toml::Value::Table(bson_to_table(doc))),
        &Array(ref ary) => Some(toml::Value::Array(bson_to_array(ary))),

        &String(ref s) => Some(toml::Value::String(s.clone())),
        &UtcDatetime(date) => Some(toml::Value::Datetime(format!("{}", date))),

        &FloatingPoint(f) => Some(toml::Value::Float(f)),
        &I32(i) => Some(toml::Value::Integer(i as i64)),
        &I64(i) => Some(toml::Value::Integer(i)),
        &TimeStamp(i) => Some(toml::Value::Integer(i)),
        &Boolean(b) => Some(toml::Value::Boolean(b)),

        _ => None
    }
}

#[no_mangle]
pub extern "C" fn ParseTableBSON(input: &[u8], errors: Option<&mut Option<Box<TomlValue>>>) -> Option<Box<toml::Table>> {
    use std::io::Cursor;

    match bson::decode_document(&mut Cursor::new(input)) {
        Ok(data) => Some(box bson_to_table(&data)),
        Err(err) => {
            if let Some(errors) = errors {
                *errors = Some(box toml::Value::String(format!("{}", err)));
            }
            None
        }
    }
}

#[no_mangle]
pub extern "C" fn SerializeTableBSON(table: &toml::Table) -> Option<Box<[u8]>> {
    let doc = table_to_bson(table);

    let mut buf = Vec::new();
    match bson::encode_document(&mut buf, &doc) {
        Ok(_) => Some(buf.into_boxed_slice()),
        Err(_) => None,
    }
}

#[no_mangle]
pub extern "C" fn FreeBSONData(_: Option<Box<[u8]>>) {
    // Let it die
}

