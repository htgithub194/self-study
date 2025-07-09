# Tags

{% for tag in tags %}
- [{{ tag.name }} ({{ tag.count }})]({{ tag.url }})
{% endfor %}
