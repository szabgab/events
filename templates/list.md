{{title}}

{{text}}

| Title | UTC | EST | PST | NZL |
| ------| --- | --- | --- | --- |
{% for event in events -%}
| [{{event.title}}]({{event.url}}) | {{event.utc}} | {{event.est}} | {{event.pst}} | {{event.nzl}} |
{% endfor %}

[source](https://events.code-maven.com/)

