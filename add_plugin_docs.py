import ast
import os
import glob

docstring_to_add = """
Boundary Conditions & Error Behavior:
- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.
- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.
- Negative Params: Negative period/length parameters will raise a ValueError.
"""

def process_file(filepath):
    with open(filepath, 'r') as f:
        source = f.read()

    tree = ast.parse(source)
    modified = False

    class DocstringInjector(ast.NodeVisitor):
        def __init__(self):
            self.replacements = []

        def visit_FunctionDef(self, node):
            if node.name.startswith('_'):
                return
            
            # Check if it returns pl.Expr
            if isinstance(node.returns, ast.Attribute) and node.returns.value.id == 'pl' and node.returns.attr == 'Expr':
                # Existing docstring
                if ast.get_docstring(node):
                    old_doc = ast.get_docstring(node)
                    new_doc = old_doc + "\n" + docstring_to_add
                    # We will just do text replacement later to avoid AST unparsing complexity
                    self.replacements.append((node.name, new_doc, True))
                else:
                    self.replacements.append((node.name, docstring_to_add, False))

    visitor = DocstringInjector()
    visitor.visit(tree)

    if not visitor.replacements:
        return

    # To keep it simple, let's just do regex replacements for the function definitions
    import re
    new_source = source
    for func_name, doc_str, has_existing in visitor.replacements:
        if has_existing:
            # We replace the old docstring. This is tricky with regex. Let's find the def
            pattern = r'(def\s+' + func_name + r'\s*\(.*?\)\s*->\s*pl\.Expr:\s*)"""(.*?)"""'
            
            def replacer(match):
                prefix = match.group(1)
                old_doc = match.group(2)
                # Keep old doc and add new
                return f'{prefix}"""{old_doc}\n{docstring_to_add}"""'
                
            new_source = re.sub(pattern, replacer, new_source, flags=re.DOTALL)
        else:
            # Insert new docstring right after def
            pattern = r'(def\s+' + func_name + r'\s*\(.*?\)\s*->\s*pl\.Expr:\s*)'
            def replacer(match):
                prefix = match.group(1)
                return f'{prefix}\n        """{docstring_to_add}"""\n'
            new_source = re.sub(pattern, replacer, new_source, flags=re.DOTALL)

    if new_source != source:
        with open(filepath, 'w') as f:
            f.write(new_source)
        print(f"Updated {filepath}")

for filepath in glob.glob("quantwave-plugins/quantwave_plugins/*.py"):
    process_file(filepath)
