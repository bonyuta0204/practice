#ifndef HASH_TABLE_H
#define HASH_TABLE_H

typedef struct Entry {
  char *key;
  char *value;
  struct Entry *next;
} Entry;

typedef struct HashTable {
  Entry **buckets;
  int size;
} HashTable;

HashTable *create_table(int size);
void free_table(HashTable *table);
void insert(HashTable *table, const char *key, const char *value);
char *search(HashTable *table, const char *key);
void delete(HashTable *table, const char *key);

#endif // HASH_TABLE_H
