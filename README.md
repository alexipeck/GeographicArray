GeographicArray
O(1) Insert
O(1) Retrieval given an exact value from one axis. Maybe not>
Uses a center indexed map, ie, [-65536 - 65536] meters.
Can be used to store GPS locations, as there is a full range X, Y, Z in the current implementation to your entered distance.
Works well for searching from a specific position.
