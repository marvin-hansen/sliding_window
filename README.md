# sliding_window


This sliding window implementation over-allocates to trade space (memory) for copying complexity; 
Specifically, for a sliding window of size N, the number of elements a sing windows can hold without any array copying
is approx C-1, where C is the total capacity defined as NxM with M as a multiple.   

For example, if the window size N is 7, and the multiple M is 7, then the max capacity C is 49 (7*7), 
means the sliding window can hold up to 48 elements before a rewind performs an array copy.   

 
  