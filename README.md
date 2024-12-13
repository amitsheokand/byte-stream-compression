# byte-stream-compression
Testing different rust compression approaches to compress a u8 vector


| Method                      | Compression Time | Decompression Time |
|-----------------------------|------------------|--------------------|
| Miniz_oxide                 | 1.402280375s     | 281.98ms           |
| GZ                          | 1.422172917s     | 817.303125ms       |
| Deflate (miniz)             | 1.300999s        | 563.450875ms       |
| Zlib                        | 1.5308745s       | 733.060417ms       |
