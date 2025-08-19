{
  "@context": {
    "know": "https://know.dev/",
    "xsd": "http://www.w3.org/2001/XMLSchema#",
    "count": {
      "@id": "know:count",
      "@type": "xsd:integer"
    },
    "tags": {
      "@id": "know:tags",
      "@type": "know:Collection"
    },
    "id": {
      "@id": "know:id",
      "@type": "xsd:string"
    },
    "name": {
      "@id": "know:name",
      "@type": "xsd:string"
    },
    "updated": {
      "@id": "know:updated",
      "@type": "xsd:integer"
    }
  },
  "@id": "https://readwise.io/tags",
  "@type": ["know:TagsCollection", "know:Collection"],
  "tags": {
    "@type": "know:Collection",
    "count": length,
    "items": [
      .[] | {
        "@type": "know:Tag",
        "id": (.updated | tostring),
        "name": .name,
        "updated": .updated
      }
    ]
  }
}
