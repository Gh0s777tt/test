# Markdown to AsciiDoc Conversion Guide

## Overview
This guide explains how to convert existing Markdown documentation to AsciiDoc format for VantisOS.

## Why Convert?

AsciiDoc provides advantages over Markdown:

- Better support for technical documentation
- More powerful formatting options
- Better table support
- Built-in includes and attributes
- Professional PDF generation
- Industry standard for technical docs

## Conversion Process

### Step 1: Install Tools

Install Asciidoctor and conversion tools:

```bash
# Install Asciidoctor
gem install asciidoctor

# Install Asciidoctor PDF
gem install asciidoctor-pdf

# Install kramdown-asciidoc (for conversion)
gem install kramdown-asciidoc
```

### Step 2: Convert Files

Convert Markdown files to AsciiDoc:

```bash
# Convert single file
kramdoc README.md -o README.adoc

# Convert multiple files
for file in *.md; do
    kramdoc "$file" -o "${file%.md}.adoc"
done
```

## Conversion Reference

### Headings

**Markdown:**
```markdown
# Heading 1
## Heading 2
```

**AsciiDoc:**
```asciidoc
= Heading 1
== Heading 2
```

### Code Blocks

**Markdown:**
```markdown
\`\`\`rust
fn main() {
    println!("Hello");
}
\`\`\`
```

**AsciiDoc:**
```asciidoc
[source,rust]
----
fn main() {
    println!("Hello");
}
----
```

### Tables

**Markdown:**
```markdown
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
```

**AsciiDoc:**
```asciidoc
|===
| Header 1 | Header 2

| Cell 1   | Cell 2
|===
```

## Best Practices

1. Use semantic markup
2. Add document metadata
3. Define attributes for reuse
4. Validate syntax
5. Test output

## Resources

- [Asciidoctor Documentation](https://asciidoctor.org/docs/)
- [AsciiDoc Syntax](https://asciidoctor.org/docs/asciidoc-syntax-quick-reference/)

---

**Version:** 1.0  
**Created:** February 24, 2025
