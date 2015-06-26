#pragma once

#include <stdint.h>
#include <string>

#define CTOML_API extern "C" __declspec(dllimport)

namespace ctoml
{
    struct TomlValue;
    struct TomlTable;
    struct TomlArray;

    enum class TomlType
    {
        String,
        Integer,
        Float,
        Boolean,
        Datetime,
        Array,
        Table,
    };

    struct Buffer
    {
        char *data;
        size_t len;
    };

    struct ConstBuffer
    {
        ConstBuffer()
            : data(nullptr), len(0)
        {
        }

        ConstBuffer(const char *data, size_t len)
            : data(data), len(len)
        {
        }

        ConstBuffer(const std::string &str)
            : data(str.data()), len(str.size())
        {
        }

        const char *data;
        size_t len;
    };

    struct ConstBufferList
    {
        ConstBuffer *data;
        size_t len;
    };

    ////////////////////////////////////////////////////////////////////
    // Value functions

    // Get the type of a value
    CTOML_API TomlType ValueType(const TomlValue *value);

    // Lookup a value
    CTOML_API const TomlValue *ValueLookup(const TomlValue *value, ConstBuffer path);

    // Free a TomlValue that you own
    CTOML_API void FreeValue(TomlValue *value);

    // Create a new table, you own it.
    CTOML_API TomlValue *CreateTable();
    // Retrieve the table from a value
    CTOML_API const TomlTable *ValueTable(const TomlValue *value);
    CTOML_API TomlTable *ValueTableMut(TomlValue *value);

    // Create a new array, you own it.
    CTOML_API TomlValue *CreateArray();
    // Retrieve the table from a value
    CTOML_API const TomlArray *ValueArray(const TomlValue *value);
    CTOML_API TomlArray *ValueArrayMut(TomlValue *value);

    // Create a new string, you own it.
    CTOML_API TomlValue *CreateString(ConstBuffer input);
    // Retrieve the string value
    CTOML_API ConstBuffer ValueString(const TomlValue *value);

    // Create a new integer, you own it.
    CTOML_API TomlValue *CreateInteger(int64_t i);
    // Retrieve the integer value
    CTOML_API int64_t ValueInteger(const TomlValue *value);

    // Create a new float, you own it
    CTOML_API TomlValue *CreateFloat(double f);
    // Retrieve the float value
    CTOML_API double ValueFloat(const TomlValue *value);

    // Create a new boolean, you own it
    CTOML_API TomlValue *CreateBoolean(bool b);
    // Retrieve the bool value
    CTOML_API bool ValueBoolean(const TomlValue *value);

    // Create a new Datetime, you own it
    CTOML_API TomlValue *CreateDatetime(ConstBuffer input);
    // Retrieve the date string
    CTOML_API ConstBuffer ValueDatetime(const TomlValue *value);

    ////////////////////////////////////////////////////////////////////
    // Table functions

    // If you have an owned table, but need a value
    CTOML_API TomlValue *WrapTable(TomlTable *table);

    // Input should be UTF-8 encoded data. You own the table.
    CTOML_API TomlTable *ParseTable(ConstBuffer input);

    // Serializes to a string, get the result with ValueString, you are responsible for calling FreeValue
    CTOML_API TomlValue *SerializeTable(const TomlTable *table);

    // Free a table that you own
    CTOML_API void FreeTable(TomlTable *table);

    // Returns the number of keys in the table
    CTOML_API uint64_t TableKeyCount(const TomlTable *table);

    // Get a list of all of the keys in the table, make
    // sure you call TableKeyCount and allocate enough ConstBuffers for it.
    // Modifying the table invalidates the result, but it does not need to be freed
    CTOML_API void TableKeys(const TomlTable *table, ConstBufferList keys);

    // Get a value from the table with the given key. The table maintains
    // ownership of the value.
    CTOML_API const TomlValue *TableGet(const TomlTable *table, ConstBuffer key);
    CTOML_API TomlValue *TableGetMut(TomlTable *table, ConstBuffer key);

    // Add a value to the table at the given key. You must have ownership of value,
    // and this function transfers ownership to the table.
    CTOML_API void TableSet(TomlTable *table, ConstBuffer key, TomlValue *value);

    // Remove the key from the table
    CTOML_API void TableRemove(TomlTable *table, ConstBuffer key);

    ////////////////////////////////////////////////////////////////////
    // Array functions

    CTOML_API size_t ArrayCount(const TomlArray *array);
    CTOML_API const TomlValue *ArrayGet(const TomlArray *array, size_t index);
    CTOML_API TomlValue *ArrayGetMut(TomlArray *array, size_t index);
    CTOML_API void ArrayPush(TomlArray *toml, TomlValue *value);
    CTOML_API void ArrayPop(TomlArray *toml);
}
