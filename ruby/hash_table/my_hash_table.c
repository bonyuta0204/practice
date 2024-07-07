#include "hash_table.h"
#include "ruby.h"

static HashTable *table;

static VALUE rb_hash_table_init(VALUE self, VALUE size) {
  int c_size = NUM2INT(size);
  table = create_table(c_size);
  return self;
}

static VALUE rb_hash_table_insert(VALUE self, VALUE key, VALUE value) {
  insert(table, StringValueCStr(key), StringValueCStr(value));
  return Qnil;
}

static VALUE rb_hash_table_search(VALUE self, VALUE key) {
  char *result = search(table, StringValueCStr(key));
  if (result) {
    return rb_str_new_cstr(result);
  } else {
    return Qnil;
  }
}

static VALUE rb_hash_table_delete(VALUE self, VALUE key) {
  delete (table, StringValueCStr(key));
  return Qnil;
}

void Init_my_hash_table() {
  VALUE cHashTable = rb_define_class("HashTable", rb_cObject);
  rb_define_method(cHashTable, "initialize", rb_hash_table_init, 1);
  rb_define_method(cHashTable, "insert", rb_hash_table_insert, 2);
  rb_define_method(cHashTable, "search", rb_hash_table_search, 1);
  rb_define_method(cHashTable, "delete", rb_hash_table_delete, 1);
}
