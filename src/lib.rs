#![allow(non_snake_case)]
#![feature(box_syntax)]

extern crate toml;

use std::mem;

pub type TomlValue = toml::Value;

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

