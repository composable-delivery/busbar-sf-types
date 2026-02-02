# Contributing

We welcome contributions! Here's how you can help:

## Getting Started

- **Questions or Ideas?** Start a [Discussion](../../discussions)
- **Found a Bug?** Open an [Issue](../../issues)
- **Have a Fix or Feature?** We'd love to see it—open an issue first to discuss, then submit a PR

## Where Help is Needed

### Categories and Overlays

The metadata types in this project benefit from rich documentation. We generate types from Salesforce's own metadata definitions, but we can enhance them significantly:

- **Categories**: Organize and classify metadata types in `crates/sf-typegen/`
- **Overlays**: Add additional metadata and documentation that layers on top of generated types

See `missing_categories.json` and `missing_overlays.json` for areas that need attention.

### Documentation Comments

One of the most impactful contributions is enriching the `///` doc comments on structs and their fields. These comments:

- Transfer directly through `schemars` to generated schemas
- Provide context and guidance for users of the generated types
- Help document Salesforce-specific behavior and gotchas

You can add overlay content that appears in these doc comments—pull in descriptions from Salesforce documentation, add usage examples, or clarify field purposes.

## Development

1. Clone the repository
2. Most changes involve modifying overlay files or adding documentation
3. For code changes, ensure existing tests pass

## Questions?

Open a [Discussion](../../discussions) if you're unsure about something or want to discuss an approach before diving in.
