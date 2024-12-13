# byte-stream-compression
Testing different rust compression approaches to compress a u8 vector

Miniz_oxide Compression time       1.402280375s
Miniz_oxide Decompression time     281.98ms

GZ Compression time                1.422172917s
GZ Decompression time              817.303125ms

Deflate (miniz) Compression time   1.300999s
Deflate (miniz) Decompression time 563.450875ms

Zlib Compression time              1.5308745s
Zlib Decompression time            733.060417ms
