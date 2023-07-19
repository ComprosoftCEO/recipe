{{- $recipesFolder := .recipes_folder -}}
# Recipes List

- [Alphabetical](#alphabetical)
- [By Tag](#by-tag)
{{- if (not .skip_sql)}}
- [SQL Dump]({{.dump_sql_filename}})

To restore the SQL dump into a new database, run:

```bash
sqlite3 recipes.db ".read recipes.sql"
```
{{- end}}

## Alphabetical
{{range $_, $recipe := .recipes}}
- [{{$recipe.recipe.name}}]({{$recipesFolder}}/{{$recipe.filename}})
{{- end}}

## By Tag
{{range $_, $tag := .tags}}
### {{$tag.tag.name}}

{{$tag.tag.description_markdown}}
{{range $_, $recipe := $tag.recipes}}
- [{{$recipe.recipe.name}}]({{$recipesFolder}}/{{$recipe.filename}})
{{- end}}
{{end}}