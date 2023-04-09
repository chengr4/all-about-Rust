# Ownership

- Ownership 主要就是為了管理 Heap

> Allocating on the heap: 當你要將資料放入堆積，你得要求一定大小的空間。Memory allocator 會找到一塊夠大的空位，標記為已佔用，然後回傳一個 pointer，指著該位置的位址