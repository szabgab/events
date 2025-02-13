{{title}}

{{text}}

| Title | UTC | EST | PST | NZL |
| ------| --- | --- | --- | --- |
{% for event in events -%}
| [{{event.title}}]({{event.url}}) | {{event.utc}} | {{event.est}} | {{event.pst}} | {{event.nzl}} |
{% endfor %}


Once an event is online it remove the physical limitation of who can participate. What remain are the time-zone differences and the language barrier. In order to make it easier for you to find an events that are in thos constraints I started to collect the [online event](https://events.code-maven.com/) where you can filter by topic and time. Above I took the events and included the starting time in a few selected time-zones. I hope it makes it easier to find an event that is relevan to you. The data and the code generating the pages are all on [GitHub](https://github.com/szabgab/events/).

