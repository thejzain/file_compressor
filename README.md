
##  File Compressor (Under Development)

### Overview

This project is a file compressor under development, currently focusing on the Huffman Coding algorithm implemented in Rust. The compressor analyzes a text file, builds a Huffman tree based on character frequencies, and outputs the generated tree structure for debugging purposes. Future versions will encode the file using the code table derived from the tree.

**Please note:** This is a work-in-progress focused on Huffman coding. Future versions will include file compression and decompression capabilities.


### Building and Running
1. **Clone the repository:**
   ```bash
   git clone https://github.com/thejzain/file_compressor.git
   ```
2. **Navigate to the project directory:**
   ```bash
   cd file_compressor
   ```
3. **Build the project:**
   ```bash
   cargo build
   ```
4. **Run the compressor:**
   ```bash
   cargo run <input_file>
   ```
   Replace `<input_file>` with the path to the text file you want to analyze.

### Current Output
Currently, the compressor outputs the generated Huffman tree structure to the console.

### Future Improvements

* **Compression and Decompression:** Implement file compression and decompression functionalities using the Huffman code table.
* **File I/O Optimization:** Improve file reading and writing performance.
* **Compression Level Options:** Allow users to choose between different compression levels.
* **Error Handling:** Enhance error handling and reporting.
* **Algorithm Integration:** Explore and integrate additional compression algorithms like Lempel-Ziv (LZ) variants.
* **Combined Algorithm Support:** Develop a framework to combine different compression algorithms for potentially better results.
