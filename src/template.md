# PyPi Secrets Summary

Total secret types: {unique_alerts}

Total secrets: {total_alerts}

| Type | Count | Unique Secrets |
|------|-------|----------------|
{{ for value in table -}}
| {value.0} | {value.1} | {value.2} |
{{ endfor }}