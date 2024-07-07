require './my_hash_table'

ht = HashTable.new(10)
ht.insert("key1", "value1")
puts ht.search("key1")  # => "value1"
ht.delete("key1")
puts ht.search("key1")  # => nil

