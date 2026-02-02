# Overlay Research Agent

A guide for using Copilot as an agent to help research and enrich metadata type documentation.

## Overview

This agent helps contributors fill in overlay documentation for Salesforce metadata types by:
1. Researching relevant Salesforce documentation
2. Extracting key information about a metadata type
3. Formatting suggestions as overlay entries
4. Creating a PR-ready contribution

## Using the Agent

### In GitHub Copilot (with Credits)

**System Context Setup** (run once in your agent session):

```
You are a Salesforce metadata documentation researcher helping enrich type definitions.

Your role:
- Research Salesforce docs (developer.salesforce.com, help.salesforce.com) for metadata types
- Extract descriptions, field purposes, constraints, and best practices
- Format findings as structured overlay documentation
- Suggest improvements that will appear in generated schemas and IDE tooltips

Output format for overlays:
```toml
[[types]]
name = "MetadataTypeName"
description = "Brief purpose and use case"

[[types.fields]]
name = "fieldName"
description = "What this field does, when to use it, any constraints"
```

When suggesting content:
- Be concise but informative (1-3 sentences per field)
- Include constraints or gotchas
- Note if field is required/optional if not obvious
- Link to relevant Salesforce docs in comments
```

### Example: Research a Metadata Type

**Your prompt to the agent:**

```
Help me document the Flow metadata type for overlay enrichment.

Current overlay file location: crates/sf-typegen/overlays.toml

Research the Salesforce Flow documentation and help me:
1. Write a clear description of what Flow is and its role in Salesforce
2. Identify 3-5 key fields that need better documentation
3. For each field, provide a concise description that explains its purpose
4. Note any constraints or important behaviors

Format your response as TOML entries I can add to the overlays file.
```

**Agent workflow:**
1. Searches salesforce.com for Flow documentation
2. Synthesizes information about the Flow type and its major fields
3. Returns TOML blocks ready to paste into `overlays.toml`

Example output:

```toml
[[types]]
name = "Flow"
description = "Represents an automated business process. Flows automate business processes by defining a series of actions that execute based on user-specified conditions."

[[types.fields]]
name = "fullName"
description = "The API name of the Flow, used when referencing it in code or configuration"

[[types.fields]]
name = "isTemplate"
description = "When true, this Flow is a template that can be reused as a starting point for other flows"

[[types.fields]]
name = "processType"
description = "The type of flow: Standard (most common), AutoLaunched, Scheduled, or Platform Event-triggered"

[[types.fields]]
name = "status"
description = "Current status of the Flow: Draft, Active, or Deprecated. Only active flows can be executed"
```

## Tips for Best Results

1. **Be Specific**: Name the exact metadata type you want to research
2. **Check Existing Content**: Look at `crates/sf-typegen/overlays.toml` first—some types may already be started
3. **Iterate**: Ask follow-up questions like "Tell me more about field X" or "What are common Flow limitations?"
4. **Review Output**: Verify the agent's suggestions match what you see in the docs
5. **Test Locally**: After getting the TOML, add it to overlays.toml and run the build to see how it renders

## Workflow: From Research to PR

1. **Start the agent** with context about the metadata type
2. **Get TOML suggestions** and review them
3. **Add to overlays.toml** in your fork
4. **Run `cargo build`** in `crates/sf-types/` to generate and validate
5. **Check the output** in generated schemas
6. **Submit PR** with your enriched overlay entries

## Metadata Types Needing Documentation

See `missing_overlays.json` for a prioritized list of types that would benefit from overlay enrichment.

## Questions?

- If the agent can't find information, try a more specific prompt ("Document the Field mapping field") or check if it's a newer/enterprise-only feature
- Some types may have sparse docs—that's fine, contribute what exists and community members can iterate
- Open an issue if you find a type that needs special handling

## Resources

- Overlays documentation: See `crates/sf-typegen/README.md`
- Example overlays: `crates/sf-typegen/overlays.toml`
- Generated types location: `crates/sf-types/src/metadata/`
