import mkdocs_gen_files
from pathlib import Path

# This is a very basic example of what gen_python_api.py might look like.
# It iterates through the python source and creates a virtual doc file for each module.

nav = mkdocs_gen_files.Nav()

# Path to the actual python source
src = Path("quantwave-python/python/quantwave")

for path in sorted(src.rglob("*.py")):
    module_path = path.relative_to(src.parent).with_suffix("")
    doc_path = path.relative_to(src.parent).with_suffix(".md")
    full_doc_path = Path("api", doc_path)

    parts = list(module_path.parts)

    if parts[-1] == "__init__":
        parts.pop()
        doc_path = doc_path.with_name("index.md")
        full_doc_path = full_doc_path.with_name("index.md")
    elif parts[-1] == "__main__":
        continue

    nav[parts] = doc_path.as_posix()

    with mkdocs_gen_files.open(full_doc_path, "w") as fd:
        ident = ".".join(parts)
        fd.write(f"::: {ident}")

    mkdocs_gen_files.set_edit_path(full_doc_path, path)

with mkdocs_gen_files.open("api/index.md", "w") as fd:
    fd.write("# Python API Reference\n\nThis section contains the automatically generated documentation for the QuantWave Python package.")

with mkdocs_gen_files.open("api/SUMMARY.md", "w") as nav_file:
    nav_file.write("- [Overview](index.md)\n")
    nav_file.writelines(nav.build_literate_nav())
