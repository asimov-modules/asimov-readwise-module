{
  "@context": {
    "know": "https://know.dev/",
    "xsd": "http://www.w3.org/2001/XMLSchema#",
    "icon": {
      "@id": "know:icon",
      "@type": "@id",
    },
    "image": {
      "@id": "know:image",
      "@type": "@id",
    },
    "items": {
      "@id": "know:items",
      "@type": "know:ReadwiseHighlight",
      "@container": "@list",
    },
    "link": {
      "@id": "know:link",
      "@type": "@id",
    },
    "position": {
      "@id": "know:position",
      "@type": "xsd:integer",
    },
    "summary": {
      "@id": "know:summary",
      "@language": "en",
    },
    "title": {
      "@id": "know:title",
      "@language": "en",
    },
    "author": {
      "@id": "know:author",
      "@language": "en",
    },
    "category": {
      "@id": "know:category",
      "@language": "en",
    },
    "highlighted_at": {
      "@id": "know:highlighted_at",
      "@type": "xsd:dateTime",
    },
    "source_type": {
      "@id": "know:source_type",
      "@language": "en",
    },
    "note": {
      "@id": "know:note",
      "@language": "en",
    }
  },
  "@id": .source_url // "readwise:highlight",
  "@type": "know:ReadwiseHighlights",
  "title": (.title // "Readwise Highlights"),
  "summary": (.category // "Highlights from Readwise"),
  "image": .cover_image_url,
  "author": .author,
  "category": .category,
  "source_type": .source,
  "highlighted_at": .last_highlight_at,
  "items": [
    .highlights[] | {
      "@type": "know:ReadwiseHighlight",
      "position": .id,
      "title": (.title // .text[0:100] + "..."),
      "summary": .text,
      "link": .highlight_url // .source_url,
      "icon": .cover_image_url,
      "author": .author,
      "category": .category,
      "highlighted_at": .highlighted_at,
      "note": .note,
      "source_type": .source_type
    }
  ]
}