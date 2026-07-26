# FAQ

## Is EZHTML a programming language?

No. EZHTML is a markup language. CSS and JavaScript work exactly as before.

## Does EZHTML replace HTML?

It **translates to** HTML. Every `.ezhtml` source is compiled down to
clean, valid HTML5. You can still open the output in any browser, host it
on any server, and inspect it with DevTools.

## Why not just use Markdown?

Markdown is great for prose, but it stops short of expressing layout
primitives like rows, columns and cards. EZHTML covers 28 HTML elements
that Markdown simply doesn't have, while keeping Markdown's
"easy to read, easy to write" feel.

## Can I use my existing CSS / component library?

Yes. The emitted HTML uses stable class names (`.btn`, `.btn-primary`,
`.card`, `.row`, `.col`, `.container`, `.navbar`, …) so anything you
write in your CSS file will keep working.

## What about JavaScript?

JS still ships as-is via the `html` pass-through keyword:

```ezhtml
html "<script src=\"/app.js\" defer></script>"
```

For more structured behaviour, full framework integrations
(React + Vite, SvelteKit, Astro) are planned in
[`docs/Roadmap.md`](Roadmap.md).

## Why no closing tags?

Because indentation is enough. The compiler emits valid HTML for you,
so you never have to remember `</div>` again.

## Why Rust?

Three reasons:

1. **One static binary.** Easy to install, easy to ship on CI.
2. **Predictable performance.** No GC pauses, no warm-up cost.
3. **Strong types.** The compiler pipeline is small enough to live in
   one crate without hiding intent behind abstractions.

## Does the editor require Electron?

The OSS reference editor uses Electron, but the underlying Rust
binary works on every platform that has a shell. A Tauri version is on
the roadmap.

## Will there be a hosted service?

Yes — see the [Roadmap](Roadmap.md#phase-5--ecosystem-next). You'll be
able to push a repo and get a generated URL back.

## How do I report a bug?

Open an issue using the [bug report template](../.github/ISSUE_TEMPLATE/bug_report.md)
and include:

- the `ezhtml version` output
- a minimal `.ezhtml` file that reproduces
- your OS and architecture

## How do I request a feature?

Use the [feature request template](../.github/ISSUE_TEMPLATE/feature_request.md).
Include a real-world use case so we can weigh impact.

## Why MIT?

Maximum reuse, maximum remix. We want every framework, IDE and CDN to
be able to integrate EZHTML without legal friction.

## Where is the playground?

[`/playground`](../../website/playground/index.html). It runs the WASM
build of the compiler directly in your browser.
