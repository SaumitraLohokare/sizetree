# **Sizetree** 🌳  
A Fast, Recursive Tool to Visualize Disk Space

**Sizetree** is a command-line utility that lets you quickly explore the size of files and directories in a given path. It’s perfect for anyone who wants to understand their disk usage and easily spot large files or directories. With recursive scanning, custom depth, and an option to show only directories, **Sizetree** makes managing disk space a breeze.

## 🛠 **Installation**

Clone the repository:
```bash
git clone git@github.com:SaumitraLohokare/sizetree.git
```

Install using cargo:
```bash
cargo install --path .
```

## 📝 **Usage**

Once installed, you can use Sizetree with the following command syntax:

```bash
sizetree [OPTIONS] [PATH]...
```

### Options:
```bash
--only-dir: Prints only directories, ignoring files.
--depth <DEPTH>: Sets the recursion depth (default: MAX). You can specify a number to limit how deep the tree will scan.
[PATH]...: One or more paths to scan for file sizes. If no paths are provided, Sizetree will scan the current directory by default.
```

### Example Commands:

Scan the current directory:
```bash
sizetree
```

Scan a specific directory (/path/to/dir) with a depth limit of 2:
```bash
sizetree --depth 2 /path/to/dir
```

Show only directories and omit files:
```bash
sizetree --only-dir /path/to/dir
```

Scan multiple directories with a specified depth:
```bash
sizetree --depth 3 /path/to/dir1 /path/to/dir2
```

## 💡 **Example Output**

```bash
─┬ ./
 ├─┬ .git/
 │ ╰ Total: 26.29 KB (".\\.git")
 ├─ .gitignore [8 B]
 ├─ Cargo.lock [152 B]
 ├─ Cargo.toml [79 B]
 ├─ README.md [1.50 KB]
 ├─┬ src/
 │ ╰ Total: 5.35 KB (".\\src")
 ├─┬ target/
 │ ╰ Total: 11.31 MB (".\\target")
 ╰ Total: 11.34 MB (".")
```

## ⭐️ **Support & Contributing**

If you like Sizetree, please consider starring the repo!
Feel free to open issues, contribute code, or suggest new features. Here's how you can help:

- Report Bugs: If you encounter any issues, please create an issue.
- Contribute: Fork the repo, create a branch, and submit a pull request.
- Star the Repo: If you found this project helpful, give it a ⭐️!

---

Now go ahead and take control of your disk space like a pro! 🚀🌳