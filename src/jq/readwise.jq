{
  "@context": {
    "know": "https://know.dev/",
    "xsd": "http://www.w3.org/2001/XMLSchema#",
    "count": {
      "@id": "know:count",
      "@type": "xsd:integer"
    },
    "highlights": {
      "@id": "know:highlights",
      "@type": "know:Collection"
    },
    "id": {
      "@id": "know:id",
      "@type": "xsd:integer"
    },
    "text": {
      "@id": "know:text",
      "@language": "en"
    },
    "note": {
      "@id": "know:note",
      "@language": "en"
    },
    "location": {
      "@id": "know:location",
      "@type": "xsd:integer"
    },
    "location_type": {
      "@id": "know:locationType",
      "@type": "xsd:string"
    },
    "highlighted_at": {
      "@id": "know:highlightedAt",
      "@type": "xsd:dateTime"
    },
    "updated": {
      "@id": "know:updated",
      "@type": "xsd:dateTime"
    }
  },
  "@id": "https://readwise.io/highlights",
  "@type": ["know:HighlightsCollection", "know:Collection"],
  "highlights": {
    "@type": "know:Collection",
    "count": ((.results // []) | length),
    "items": [
      (.results // [])[] | {
        "@type": "know:Highlight",
        "id": .id,
        "text": .text,
        "note": (.note // ""),
        "location": .location,
        "location_type": .location_type,
        "highlighted_at": .highlighted_at,
        "updated": .updated
      }
    ]
  }
}