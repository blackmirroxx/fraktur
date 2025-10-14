# Fraktur Translator

A Rust CLI tool that converts standard Latin text into **Mathematical Fraktur** characters, with support for common special characters (ä, ö, ü, ß, å, etc.) and their approximations.

---

## Features

- Converts Latin letters to their Fraktur equivalents.
- Approximates special characters (e.g., `ä` → `𝔞𝔢`, `ß` → `𝔰𝔷`).
- Preserves unsupported characters as-is.
- Works with pipes for seamless integration into workflows.

---

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- A terminal/editor with **UTF-8** and **Mathematical Alphanumeric Symbols** support (e.g., DejaVu Sans, STIX, or Cambria Math).

---

## Build Instructions

1. **Clone the repository:**
   ```bash
   git clone https://github.com/yourusername/fraktur_translator.git
   cd fraktur_translator
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

---

## Usage

### **Pipe Input from a File**
```bash
cat source.txt | ./target/release/fraktur_translator >> destination.txt
```

### **Direct Input**
```bash
echo "Hello äöüßå" | ./target/release/fraktur_translator
# Output: 𝔥𝔢𝔩𝔩𝔬 𝔞𝔢𝔬𝔢𝔲𝔢𝔰𝔷𝔞𝔞
```

### **Example**
**Input:**
```
Artthur Schopenhauer: „Dem intellektuell hochstehenden Menschen gewährt zunächst der Anblick der Einsamkeit einen zweifachen Vortheil: erstlich den, mit sich selbst allein zu seyn, und zweitens den, nicht mit Anderen zu seyn.“
```
**Output:**
```
𝔄𝔯𝔱𝔥𝔲𝔯 𝔖𝔠𝔥𝔬𝔭𝔢𝔫𝔥𝔞𝔲𝔢𝔯: „𝔇𝔢𝔪 𝔦𝔫𝔱𝔢𝔩𝔩𝔢𝔨𝔱𝔲𝔢𝔩𝔩 𝔥𝔬𝔠𝔥𝔰𝔱𝔢𝔥𝔢𝔫𝔡𝔢𝔫 𝔐𝔢𝔫𝔰𝔠𝔥𝔢𝔫 𝔤𝔢𝔴𝔞𝔢𝔥𝔯𝔱 𝔫𝔞𝔢𝔪𝔩𝔦𝔠𝔥 𝔡𝔦𝔢 𝔈𝔦𝔫𝔰𝔞𝔪𝔨𝔢𝔦𝔱 𝔢𝔦𝔫𝔢𝔫 𝔷𝔴𝔢𝔦𝔣𝔞𝔠𝔥𝔢𝔫 𝔙𝔬𝔯𝔱𝔥𝔢𝔦𝔩: 𝔢𝔯𝔰𝔱𝔩𝔦𝔠𝔥 𝔡𝔢𝔫, 𝔪𝔦𝔱 𝔰𝔦𝔠𝔥 𝔰𝔢𝔩𝔟𝔢𝔯 𝔷𝔲 𝔰𝔢𝔶𝔫, 𝔲𝔫𝔡 𝔷𝔴𝔢𝔦𝔱𝔢𝔫𝔰 𝔡𝔢𝔫, 𝔫𝔦𝔠𝔥𝔱 𝔪𝔦𝔱 𝔄𝔫𝔡𝔢𝔯𝔢𝔫 𝔷𝔲 𝔰𝔢𝔶𝔫.“
```

---

## Supported Characters

| Standard | Fraktur Equivalent | Unicode Code |
|----------|--------------------|--------------|
| A        | 𝔄                 | U+1D504       |
| B        | 𝔅                 | U+1D505       |
| ...      | ...                | ...          |
| a        | 𝔞                 | U+1D51E       |
| b        | 𝔟                 | U+1D51F       |
| ...      | ...                | ...          |
| ä        | 𝔞𝔢               | U+1D51E + U+1D522 |
| ö        | 𝔬𝔢               | U+1D52C + U+1D522 |
| ü        | 𝔲𝔢               | U+1D532 + U+1D522 |
| ß        | 𝔰𝔷               | U+1D530 + U+1D537 |
| å        | 𝔞𝔞               | U+1D51E + U+1D51E |

> **Note:** Not all Fraktur letters are assigned in Unicode. Unassigned letters are preserved as-is.

---

## License

This project is licensed under the **GPLv3 License** – see the [LICENSE](LICENSE) file for details.
```

---

### **Next Steps**
1. Replace `yourusername` with your GitHub username.
2. Add a `LICENSE` file with the [GPLv3 text](https://www.gnu.org/licenses/gpl-3.0.txt).
3. Optionally, add a **screenshot** of the output in the `README.md` for visual appeal.

---
