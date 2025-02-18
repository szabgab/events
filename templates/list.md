{{title}}

{{text}}

| Title | UTC | EST | PST | NZL |
| ------| --- | --- | --- | --- |
{% for event in events -%}
| [{{event.title}}]({{event.url}}) | {{event.utc}} | {{event.est}} | {{event.pst}} | {{event.nzl}} |
{% endfor %}


Online events remove the physical limitation of who can participate. What remain are the time-zone differences and the language barrier. In order to make it easier for you to find events that match those constraints I started to collect the [online events](https://events.code-maven.com/) where you can filter by topic and time. Above I took the events and included the starting time in a few selected time-zones. I hope it makes it easier to find an event that is relevant to you. The data and the code generating the pages are all on [GitHub](https://github.com/szabgab/events/). Share your ideas on how to improve the listings to help you more.

