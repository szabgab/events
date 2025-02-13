{{title}}

{{text}}

| Title | UTC | EST | PST |
| ------| --- | --- | --- |
{% for event in events -%}
| [{{event.title}}]({{event.url}}) | {{event.utc}} | {{event.est}} | {{event.pst}} |
{% endfor %}

[source](https://events.code-maven.com/)

