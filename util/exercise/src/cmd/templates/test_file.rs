{% import "macros.rs" as macros -%}

//! Tests for {{ exercise }}
//!
//! Generated by [utility][utility] using [canonical data][canonical_data]
//!
//! [utility]: https://github.com/exercism/rust/tree/master/util/exercise
//! [canonical_data]: https://raw.githubusercontent.com/exercism/problem-specifications/master/exercises/{{ exercise }}/canonical-data.json

{% for comment in comments -%}
    /// {{ comment }}
{% endfor -%}

{% if use_maplit -%}
use maplit::hashmap;
{% endif -%}

{# Prepare an array (global) to store the properties. Also, don't ignore the first case. -#}
{% set properties = [] -%}
{% set dont_ignore = true -%}

{% for item in cases -%}
    {# Check if we're dealing with a group of cases. #}
    {% if item.cases -%}
        /// {{ item.description }}
        {% if item.optional -%}
        /// {{ item.optional }}
        {% endif -%}

        {% if item.comments -%}
            {% for comment in item.comments -%}
            /// {{ comment }}
            {% endfor -%}
        {% endif -%}

        {% for case in item.cases -%}
            {% set_global properties = properties | concat(with=case.property) -%}
            {{ macros::gen_test_fn(case=case, dont_ignore=dont_ignore) }}
            {% set_global dont_ignore = false -%}
        {% endfor -%}

    {# Or just a single one. #}
    {% else -%}
        {% set_global properties = properties | concat(with=item.property) -%}
        {{ macros::gen_test_fn(case=item, dont_ignore=dont_ignore) }}
        {% set_global dont_ignore = false -%}
    {% endif -%}
{% endfor -%}

{%- for property in properties | unique -%}
    {% include "property_fn.rs" %}
{%- endfor -%}
