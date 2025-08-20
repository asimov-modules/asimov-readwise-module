{
  "@context": {
    "know": "https://know.dev/",
    "xsd": "http://www.w3.org/2001/XMLSchema#",
    "count": {
      "@id": "know:count",
      "@type": "xsd:integer"
    },
    "books": {
      "@id": "know:books",
      "@type": "know:Collection"
    },
    "id": {
      "@id": "know:id",
      "@type": "xsd:string"
    },
    "title": {
      "@id": "know:title",
      "@language": "en"
    },
    "author": {
      "@id": "know:author",
      "@type": "xsd:string"
    },
    "category": {
      "@id": "know:category",
      "@type": "xsd:string"
    },
    "num_highlights": {
      "@id": "know:numHighlights",
      "@type": "xsd:integer"
    },
    "last_highlight_at": {
      "@id": "know:lastHighlightAt",
      "@type": "xsd:dateTime"
    },
    "cover_image_url": {
      "@id": "know:coverImageUrl",
      "@type": "@id"
    },
    "source_url": {
      "@id": "know:sourceUrl",
      "@type": "@id"
    },
    "source_type": {
      "@id": "know:sourceType",
      "@type": "xsd:string"
    }
  },
  "@id": "https://readwise.io/books",
  "@type": ["know:BookCollection", "know:Collection"],
  "books": {
    "@type": "know:Collection",
    "count": ((.results // []) | length),
    "items": [
      (.results // [])[] | {
        "@type": "know:Book",
        "id": .id,
        "title": .title,
        "author": .author,
        "category": .category,
        "num_highlights": .num_highlights,
        "last_highlight_at": .last_highlight_at,
        "cover_image_url": .cover_image_url,
        "source_url": .source_url,
        "source_type": .source_type
      }
    ]
  }
}
