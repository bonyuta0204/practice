#include "hash_table.h"
#include <stdlib.h>
#include <string.h>

static unsigned int hash(const char *key, int size) {
  unsigned int hash = 0;
  while (*key) {
    hash = (hash << 5) + *key++;
  }
  return hash % size;
}

HashTable *create_table(int size) {
  HashTable *table = (HashTable *)malloc(sizeof(HashTable));
  table->buckets = (Entry **)calloc(size, sizeof(Entry *));
  table->size = size;
  return table;
}

void free_table(HashTable *table) {
  for (int i = 0; i < table->size; i++) {
    Entry *entry = table->buckets[i];
    while (entry) {
      Entry *next = entry->next;
      free(entry->key);
      free(entry->value);
      free(entry);
      entry = next;
    }
  }
  free(table->buckets);
  free(table);
}

void insert(HashTable *table, const char *key, const char *value) {
  unsigned int index = hash(key, table->size);
  Entry *new_entry = (Entry *)malloc(sizeof(Entry));
  new_entry->key = strdup(key);
  new_entry->value = strdup(value);
  new_entry->next = table->buckets[index];
  table->buckets[index] = new_entry;
}

char *search(HashTable *table, const char *key) {
  unsigned int index = hash(key, table->size);
  Entry *entry = table->buckets[index];
  while (entry) {
    if (strcmp(entry->key, key) == 0) {
      return entry->value;
    }
    entry = entry->next;
  }
  return NULL;
}

void delete(HashTable *table, const char *key) {
  unsigned int index = hash(key, table->size);
  Entry *entry = table->buckets[index];
  Entry *prev = NULL;
  while (entry) {
    if (strcmp(entry->key, key) == 0) {
      if (prev) {
        prev->next = entry->next;
      } else {
        table->buckets[index] = entry->next;
      }
      free(entry->key);
      free(entry->value);
      free(entry);
      return;
    }
    prev = entry;
    entry = entry->next;
  }
}
