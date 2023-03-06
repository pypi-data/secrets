# PyPi Secrets Summary

Total secret types: {unique_alerts}

Total secrets: {total_alerts}

| Type | Count | Packages | Unique Secrets |
|------|-------|----------|----------------|
{{ for value in table -}}
| {value.0} | {value.1} | {value.2} | {value.3} |
{{ endfor }}