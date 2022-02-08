# Industrial-IO

An industrial chain app by using MongoDB as Graph Database

## Structure

```txt
    .
    ├── docs
    │   └── mongo_aggregation.md
    ├── iio-frontend
    │   ├── pages
    │   │   ├── api
    │   │   │   └── hello.ts
    │   │   ├── _app.tsx
    │   │   └── index.tsx
    │   ├── public
    │   │   ├── favicon.ico
    │   │   └── vercel.svg
    │   ├── styles
    │   │   ├── Home.module.css
    │   │   └── globals.css
    │   ├── next-env.d.ts
    │   └── next.config.js
    ├── industrial-io
    │   ├── app
    │   │   └── src
    │   │       └── main.rs
    │   ├── crud
    │   │   ├── src
    │   │   │   ├── cache.rs
    │   │   │   ├── lib.rs
    │   │   │   └── persistence.rs
    │   │   └── tests
    │   │       ├── test_crud_derive.rs
    │   │       └── test_persistence.rs
    │   ├── crud-derive
    │   │   └── src
    │   │       ├── indexes.rs
    │   │       └── lib.rs
    │   ├── domain
    │   │   ├── src
    │   │   │   ├── actions
    │   │   │   │   ├── handle_relationships.rs
    │   │   │   │   ├── maintain_companies.rs
    │   │   │   │   ├── maintain_properties.rs
    │   │   │   │   ├── mod.rs
    │   │   │   │   ├── operate_catalog.rs
    │   │   │   │   └── search_relationships.rs
    │   │   │   ├── entities
    │   │   │   │   ├── category.rs
    │   │   │   │   ├── company.rs
    │   │   │   │   ├── industry.rs
    │   │   │   │   ├── mod.rs
    │   │   │   │   ├── objects.rs
    │   │   │   │   ├── property.rs
    │   │   │   │   ├── relationship.rs
    │   │   │   │   └── view.rs
    │   │   │   ├── repository
    │   │   │   │   ├── mod.rs
    │   │   │   │   └── repo.rs
    │   │   │   ├── errors.rs
    │   │   │   └── lib.rs
    │   │   └── tests
    │   └── service
    │       └── src
    │           ├── lib.rs
    │           └── provider.rs
    ├── LICENSE
    ├── Makefile
    └── README.md

```

## Concepts

According to mongoDB official documentation, a graph database has four main concepts:

> - **Nodes (or vertices)**: You can think of nodes as the nouns in your databases; they store information about people, places, and things.
> - **Edges (or relationships)**: You can think of edges as the verbs in your databases; they store information about the actions that are taken between the nodes.
> - **Properties**: A property is a key-value pair that stores information about a particular node or edge.
> - **Labels**: A label can optionally be used to tag a group of related nodes.

## References

- [mongodb graph database](https://www.mongodb.com/databases/mongodb-graph-database)
- [sample training database](https://docs.atlas.mongodb.com/sample-data/sample-training/#std-label-training-routes)
