import re
from pathlib import Path

content = Path("quantwave-python/src/lib.rs").read_text()
docstring = '#[doc = "Boundary Conditions & Error Behavior:\\n- Period > Length: If a period parameter exceeds the input length, outputs will be NaN until the warmup is satisfied.\\n- NaN Inputs: NaN values in inputs propagate as NaN in the output for the duration of the rolling window.\\n- Negative Params: Negative period/length parameters will raise a ValueError."]'

new_content = re.sub(
    r'(#\[derive\(uniffi::Object\)\] pub struct \$name \{ inner: Mutex<\$core_type> \})',
    f'{docstring}\\n            \\1',
    content
)

Path("quantwave-python/src/lib.rs").write_text(new_content)
print("Updated lib.rs")
